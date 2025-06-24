#!/bin/bash

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
    "python main.py $RUNS"
