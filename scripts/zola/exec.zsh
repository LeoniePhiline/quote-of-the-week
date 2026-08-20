#!/usr/bin/zsh

set -eux

docker run \
  --tty \
  --interactive \
  --user "$(id -u)":"$(id -g)" \
  --volume "$PWD"/web:/app \
  --workdir /app \
  --publish 8080:8080 \
  ghcr.io/getzola/zola:v0.23.4@sha256:b1b5cbf8abe8c5da983372d1baaeb87a5fbe231fc8cf35dbefeb778d301ba237 \
  "$@"
