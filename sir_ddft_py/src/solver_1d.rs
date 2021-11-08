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
use pyo3::types::PyDict;
use numpy::{ToPyArray};

use crate::*;

use sir_ddft::{
    ode::{RKF45Solver, ExplicitODESolver}, 
    SIRDiffusion1DIVP, SIRDDFT1DIVP
};

fn export_result<'py>(time: f64, state: &sir_ddft::SIRStateSpatial1DBorrowed, 
    py: Python<'py>) -> PyResult<&'py PyDict> 
{
    let result = PyDict::new(py);
    result.set_item("time", time)?;
    result.set_item("S", state.S.to_pyarray(py))?;
    result.set_item("I", state.I.to_pyarray(py))?;
    result.set_item("R", state.R.to_pyarray(py))?;
    Ok(result)
}

/* === SIR with diffusion === */

#[pyclass]
#[pyo3(text_signature = "(sir_parameters, diffusion_parameters, state_1d)")]
/// Solver for the 1D SIR model with diffusion
pub struct SIRDiffusion1DSolver {
    solver: RKF45Solver<SIRDiffusion1DIVP>,
    ivp: SIRDiffusion1DIVP
}

#[pymethods]
impl SIRDiffusion1DSolver {
    #[new]
    pub fn new(params: &SIRParameters, diff_params: &SIRDiffusionParameters,
        state: &SIRStateSpatial1D) 
    -> Self {
        SIRDiffusion1DSolver {
            solver: RKF45Solver::<SIRDiffusion1DIVP>::new(),
            ivp: SIRDiffusion1DIVP::new(params.params.clone(),
                diff_params.diff_params.clone(), state.state.clone())
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
        export_result(time, &state, py)
    }
}

/* === SIR-DDFT === */

#[pyclass]
/// Solver for the 1D SIR DDFT model
#[pyo3(text_signature = "(sir_parameters, diffusion_parameters, ddft_parameters, state_1d, num_threads)")]
pub struct SIRDDFT1DSolver {
    solver: RKF45Solver<SIRDDFT1DIVP>,
    ivp: SIRDDFT1DIVP
}

#[pymethods]
impl SIRDDFT1DSolver {
    #[new]
    pub fn new(params: &SIRParameters, diff_params: &SIRDiffusionParameters,
        ddft_params: &SIRDDFTParameters, state: &SIRStateSpatial1D, num_threads: usize) 
    -> Self {
        SIRDDFT1DSolver {
            solver: RKF45Solver::<SIRDDFT1DIVP>::new(),
            ivp: SIRDDFT1DIVP::new(params.params.clone(), diff_params.diff_params.clone(), 
                ddft_params.ddft_params.clone(), state.state.clone(), num_threads)
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
        export_result(time, &state, py)
    }
}