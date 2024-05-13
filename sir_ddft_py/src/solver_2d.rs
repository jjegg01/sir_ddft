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
use numpy::ToPyArray;

use crate::*;

use ::sir_ddft::{
    ode::{RKF45Solver, ExplicitODESolver}, 
    SIRDiffusion2DIVP, SIRDDFT2DIVP, SIRStateSpatial2DBorrowed, SZDDFT2DIVP, SZStateSpatial2DBorrowed
};

fn get_grid_dims(grid: &::sir_ddft::Grid2D) -> (usize, usize) {
    #[allow(unreachable_patterns)]
    match grid {
        ::sir_ddft::Grid2D::Cartesian(grid) => {(
            match &grid.grid_x {
                ::sir_ddft::Grid1D::Equidistant(grid) => grid.n,
                _ => panic!("Cannot process non-equidistant grids!")
            },
            match &grid.grid_y {
                ::sir_ddft::Grid1D::Equidistant(grid) => grid.n,
                _ => panic!("Cannot process non-equidistant grids!")
            }
        )},
        _ => panic!("Cannot process non-Cartesian grids!")
    }
}

macro_rules! export_result {
    ($py: expr, $time: expr, $state: expr, $state_type: ty, $($field:ident, $outputname:literal),*) => {
        (|time, state: $state_type, py| {
            let (nx,ny) = get_grid_dims(&state.grid);
            let result = PyDict::new(py);
            result.set_item("time", time)?;
            for (state, key) in [$(state.$field,)*].iter().zip(&[$($outputname,)*]) {
                let arr = state.to_pyarray(py);
                let arr = arr.reshape([nx,ny]).unwrap();
                result.set_item(key, arr)?;
            }
            Ok(result)
        })($time, $state, $py)
    };
}

macro_rules! export_result_sir {
    ($py: expr, $time: expr, $state: expr, $state_type: ty) => {
        export_result!($py, $time, $state, $state_type, S, "S", I, "I", R, "R")
    };
}

macro_rules! export_result_sz {
    ($py: expr, $time: expr, $state: expr, $state_type: ty) => {
        export_result!($py, $time, $state, $state_type, S, "S", Z, "Z")
    };
}

/* === SIR with diffusion === */

#[pyclass]
#[pyo3(text_signature = "(sir_parameters, diffusion_parameters, state_2d)")]
/// Solver for the 2D SIR model with diffusion
pub struct SIRDiffusion2DSolver {
    solver: RKF45Solver<SIRDiffusion2DIVP,f64>,
    ivp: SIRDiffusion2DIVP
}

#[pymethods]
impl SIRDiffusion2DSolver {
    #[new]
    pub fn new(params: &SIRParameters, diff_params: &SIRDiffusionParameters, state: &SIRStateSpatial2D) 
    -> Self {
        Self {
            solver: RKF45Solver::<SIRDiffusion2DIVP,_>::new(),
            ivp: SIRDiffusion2DIVP::new(params.inner.clone(), diff_params.inner.clone(),
            state.state.clone())
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
        export_result_sir!(py, time, &state, &SIRStateSpatial2DBorrowed)
    }
}

/* === SIR-DDFT === */

#[pyclass]
#[pyo3(text_signature = "(sir_parameters, diffusion_parameters, ddft_parameters, state_2d, num_threads)")]
/// Solver for the 2D SIR DDFT model
pub struct SIRDDFT2DSolver {
    solver: RKF45Solver<SIRDDFT2DIVP,f64>,
    ivp: SIRDDFT2DIVP
}

#[pymethods]
impl SIRDDFT2DSolver {
    #[new]
    pub fn new(params: &SIRParameters, diff_params: &SIRDiffusionParameters,
        ddft_params: &SIRDDFTParameters, state: &SIRStateSpatial2D, num_threads: usize) 
    -> Self {
        SIRDDFT2DSolver {
            solver: RKF45Solver::<SIRDDFT2DIVP,_>::new(),
            ivp: SIRDDFT2DIVP::new(params.inner.clone(), diff_params.inner.clone(), 
                ddft_params.inner.clone(), state.state.clone(), num_threads)
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
        export_result_sir!(py, time, &state, &SIRStateSpatial2DBorrowed)
    }
}

/* == SZ-DDFT == */

#[pyclass]
#[pyo3(text_signature = "(sir_parameters, diffusion_parameters, ddft_parameters, state_2d, num_threads)")]
/// Solver for the 2D SIR DDFT model
pub struct SZDDFT2DSolver {
    solver: RKF45Solver<SZDDFT2DIVP,f64>,
    ivp: SZDDFT2DIVP
}

#[pymethods]
impl SZDDFT2DSolver {
    #[new]
    pub fn new(params: &SZParameters, diff_params: &SZDiffusionParameters,
        ddft_params: &SZDDFTParameters, state: &SZStateSpatial2D, num_threads: usize) 
    -> Self {
        Self {
            solver: RKF45Solver::<SZDDFT2DIVP,_>::new(),
            ivp: SZDDFT2DIVP::new(params.inner.clone(), diff_params.inner.clone(), 
                ddft_params.inner.clone(), state.state.clone(), num_threads)
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
        export_result_sz!(py, time, &state, &SZStateSpatial2DBorrowed)
    }
}