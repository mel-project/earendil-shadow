#!/bin/bash

# Workaround to add users' local cargo binaries to path
for dir in /home/*/ ; do
    PATH=$PATH:"$dir".cargo/bin
done

# A received message, in the form 'b"{message}" from {sender}'
received=$(earendil control recv-message)

# Extract message
message=$(echo "$received" | cut -d'"' -f2)

# Extract sender
sender=$(echo "$received" | rev | cut -d' ' -f1 | rev)

earendil control send-message --destination $sender --message $message