# run shadow test
rm -rf shadow.data && shadow shadow.yaml;

# check shadow.data to see if bob received alice's message
#!/bin/bash

# Define the directory and the string to be searched
FILE_PATH="./shadow.data/hosts/charlie/earendil.1004.stdout"
SEARCH_STRING="hellocharlie"

# Check each file in the directory for the search string
if grep -q "$SEARCH_STRING" "$FILE_PATH"; then
    echo "PASSED"
else
    echo "FAILED"
fi