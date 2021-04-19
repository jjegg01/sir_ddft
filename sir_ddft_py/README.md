Python Bindings to the SIR DDFT library
=======================================

This crate contains Python bindings to the SIR DDFT library.

Requirements
------------
- Rust >= 1.41 (e.g. via https://rustup.rs/)
- Python >= 3.6
- `numpy` >= 1.16.0 (e.g. via pip)

Build instructions
------------------
First, build the native library with `cargo`:
```bash
cargo build --release
```

Then (depending on your platform) rename and copy the build artifact from `target/release/` to the desired location (e.g. your `PYTHONPATH` or just the same directory as your script using `sir_ddft`):
- Linux: Rename `libsir_ddft.so` to `sir_ddft.so`
- Windows: Rename `libsir_ddft.dll` to `sir_ddft.pyd`
- macOS: Rename `libsir_ddft.dylib` to `sir_ddft.so`