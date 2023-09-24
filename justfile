default:
    just --list --unsorted

html:
    #!/usr/bin/env bash
    set -o pipefail
    rm -rf ./docs &&
    cargo doc &&
    cp -r ./target/doc ./docs &&
    for docfile in $(cd ./src/docs && find . -type f); do
        cp "./src/docs/${docfile}" "./docs/${docfile}" || exit $?
    done
