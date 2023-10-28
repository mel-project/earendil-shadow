#!/bin/bash

# run shadow test
rm -rf shadow.data && shadow shadow.yaml;

# Define the file and the string to be searched
FILE_PATH="./shadow.data/hosts/charlie/earendil.1004.stdout"
SEARCH_STRING="hellocharlie"

# Check the file for the search string
if grep -q "$SEARCH_STRING" "$FILE_PATH"; then
    echo "PASSED"
else
    echo "FAILED"
fi