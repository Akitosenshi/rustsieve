#!/bin/bash

shell="$(getent passwd | grep $USER)"
shell=${shell##*:}

$shell -c 'RUSTFLAGS="-C target-cpu=native" cargo build --release && time ./target/release/rustsieve'
