#!/bin/sh

set -e

fold_start () {
  if [ ! -z "${TRAVIS}" ]; then
    echo -en "travis_fold:start:${1}"
  else
    tput setaf 8
    echo "[${1}]"
    tput sgr0
  fi
}

fold_end () {
  if [ ! -z "${TRAVIS}" ]; then
    echo -en "travis_fold:end:${1}"
  fi
}

SRC_DIR="`pwd`/src"

for DIR in $(ls "${SRC_DIR}")
do
    cd "${SRC_DIR}/${DIR}"

    FEATURES=""

    if [ "${DIR}" = "libradium" ]; then
        FEATURES="--all-features"
    fi

    fold_start "${DIR}.rustfmt"
    cargo fmt -- --write-mode=diff || true
    fold_end "${DIR}.rustfmt"

    fold_start "${DIR}.build"
    cargo build
    fold_end "${DIR}.build"

    if [ "${DIR}" = "libradium" ]; then
        fold_start "${DIR}.example"
        cargo build ${FEATURES} --example main
        fold_end "${DIR}.example"
    fi

    fold_start "${DIR}.test"
    cargo test ${FEATURES}
    fold_end "${DIR}.test"
done