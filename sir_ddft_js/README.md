Javascript/WebAssembly Bindings to the SIR DDFT library
=======================================================

This crate contains JavaScript/WebAssembly bindings to the SIR DDFT library.

Build instructions
------------------
`wasm-pack` can be used to generate the WebAssembly module and JavaScript glue:
```bash
wasm-pack build
```
The build artifacts are placed in the `pkg` directory.

To build the demo page, use the supplied script `makeDist.sh`. All files needed
are then placed in the `dist` directory.