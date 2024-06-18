#!/usr/bin/zsh

set -eux

docker run \
  --user "$(id -u):$(id -g)" \
  --volume "$PWD":/app \
  --workdir /app \
  --publish "8080:8080" \
  ghcr.io/getzola/zola:v0.17.1@sha256:26fa853200306cfd39a93a8434e97f29f36ebddaeb698f832a436e08f8c615f0 \
  serve \
  --interface "0.0.0.0" \
  --port 8080 \
  --base-url localhost
