#!/usr/bin/env bash

if [ -z "$1" ]
  then
    echo "Image argument not supplied"
    exit 1
fi


RUST_BACKTRACE=1 cargo run --release --features=dump-json -- image $1 --type scheme-vibrant --import-json ./example/custom.json --import-json ./example/custom2.json --import-json-string '{ "text3": "Hello from args!"}' --verbose --config ./example/config.toml --continue-on-error