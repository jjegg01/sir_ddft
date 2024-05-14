#!/bin/bash

rm -rf doc
mkdir doc
maturin develop --release
pdoc --html --output-dir doc sir_ddft