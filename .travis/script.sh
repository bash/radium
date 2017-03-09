#!/bin/sh

SRC_DIR="${TRAVIS_BUILD_DIR}/src"

for DIR in $(ls "${SRC_DIR}")
do
    cd "${SRC_DIR}/${DIR}"

    cargo fmt -- --write-mode=diff
    cargo build
    cargo test
done