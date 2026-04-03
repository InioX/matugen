#!/usr/bin/env bash

if [ -z "$1" ]
  then
    echo "Image argument not supplied"
    exit 1
fi


RUST_BACKTRACE=1 cargo run --release --features="dump-json, jxl-image" -- \
  image $1 \
  --type scheme-cmf \
  --import-json ./example/custom.json \
  --import-json ./example/custom2.json \
  --import-json-string '{ "text3": "Hello from args!"}' \
  --verbose \
  --continue-on-error \
  --fallback-color "#0000ff" \
  --base16-backend "wal" \
  --opacity 0.5 \
  "${@:2}"
  # --config ./example/config.toml \
  # --show-colors \
  # --json hex \
  # --old-json-output \
  # --show-colors \ 