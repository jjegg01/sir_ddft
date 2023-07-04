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

use pyo3::prelude::*;
use pyo3::types::{PyDict};

use crate::*;

use ::sir_ddft::{
    ode::{RKF45Solver, ExplicitODESolver}, 
    SIRODEIVP
};

#[pyclass]
/// Solver for the SIR model
#[pyo3(text_signature = "(params, state)")]
pub struct SIRSolver {
    solver: RKF45Solver<SIRODEIVP,f64>,
    ivp: ::sir_ddft::SIRODEIVP
}

#[pymethods]
impl SIRSolver {
    #[new]
    pub fn new(params: &SIRParameters, state: &SIRState) -> Self {
        SIRSolver {
            solver: RKF45Solver::<SIRODEIVP,_>::new(),
            ivp: SIRODEIVP::new(params.params.clone(), state.state)
        }
    }

    #[pyo3(text_signature = "(time)")]
    /// Add time to the total integration time
    pub fn add_time(&mut self, time: f64) {
        self.ivp.add_time(time);
    }    
    
    #[pyo3(text_signature = "()")]
    /// Integrate to the current integration time
    pub fn integrate(&mut self) {
        self.solver.integrate(&mut self.ivp);
    }

    #[pyo3(text_signature = "()")]
    /// Get result of integration
    pub fn get_result<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        let (time, state) = self.ivp.get_result();
        let result = PyDict::new(py);
        result.set_item("time", time)?;
        result.set_item("S", state.S)?;
        result.set_item("I", state.I)?;
        result.set_item("R", state.R)?;
        Ok(result)
    }
}