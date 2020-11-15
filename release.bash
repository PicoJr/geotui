#!/bin/bash -v

RELEASE_DIR="release$(date +%F-%H-%M-%S)"

rustfmt --check src/**/*.rs &&
    docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder:nightly-2020-08-15 cargo build --release &&
    mkdir -p ${RELEASE_DIR} &&
    cp target/x86_64-unknown-linux-musl/release/geotui ${RELEASE_DIR}/geotui-x86_64-unknown-linux-musl

