[![crates.io](https://img.shields.io/crates/v/sir_ddft.svg)](https://crates.io/crates/sir_ddft)
[![Documentation](https://docs.rs/sir_ddft/badge.svg)](https://docs.rs/sir_ddft/)
[![Documentation for Python bindings](https://img.shields.io/badge/docs--py-2.0.0-yellow)](https://jjegg01.github.io/sir_ddft/pydoc/v2.0.0/)
[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.4702572.svg)](https://doi.org/10.5281/zenodo.4702572)
[![Funding provided by DFG WI 4170/3-1](https://img.shields.io/badge/DFG%20funded-WI%204170%2F3--1-blue)](https://www.dfg.de/foerderung/programme/einzelfoerderung/emmy_noether/index.html)
[![Funding provided by DFG Project ID 433682494 - SFB 1459](https://img.shields.io/badge/DFG%20funded-Project%20ID%20433682494%20--%20SFB%201459%20-blue)](https://gepris.dfg.de/gepris/projekt/433682494?context=projekt&task=showDetail&id=433682494&)

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

For ease of use we also include a [web-based demonstration](https://jjegg01.github.io/sir_ddft/demo/) which is suitable for small scale / low resolution simulations. For more advanced usage, we recommend using the [Rust](https://docs.rs/sir_ddft/) or [Python API](https://jjegg01.github.io/sir_ddft/pydoc/).

License
-------
All code in this repository is license under the GNU Affero General Public License Version 3 with the exception of the code in `sir_ddft_js/www/lib`, which is licensed under the MIT License. We gratefully acknowledge the copyright holders of these libraries:
- **Plotly.js:** Copyright (c) 2021 Plotly, Inc
- **Micromodal.js** Copyright (c) 2017 Indrashish Ghosh
