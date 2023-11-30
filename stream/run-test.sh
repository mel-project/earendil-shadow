#!/bin/bash

# run shadow test
rm -rf shadow.data && shadow shadow.yaml;

# Define the file and the string to be searched
FILE_PATH="./shadow.data/hosts/bob/earendil.1010.stdout"
SEARCH_STRING="the future is now"

# Check the file for the search string
if grep -q "$SEARCH_STRING_1" "$FILE_PATH_1" && grep -q "$SEARCH_STRING_2" "$FILE_PATH_2"; then
    echo "PASSED"
else
    echo "FAILED"
fi