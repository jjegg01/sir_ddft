#!/bin/bash

rm -rf doc
mkdir doc
cargo build --release
ln -s ../target/release/libsir_ddft.so sir_ddft.so
pdoc --html --output-dir doc sir_ddft
rm -rf sir_ddft.so