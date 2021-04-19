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

//! JavaScript/WebAssembly bindings for the SIR DDFT library
//!
//! Please see the documentation of the `sir_ddft` crate for a detailed documentation
//! of the API.

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