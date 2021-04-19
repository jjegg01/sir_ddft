`sir_ddft` - A Rust implementation of the SIR-DDFT model
======================================================

This repository contains a numerical implementation of the SIR-DDFT model as described in the article

[te Vrugt, M., Bickmann, J. & Wittkowski, R.
Effects of social distancing and isolation on epidemic spreading modeled via dynamical density functional theory.
*Nat. Commun.* 11, 5576 (2020)](https://doi.org/10.1038/s41467-020-19024-0)

For comparison an implementation of the standard SIR model and a spatial SIR model with diffusion are included as well.

Bindings
--------
Bindings of this library to Python and JavaScript/WebAssembly exist in their respective crates `sir_ddft_py` and `sir_ddft_js` within this repository.

Examples and demo
-----------------
Examples of the usage of this library and its bindings can be found in the `examples` subfolder of the corresponding crate.

For ease of use we also include a web-based demonstration at [TODO](todo) which is suitable for small scale / low resolution simulations. For more advanced usage, we recommend using the Rust or Python API.

License
-------
All code in this repository is license under the GNU Affero General Public License Version 3 with the exception of the code in `sir_ddft_js/www/lib`, which is licensed under the MIT License. We gratefully acknowledge the copyright holders of these libraries:
- **Plotly.js:** Copyright (c) 2021 Plotly, Inc
- **Micromodal.js** Copyright (c) 2017 Indrashish Ghosh