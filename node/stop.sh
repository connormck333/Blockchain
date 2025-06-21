#!/bin/bash

set -e

for port in $(seq 8080 8094); do
  release="blockchain-join-$port"
  echo "Uninstalling $release..."
  helm uninstall "$release" || echo "$release not found"
done

kubectl delete deployment blockchain-open

kubectl wait --for=delete pod -l role=open --timeout=60s
kubectl wait --for=delete pod -l role=join --timeout=60s

kubectl delete deployment postgres