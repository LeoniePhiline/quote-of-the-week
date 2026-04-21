#!/usr/bin/zsh

set -eux

docker run \
  --tty \
  --interactive \
  --user "$(id -u)":"$(id -g)" \
  --volume "$PWD"/web:/app \
  --workdir /app \
  --publish 8080:8080 \
  ghcr.io/getzola/zola:v0.22.1@sha256:b491a8bec3773815bdcdbbfbc2d0d60eda880ef550c4cc512349ab6451fbd72a \
  "$@"
