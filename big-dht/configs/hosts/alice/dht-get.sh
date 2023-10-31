#! /bin/bash

# Workaround to add users' local binaries to path
for dir in /home/*/ ; do
    PATH=$PATH:"$dir".local/bin:"$dir".cargo/bin
done

keys=("hello" "salutations" "greetings")
fingerprints=()

for yaml_file in ../**/*-cfg.yaml; do
    # Horrible hack for parsing fingerprint field with variable parent name from yaml without using python
    fingerprint=$(awk -v word="fingerprint:" '$0 ~ word {for(i=1; i<=NF; i++) if($i==word) {print $(i+1); exit}}' "$yaml_file")
    fingerprints+=($fingerprint)
done

for fingerprint in "${fingerprints[@]}"; do
    for key in "${keys[@]}"; do
        destination="${fingerprint//\"}"
        earendil control global-rpc --destination $destination --method dht-get "$key"
    done
done