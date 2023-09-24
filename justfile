default:
    just --list --unsorted

html:
    #!/usr/bin/env bash
    rm -rf ./html &&
    cargo doc &&
    cp -r ./target/doc ./html
