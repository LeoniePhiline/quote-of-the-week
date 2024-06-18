#!/usr/bin/zsh

set -eux

docker run \
  --user "$(id -u):$(id -g)" \
  --volume "$PWD":/app \
  --workdir /app \
  ghcr.io/getzola/zola:v0.18.0@sha256:a514f95eb320062c4bb5a892d2ef8948bafa71279a45b9d7523d183abcdaa3dd \
  build
