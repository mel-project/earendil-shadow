use clap::Parser;
use rand::{seq::SliceRandom, Rng};

#[derive(Parser)]
struct Args {
    number: usize,
}

fn main() {
    let number = Args::parse().number;
    let mut nodes = Vec::new();
    let mut rng = rand::thread_rng();
    let mut ret = "adjacencies:\n".to_owned();

    for _ in 0..number {
        let pname = petname::petname(1, ":");
        nodes.push(pname);
    }

    for (i, node) in nodes.iter().enumerate() {
        let mut other_nodes = nodes.clone();
        other_nodes.remove(i);
        let neigh1 = other_nodes.choose(&mut rng).unwrap();
        let neigh2 = other_nodes.choose(&mut rng).unwrap();
        let neigh3 = other_nodes.choose(&mut rng).unwrap();

        ret += &format!("   - [{}, {}]\n", node, neigh1);
        ret += &format!("   - [{}, {}]\n", node, neigh2);
        ret += &format!("   - [{}, {}]\n", node, neigh3);
    }

    println!("{ret}")
}
