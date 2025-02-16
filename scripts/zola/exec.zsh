#!/usr/bin/zsh

set -eux

docker run \
  --tty \
  --interactive \
  --user "$(id -u)":"$(id -g)" \
  --volume "$PWD"/web:/app \
  --workdir /app \
  --publish 8080:8080 \
  ghcr.io/getzola/zola:v0.20.0@sha256:c1c1be412af48740e37f25faf9b5cb7eeee5e52d9885f51a34479a385c9cc1e3 \
  "$@"
