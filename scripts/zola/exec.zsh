#!/usr/bin/zsh

set -eux

docker run \
  --tty \
  --interactive \
  --user "$(id -u)":"$(id -g)" \
  --volume "$PWD"/web:/app \
  --workdir /app \
  --publish 8080:8080 \
  ghcr.io/getzola/zola:v0.23.6@sha256:722c7af3c6a7c0ccc37f01d85d47ee3e8b53ea6399f2b7dd7f9b82af7624fd61 \
  "$@"
