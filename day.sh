#!/usr/bin/env bash


set -euo pipefail

DAY="${1}"

cargo new \
    --vcs none \
    --name "day_${DAY}" \
    --bin \
    "./days/day_${DAY}"
