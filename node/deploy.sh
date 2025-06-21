#!/bin/bash

# Load Environment Variables
set -a
source .env
set -e

# Deploy Database Node
helm upgrade --install blockchain-db ./deployment/database \
  --set db.username="$POSTGRES_USERNAME" \
  --set db.password="$POSTGRES_PASSWORD"

echo "Waiting for database pod to be ready..."
kubectl wait --for=condition=ready pod -l role=database --timeout=60s

# Deploy Open Node
helm upgrade --install blockchain-open ./deployment/open \
  --set db.username="$POSTGRES_USERNAME" \
  --set db.password="$POSTGRES_PASSWORD"

echo "Waiting for open node to be ready..."
kubectl wait --for=condition=ready pod -l role=open --timeout=60s

# Deploy Join Nodes with ticket
for i in $(seq 0 14); do
  port=$((8080 + i))
  helm upgrade --install blockchain-join-$port ./deployment/join \
    --set db.username="$POSTGRES_USERNAME" \
    --set db.password="$POSTGRES_PASSWORD" \
    --set host_url="0.0.0.0:$port"
done