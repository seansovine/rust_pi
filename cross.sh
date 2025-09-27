#!/usr/bin/env bash

set -e

cargo build --release --target aarch64-unknown-linux-gnu

read -p "Do you want to transfer built executables to the Raspberry Pi 3B+? (yes/no) " yn

case "$yn" in
    [Yy]* ) echo "Proceeding...";;
    [Nn]* ) echo "Exiting..."; exit;;
    * ) echo "Invalid response. Please answer yes or no."; exit 1;;
esac

scp target/aarch64-unknown-linux-gnu/release/rust_pi \
    target/aarch64-unknown-linux-gnu/release/prime_benchmark \
    rpi3bp:/home/sean/code/rust_pi_demo/
