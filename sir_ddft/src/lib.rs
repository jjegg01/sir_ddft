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

//! This crate contains solvers for the SIR model, the spatial SIR model extension
//! including diffusion and a [novel SIR model based on dynamical density
//! functional theory](https://doi.org/10.1038/s41467-020-19024-0).
//!
//! Usage
//! -----
//! Basic usage follows the same pattern for all models:
//! 
//! ```
//! // Setup parameters and initial state
//! let params = SIRParameters::new(0.5, 0.1);
//! let state = SIRState::new(0.998, 0.002, 0.);
//! // Create the IVP and solver
//! let mut ivp = SIRODEIVP::new(params, state);
//! let solver = RKF45Solver::<SIRODEIVP>::new();
//! // Integrate for some time
//! ivp.add_time(2.0);
//! solver.integrate(&mut ivp);
//! // Retrieve the result
//! let (t,state) = ivp.get_result();
//! ```

pub mod ode;
mod sir;
mod sir_solver;
mod sir_diffusion_solver_1d;
mod sir_diffusion_solver_2d;
mod sir_ddft_solver_1d;
mod sir_ddft_solver_2d;
mod helpers;

pub use sir::*;
pub use sir_solver::*;
pub use sir_diffusion_solver_1d::*;
pub use sir_diffusion_solver_2d::*;
pub use sir_ddft_solver_1d::*;
pub use sir_ddft_solver_2d::*;