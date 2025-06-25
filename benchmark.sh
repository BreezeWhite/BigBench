#!/bin/bash

# This is for bigdecimal crate
# See: https://github.com/akubera/bigdecimal-rs?tab=readme-ov-file#compile-time-configuration
export RUST_BIGDECIMAL_DEFAULT_PRECISION=1000 
cargo build -r

BIN_PATH=./target/release/pi_series
RUNS=1000

hyperfine --export-markdown result.md --shell=none \
    "$BIN_PATH $RUNS raw-bbp" \
    "$BIN_PATH $RUNS rs-decimal-bbp" \
    "$BIN_PATH $RUNS big-decimal-bbp" \
    "$BIN_PATH $RUNS rug-bbp" \
    "$BIN_PATH $RUNS dashu-bbp" \
    "$BIN_PATH $RUNS big-float-bbp" \
    "$BIN_PATH $RUNS astro-float-bbp" \
    "$BIN_PATH $RUNS fastnum-bbp" \
    "$BIN_PATH $RUNS decimal-rs-bbp" \
    "python main.py C $RUNS" \
    "python main.py PY $RUNS" \
