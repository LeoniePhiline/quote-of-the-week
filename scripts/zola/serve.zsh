#!/usr/bin/zsh

set -eux

docker run \
  --user "$(id -u):$(id -g)" \
  --volume "$PWD":/app \
  --workdir /app \
  --publish "8080:8080" \
  ghcr.io/getzola/zola:v0.17.2@sha256:2b902803cc5f64685f25861ac9aad6c0903a023357992eb727ec1c26e67463b3 \
  serve \
  --interface "0.0.0.0" \
  --port 8080 \
  --base-url localhost
