#!/bin/sh -x
CONTAINER_RUNTIME=podman

# Start nats as container
IMAGE=$($CONTAINER_RUNTIME run -d --rm -p4222:4222 nats --jetstream)

cargo build

# Setup test
nats kv add config

nats kv put config b.K V
nats kv put config a.K V

timeout 22s cargo run

echo "Killed rust program"
nats consumer ls -a KV_config

echo "Stopping container"
$CONTAINER_RUNTIME stop $IMAGE
