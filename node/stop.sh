#!/bin/bash

set -e

kubectl delete statefulset blockchain-join

kubectl delete deployment blockchain-open

kubectl wait --for=delete pod -l role=open --timeout=60s
kubectl wait --for=delete pod -l role=join --timeout=60s

kubectl delete deployment postgres