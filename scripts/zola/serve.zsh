#!/usr/bin/zsh

set -eux

./scripts/zola/exec.zsh \
  serve \
  --interface 0.0.0.0 \
  --port 8080 \
  --base-url localhost
