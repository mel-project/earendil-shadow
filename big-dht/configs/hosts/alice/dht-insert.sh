#! /bin/bash

# Workaround to add users' local binaries to path
for dir in /home/*/ ; do
    PATH=$PATH:"$dir".local/bin:"$dir".cargo/bin
done

keys=("hello" "salutations" "greetings")
values=("world" "solar system" "galaxy")
fingerprints=()

for yaml_file in ../**/*-cfg.yaml; do
    # Horrible hack for parsing fingerprint field with variable parent name from yaml without using python
    fingerprint=$(awk -v word="fingerprint:" '$0 ~ word {for(i=1; i<=NF; i++) if($i==word) {print $(i+1); exit}}' "$yaml_file")
    fingerprints+=($fingerprint)
done

echo ${fingerprints[@]}

for i in {0..2}; do
    index=$(( RANDOM % ${#fingerprints[@]} ))
    destination="${fingerprints[$index]//\"}"
    earendil control global-rpc --destination $destination --method dht-insert "${keys[$i]}" "${values[$i]}"
done

echo "finished DHT insertions"