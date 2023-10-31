#! /bin/bash

keys=("hello" "salutations" "greetings")
values=("world" "solar system" "galaxy")
fingerprints=()

for yaml_file in ../**/*-cfg.yaml; do
    if [[ -f $yaml_file ]]; then
        fingerprints+=("$(yq '.. | .fingerprint? | select(type=="string")' "$yaml_file")")
    fi
done

echo "fingerprints: "
echo "${fingerprints[@]}"

for i in {0..2}; do
    index=$(( RANDOM % ${#fingerprints[@]} ))
    destination="${fingerprints[$index]//\"}"
    earendil control global-rpc --destination $destination --method dht-insert "${keys[$i]}" "${values[$i]}"
done