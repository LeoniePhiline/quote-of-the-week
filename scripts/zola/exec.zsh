#!/usr/bin/zsh

set -eux

docker run \
  --tty \
  --interactive \
  --user "$(id -u)":"$(id -g)" \
  --volume "$PWD"/web:/app \
  --workdir /app \
  --publish 8080:8080 \
  ghcr.io/getzola/zola:v0.19.1@sha256:6e4f78e3f1338b2f117e4a88106ddf01bc2727bf7dcd06a97a8ea36e0fe14edd \
  "$@"
