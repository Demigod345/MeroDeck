#!/bin/bash
set -e

# Function to handle errors
error_handler() {
    echo "Error occurred in script at line: $1"
    exit 1
}

trap 'error_handler $LINENO' ERR

# Load environment variables from the file
source node_vars.env

# Create a new node first
NEW_NODE_NAME="test2"
NEW_SERVER_PORT=4002
NEW_SWARM_PORT=5002

# First check and remove existing node directory if it exists
if [ -d "$HOME/.calimero/$NEW_NODE_NAME" ]; then
    echo "Removing existing node directory..."
    rm -rf "$HOME/.calimero/$NEW_NODE_NAME"
fi


echo "Initializing node2 ..."
merod --node-name $NEW_NODE_NAME init --server-port $NEW_SERVER_PORT --swarm-port $NEW_SWARM_PORT --protocol starknet >/dev/null 2>&1 &
sleep 2

echo "Starting node2 ..."
merod --node-name $NEW_NODE_NAME run >/dev/null 2>&1 &
sleep 5


# Installing application on second node
echo "Installing application on node2 ..."
APP_ID=$(meroctl --node-name $NODE_NAME app install --path /home/hawkeye/works/MeroDeck/logic/res/proxy_contract_demo.wasm | grep "id:" | awk '{print $2}')
sleep 3

echo "Generating new identity pair for node2 ..."
OUTPUT=$(meroctl --node-name $NEW_NODE_NAME identity generate)
NODE2_PUBLIC_KEY=$(echo "$OUTPUT" | grep "public_key:" | awk '{print $2}')
NODE2_PRIVATE_KEY=$(echo "$OUTPUT" | grep "private_key:" | awk '{print $2}')

echo "Joining node2 to the network ..."
OUTPUT=$(meroctl --node-name test1 --output-format json context invite $CONTEXT_ID $MEMBER_PUBLIC_KEY $NODE2_PUBLIC_KEY)
sleep 5
JOIN_PAYLOAD=$(echo "$OUTPUT" | jq -r '.data')

meroctl --node-name $NEW_NODE_NAME --output-format json context join $NODE2_PRIVATE_KEY $JOIN_PAYLOAD
sleep 5

echo "NODE2_PK=$NODE2_PUBLIC_KEY" >> node_vars.env
echo "NODE2_SK=$NODE2_PRIVATE_KEY" >> node_vars.env

echo "Node2 joined the network!"
echo "====================================="
echo "Node2 Public Key: $NODE2_PUBLIC_KEY"
echo "Node2 Private Key: $NODE2_PRIVATE_KEY"
echo "====================================="