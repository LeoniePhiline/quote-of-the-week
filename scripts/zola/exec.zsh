#!/usr/bin/zsh

set -eux

docker run \
  --tty \
  --interactive \
  --user "$(id -u)":"$(id -g)" \
  --volume "$PWD"/web:/app \
  --workdir /app \
  --publish 8080:8080 \
  ghcr.io/getzola/zola:v0.19.2@sha256:9ab1bad3a575087cc2fe43032fcd159f88dc408e66334fc6a39f289a21308e06 \
  "$@"
