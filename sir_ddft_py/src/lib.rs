// sir_ddft - A Rust implementation of the SIR-DDFT model
// Copyright (C) 2021 Julian Jeggle, Raphael Wittkowski

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Python bindings for the SIR DDFT library
//!
//! Please see the documentation of the `sir_ddft` crate and the pydoc generated
//! documentation for more details on the API.

use pyo3::prelude::*;

mod parameters;
mod state;
mod solver_lumped;
mod solver_1d;
mod solver_2d;

pub use parameters::*;
pub use state::*;
pub use solver_lumped::*;
pub use solver_1d::*;
pub use solver_2d::*;

#[pymodule]
/// Python bindings for the SIR DDFT library.
///
/// For a detailed documentation please consult the documentation of the Rust
/// implementation. This document is only meant to document the nature of the
/// Python bindings.
fn sir_ddft(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SIRParameters>()?;
    m.add_class::<SIRDiffusionParameters>()?;
    m.add_class::<SIRDDFTParameters>()?;
    m.add_class::<SZParameters>()?;
    m.add_class::<SZDiffusionParameters>()?;
    m.add_class::<SZDDFTParameters>()?;
    m.add_class::<SIRState>()?;
    m.add_class::<Grid1D>()?;
    m.add_class::<Grid2D>()?;
    m.add_class::<SIRStateSpatial1D>()?;
    m.add_class::<SIRStateSpatial2D>()?;
    m.add_class::<SZStateSpatial2D>()?;
    m.add_class::<SIRSolver>()?;
    m.add_class::<SIRDiffusion1DSolver>()?;
    m.add_class::<SIRDDFT1DSolver>()?;
    m.add_class::<SIRDiffusion2DSolver>()?;
    m.add_class::<SIRDDFT2DSolver>()?;
    m.add_class::<SZDDFT2DSolver>()?;
    Ok(())
}

