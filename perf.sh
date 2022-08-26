#!/bin/bash

RUSTFLAGS="-C target-cpu=native" cargo build --release && perf record ./target/release/rustsieve > /dev/null
