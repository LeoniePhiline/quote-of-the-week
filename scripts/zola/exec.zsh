#!/usr/bin/zsh

set -eux

docker run \
  --tty \
  --interactive \
  --user "$(id -u)":"$(id -g)" \
  --volume "$PWD"/web:/app \
  --workdir /app \
  --publish 8080:8080 \
  ghcr.io/getzola/zola:v0.20.0@sha256:362e902e9d1e49d3da6b5d80efa982e0696dbb4f211ef8180cfbf210b10356be \
  "$@"
