#!/usr/bin/env bash

if [ -z "$1" ]
  then
    echo "Image argument not supplied"
    exit 1
fi

RUST_BACKTRACE=1 cargo run --release --all-features -- \
  image $1 \
    --filter-docs-html \
    --quiet \
    --dry-run \
    --source-color-index 0