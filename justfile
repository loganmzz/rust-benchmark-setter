default:
    just --list --unsorted

html:
    #!/usr/bin/env bash
    rm -rf ./docs &&
    cargo doc &&
    cp -r ./target/doc ./docs
