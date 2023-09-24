default:
    just --list --unsorted

docs:
    #!/usr/bin/env bash
    set -o pipefail
    rm -rf ./docs &&
    cargo doc --no-deps &&
    cp -r ./target/doc ./docs &&
    for docfile in $(cd ./src/docs && find . -type f); do
        cp "./src/docs/${docfile}" "./docs/${docfile}" || exit $?
    done
