#! /bin/bash

# Workaround to add users' local binaries to path
for dir in /home/*/ ; do
    PATH=$PATH:"$dir".local/bin:$PATH:"$dir".cargo/bin
done

keys=("hello" "salutations" "greetings")
fingerprints=()

for yaml_file in ../**/*-cfg.yaml; do
    if [[ -f $yaml_file ]]; then
        fingerprints+=("$(yq '.. | .fingerprint? | select(type=="string")' "$yaml_file")")
    fi
done

for fingerprint in "${fingerprints[@]}"; do
    for key in "${keys[@]}"; do
        destination="${fingerprint//\"}"
        earendil control global-rpc --destination $destination --method dht-get "$key"
    done
done