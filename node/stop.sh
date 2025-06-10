#!/bin/bash

set -e

kubectl delete deployment blockchain-join

kubectl delete deployment blockchain-open

kubectl delete deployment postgres