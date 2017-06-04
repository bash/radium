#!/bin/sh

set -e

fold_start () {
  if [ ! -z "${TRAVIS}" ]; then
    echo -en "travis_fold:start:${1}\\r"
  else
    tput setaf 4
    echo "=> ${1}"
    tput sgr0
  fi
}

fold_end () {
  if [ ! -z "${TRAVIS}" ]; then
    echo -en "travis_fold:end:${1}\\r"
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

    fold_start "Running rustfmt for ${DIR}"
    # cargo fmt -- --write-mode=diff || true
    fold_end "Running rustfmt for ${DIR}"

    fold_start "Building ${DIR}"
    cargo build
    fold_end "Building ${DIR}"

    if [ "${DIR}" = "libradium" ]; then
        fold_start "Building ${DIR} example"
        cargo build ${FEATURES} --example main
        fold_end "Building ${DIR} example"
    fi

    fold_start "Testing ${DIR}"
    cargo test ${FEATURES}
    fold_end "Testing ${DIR}"
done