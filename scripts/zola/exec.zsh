#!/usr/bin/zsh

set -eux

docker run \
  --tty \
  --interactive \
  --user "$(id -u)":"$(id -g)" \
  --volume "$PWD"/web:/app \
  --workdir /app \
  --publish 8080:8080 \
  ghcr.io/getzola/zola:v0.23.2@sha256:de5c3e154869aa4a1599835a315e0e9c859598106bb64a6676b30c89729d4f1c \
  "$@"
