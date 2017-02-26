#!/bin/sh

cargo build --verbose;
target/debug/dockup install tests/resources/dockup.yaml;
cargo test -p dockup_utils;
cargo test --verbose;
