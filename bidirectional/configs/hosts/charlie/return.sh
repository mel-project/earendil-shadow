#!/bin/bash

# A received message, in the form 'b"{message}" from {sender}'
received = $(earendil control recv-message)

# Extract message
message=$(echo "$received" | cut -d'"' -f2)

# Extract sender
sender=$(echo "$received" | awk '{print $NF}')

earendil control send-message --destination $sender --message $message