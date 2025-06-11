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

# Get the pod name and logs
OPEN_POD=$(kubectl get pods -l role=open -o jsonpath="{.items[0].metadata.name}")
echo "Open pod: $OPEN_POD"

# Extract ticket from logs
echo "Waiting for ticket to be logged..."
TICKET=""
for _ in {1..10}; do
  TICKET=$(kubectl logs "$OPEN_POD" | grep "Ticket to join us:" | awk -F': ' '{print $2}')
  if [ -n "$TICKET" ]; then
    break
  fi
  sleep 3
done

if [ -z "$TICKET" ]; then
  echo "Failed to extract ticket. Aborting."
  exit 1
fi

echo "Ticket extracted: $TICKET"

# Deploy Join Nodes with ticket
helm upgrade --install blockchain-join ./deployment/join \
  --set db.username="$POSTGRES_USERNAME" \
  --set db.password="$POSTGRES_PASSWORD" \
  --set replicas=15 \
  --set ticket="$TICKET"