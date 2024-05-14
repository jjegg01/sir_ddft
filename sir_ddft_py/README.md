Python Bindings to the SIR DDFT library
=======================================

This crate contains Python bindings to the SIR DDFT library.

Requirements
------------
- Rust >= 1.48 (e.g. via https://rustup.rs/)
- Python >= 3.7
- `numpy` >= 1.16.0 (e.g. via pip)
- Optional, but strongly recommended: `maturin` (e.g. via pip)
- Optional, only required for building the documentation: `pdoc3` (e.g. via pip)

Build instructions for building from source
-------------------------------------------
- Create a clean virtualenv and activate it
```shell
python3 -m venv sir_ddft_env
. sir_ddft_env/bin/activate
```
- Install dependencies
```shell
pip install numpy maturin
```
- Install into current virtualenv via Maturin
```shell
cd sir_ddft_py
maturin develop --release
```
- You can now import `sir_ddft` from any Python Skript running inside your virtualenv
(see the `examples` directory for some basic usage examples)
- Optional: Build the documentation after installing `pdoc3`
```
makeDoc.sh
```
