#!/bin/bash

wasm-pack build --target no-modules
rm -rf dist
cp -R www dist
rm -rf dist/serve.py
cp pkg/sir_ddft_js.js dist/
cp pkg/sir_ddft_js_bg.wasm dist/