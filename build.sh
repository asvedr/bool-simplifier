#!/bin/bash
cd rust
cargo build --release
cp target/release/libsolver.so ../
cd ..
