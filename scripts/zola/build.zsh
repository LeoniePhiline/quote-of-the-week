#!/usr/bin/zsh

set -eux

docker run \
  --user "$(id -u):$(id -g)" \
  --volume "$PWD":/app \
  --workdir /app \
  ghcr.io/getzola/zola:v0.17.2 \
  build
