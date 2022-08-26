#!/bin/bash

shell="$(getent passwd | grep $USER)"
shell=${shell##*:}

$shell -c 'cargo build --release && time ./target/release/rustsieve'
