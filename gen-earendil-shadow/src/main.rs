use anyhow::Context;
use clap::Parser;
use earendil_crypt::{Fingerprint, IdentitySecret};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sosistab2::ObfsUdpSecret;

use std::{
    collections::{HashMap, HashSet},
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

/// YAML-serializable adjacencies representation
#[derive(Serialize, Deserialize)]
struct Adjacencies {
    adjacencies: Vec<(String, String)>,
}

#[derive(Parser)]
struct Args {
    config: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let yaml_path = Args::parse().config;
    let adjacencies: Adjacencies =
        serde_yaml::from_slice(&std::fs::read(yaml_path).context("cannot read config file")?)?;
    let adjacencies = get_adjacencies(&adjacencies)?;
    eprintln!("{:?}", adjacencies);

    let mut ips: HashMap<String, Ipv4Addr> = HashMap::new();
    let mut secrets: HashMap<String, String> = HashMap::new();
    let mut identities: HashMap<String, IdentitySecret> = HashMap::new();
    let mut fingerprints: HashMap<String, Fingerprint> = HashMap::new();
    let mut cookies: HashMap<String, [u8; 32]> = HashMap::new();

    let min: u32 = Ipv4Addr::new(200, 64, 1, 1).into();
    let max: u32 = Ipv4Addr::new(200, 64, 255, 255).into();

    for (i, node_name) in adjacencies.keys().enumerate() {
        // ip
        let i = i as u32;
        if min + i < max {
            ips.insert(node_name.to_owned(), (min + i).into());
        } else {
            anyhow::bail!("too many nodes")
        }

        // secret
        let secret: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();
        secrets.insert(node_name.to_owned(), secret.clone());

        // identity
        let identity = IdentitySecret::generate();
        identities.insert(node_name.to_owned(), identity.clone());

        // fingerprint
        let fingerprint = identity.public().fingerprint();
        fingerprints.insert(node_name.to_owned(), fingerprint);

        // cookies
        let obfs_udp_secret =
            ObfsUdpSecret::from_bytes(*blake3::hash(secret.as_bytes()).as_bytes());
        let cookie = *obfs_udp_secret.to_public().as_bytes();
        cookies.insert(node_name.to_owned(), cookie);
    }
    // construct strings
    let mut earendil_configs: Vec<(String, String)> = Vec::new();
    let mut shadow_hosts: HashMap<String, serde_json::Value> = HashMap::new();
    for (node_name, neighbors) in adjacencies.iter() {
        // earendil config
        let out_routes: HashMap<String, serde_json::Value> = neighbors.iter().map(|neigh| {
            (neigh.to_owned(), json!({
                    "fingerprint": fingerprints.get(neigh).unwrap().to_string(),
                    "protocol": "obfsudp",
                    "connect": SocketAddr::new(IpAddr::V4(ips.get(neigh).unwrap().to_owned()), 19999).to_string(),
                    "cookie": hex::encode(cookies.get(neigh).unwrap()),
                
            }))
        }).collect();
        let earendil_json = json!({
            "identity": format!("./{}-identity.asc", node_name),
            "state_cache": format!("./{}-state-cache.db", node_name),
            "in_routes":
            {
                "main_udp":
                {
                    "protocol": "obfsudp",
                    "listen": "0.0.0.0:19999",
                    "secret": secrets.get(node_name).unwrap()
                }
            },
            "out_routes": out_routes
        });
        let fingerprint_comment = format!("# {} fingerprint: {}\n", node_name, fingerprints.get(node_name).unwrap().to_string());
        earendil_configs.push((node_name.to_owned(), fingerprint_comment + &serde_yaml::to_string(&earendil_json)?));

        // shadow host
        let shadow_json = json!({
            "ip_addr": ips.get(node_name).unwrap().to_string(),
            "network_node_id": 0,
            "processes": [
                {
                    "path": "earendil",
                    "args": format!("daemon --config {}-cfg.yaml", node_name),
                    "expected_final_state": "running",
                }
            ]
        });
        shadow_hosts.insert(node_name.to_owned(), shadow_json);
    }
    let  shadow_yaml = serde_yaml::to_string(&json!({
        "general": {
            "model_unblocked_syscall_latency": true,
            "template_directory": "./configs/",
            "stop_time": "300s"
        },
        "network": {
            "graph": {
                "type": "1_gbit_switch"
            }
        },
        "hosts": shadow_hosts
    }))?;

    // write everything to files, building a nice directory tree
    std::fs::create_dir_all("./earendil-sim/configs/hosts/")?;
    std::fs::write("./earendil-sim/shadow.yaml", shadow_yaml)?;
    for (name, earendil_cfg) in earendil_configs {
        std::fs::create_dir(format!("./earendil-sim/configs/hosts/{name}"))?;
        std::fs::write(format!("./earendil-sim/configs/hosts/{name}/{name}-cfg.yaml"), earendil_cfg)?;
        let identity = hex::encode(stdcode::serialize(identities.get(&name).unwrap())?);
        std::fs::write(format!("./earendil-sim/configs/hosts/{name}/{name}-identity.asc"), identity)?;
    }

    Ok(())
}

fn get_adjacencies(adjacencies: &Adjacencies) -> anyhow::Result<HashMap<String, HashSet<String>>> {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    for (n1, n2) in adjacencies.adjacencies.iter() {
        map.entry(n1.to_owned()).or_default().insert(n2.to_owned());
    }
    Ok(map)
}
