#!/bin/bash

echo "galois_16_clmul sigle mul"
time  RUSTFLAGS=-Awarnings cargo test galois_16_clmul::tests::lots_of_single_mul --release --features simd-accel -- --nocapture

echo "galois_16_clmul double mul"
time RUSTFLAGS=-Awarnings cargo test galois_16_clmul::tests::lots_of_mul --release --features simd-accel -- --nocapture

echo "galois_16"
time RUSTFLAGS=-Awarnings cargo test galois_16::tests::lots_of_mul --release --features simd-accel -- --nocapture

