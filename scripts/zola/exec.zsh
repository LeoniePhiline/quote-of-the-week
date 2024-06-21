#!/usr/bin/zsh

set -eux

docker run \
  --tty \
  --interactive \
  --user "$(id -u)":"$(id -g)" \
  --volume "$PWD"/web:/app \
  --workdir /app \
  --publish 8080:8080 \
  ghcr.io/getzola/zola:v0.19.0@sha256:7ca6dd96e428a09294d09c29dcf371e11cdc5260bb426f2b22d36fb40ce1b135 \
  "$@"
