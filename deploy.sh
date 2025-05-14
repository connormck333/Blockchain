#!/bin/bash

set -e

# Step 1: Deploy Open Node
helm upgrade --install blockchain-open ./deployment/open

echo "Waiting for open node to be ready..."
kubectl wait --for=condition=ready pod -l role=open --timeout=60s

# Step 2: Get the pod name and logs
OPEN_POD=$(kubectl get pods -l role=open -o jsonpath="{.items[0].metadata.name}")
echo "Open pod: $OPEN_POD"

# Step 3: Extract ticket from logs
echo "Waiting for ticket to be logged..."
TICKET=""
for i in {1..10}; do
  TICKET=$(kubectl logs $OPEN_POD | grep "Ticket to join us:" | awk -F': ' '{print $2}')
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

# Step 4: Deploy Join Nodes with ticket
helm upgrade --install blockchain-join ./deployment/join \
  --set replicas=4 \
  --set ticket="$TICKET"