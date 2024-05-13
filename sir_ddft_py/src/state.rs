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
use pyo3::exceptions;
use pyo3::types::{PyFloat, PyType};
use numpy::IntoPyArray;

/* === Lumped === */

#[allow(non_snake_case)]
#[pyclass]
#[derive(Copy, Clone)]
/// State vector of the SIR model
pub struct SIRState {
    pub(crate) state: sir_ddft::SIRState
}

#[pymethods]
impl SIRState {
    #[allow(non_snake_case)]
    #[new]
    pub fn new(S: f64, I: f64, R: f64) -> Self {
        SIRState { state: sir_ddft::SIRState::new(S, I, R) }
    }

    #[allow(non_snake_case)]
    #[getter]
    /// Susceptible population
    pub fn get_S(&self) -> PyResult<f64> {
        Ok(self.state.S)
        
    }

    #[allow(non_snake_case)]
    #[setter]
    pub fn set_S(&mut self, S: f64) -> PyResult<()> {
        self.state.S = S;
        Ok(())
    }

    #[allow(non_snake_case)]
    #[getter]
    /// Infected population
    pub fn get_I(&self) -> PyResult<f64> {
        Ok(self.state.I)
        
    }

    #[allow(non_snake_case)]
    #[setter]
    pub fn set_I(&mut self, I: f64) -> PyResult<()> {
        self.state.I = I;
        Ok(())
    }

    #[allow(non_snake_case)]
    #[getter]
    /// Recovered population
    pub fn get_R(&self) -> PyResult<f64> {
        Ok(self.state.R)
        
    }

    #[allow(non_snake_case)]
    #[setter]
    pub fn set_R(&mut self, R: f64) -> PyResult<()> {
        self.state.R = R;
        Ok(())
    }
}

/* === 1D === */

#[pyclass]
#[derive(Clone)]
/// Grid in 1D
pub struct Grid1D {
    pub(crate) grid: sir_ddft::Grid1D
}

#[pymethods]
impl Grid1D {
    #[classmethod]
    /// Create an equidistant grid of n points in 1D ranging from xlo to xhi (inclusive)
    #[pyo3(text_signature = "(xlo, xhi, n)")]
    pub fn new_equidistant(_: &PyType, xlo: f64, xhi: f64, n: usize) -> PyResult<Self> {
        Ok(Self {
            grid: sir_ddft::Grid1D::new_equidistant((xlo, xhi), n)
        })
    }
}

#[pyclass]
/// State vector for spatial SIR models in 1D
///
/// To create the initial state, a Grid1D as well as an initializer function initfunc are required. The initializer function takes the grid position x as its sole argument and must return an array of three floats (!) representing the value of the S, I and R fields at this point respectively
#[pyo3(text_signature = "(grid, initfunc)")]
pub struct SIRStateSpatial1D {
    pub (crate) state: sir_ddft::SIRStateSpatial1D
}

#[pymethods]
impl SIRStateSpatial1D {
    #[allow(non_snake_case)]
    #[new]
    pub fn new(grid: &Grid1D, initfunc: &PyAny) -> PyResult<Self> {
        if !initfunc.is_callable() {
            return Err(PyErr::new::<exceptions::PyTypeError, _>("Initfunc is not callable"));
        }
        Ok(Self {
            state: sir_ddft::SIRStateSpatial1D::new(grid.grid.clone(), |x| {
                let ret = initfunc.call1((x,)).expect("Error calling initfunc in grid");
                let extract = |idx| {
                    ret.get_item(idx).expect("Missing value in initfunc return")
                    .downcast::<PyFloat>().expect("Invalid value in initfunc return").value() as f64
                };
                let S = extract(0);
                let I = extract(1);
                let R = extract(2);
                (S,I,R)
            })
        })
    }

    // Expensive getters (ownership is transferred to Python, so we must clone)

    #[allow(non_snake_case)]
    #[getter]
    /// State vector of susceptible population
    pub fn get_S<'py>(&self, py: Python<'py>) -> PyResult<&'py numpy::PyArray1<f64>> {
        Ok(self.state.S.clone().into_pyarray(py))
    }

    #[allow(non_snake_case)]
    #[getter]
    /// State vector of infected population
    pub fn get_I<'py>(&self, py: Python<'py>) -> PyResult<&'py numpy::PyArray1<f64>> {
        Ok(self.state.I.clone().into_pyarray(py))
    }

    #[allow(non_snake_case)]
    #[getter]
    /// State vector of recovered population
    pub fn get_R<'py>(&self, py: Python<'py>) -> PyResult<&'py numpy::PyArray1<f64>> {
        Ok(self.state.R.clone().into_pyarray(py))
    }
}

/* === 2D === */

#[pyclass]
#[derive(Clone)]
/// Grid in 2D
pub struct Grid2D {
    pub(crate) grid: sir_ddft::Grid2D
}

#[pymethods]
impl Grid2D {
    #[classmethod]
    /// Create an equidistant grid of nx and ny points in 2D ranging from xlo to xhi (inclusive) and ylo to yhi (inclusive) in x and y respectively 
    #[pyo3(text_signature = "(xlo, xhi, ylo, yhi, nx, ny)")]
    pub fn new_equidistant(_: &PyType, xlo: f64, xhi: f64, ylo: f64, yhi: f64, nx: usize, ny: usize) -> PyResult<Self> {
        Ok(Self {
            grid: sir_ddft::Grid2D::new_cartesian(
                sir_ddft::Grid1D::new_equidistant((xlo, xhi), nx), 
                sir_ddft::Grid1D::new_equidistant((ylo, yhi), ny))
        })
    }
}

#[pyclass]
/// State vector for spatial SIR models in 2D
///
/// To create the initial state, a Grid2D as well as an initializer function initfunc are required. The initializer function takes the grid position x and y as its arguments and must return an array of three floats (!) representing the value of the S, I and R fields at this point respectively
#[pyo3(text_signature = "(grid, initfunc)")]
pub struct SIRStateSpatial2D {
    pub (crate) state: sir_ddft::SIRStateSpatial2D
}

#[pymethods]
impl SIRStateSpatial2D {
    #[allow(non_snake_case)]
    #[new]
    pub fn new(grid: &Grid2D, initfunc: &PyAny) -> PyResult<Self> {
        if !initfunc.is_callable() {
            return Err(PyErr::new::<exceptions::PyTypeError, _>("Initfunc is not callable"));
        }
        Ok(Self {
            state: sir_ddft::SIRStateSpatial2D::new(grid.grid.clone(), |x,y| {
                let ret = initfunc.call1((x,y)).expect("Error calling initfunc in grid");
                let extract = |idx| {
                    ret.get_item(idx)
                        .expect("Missing value in initfunc return")
                        .downcast::<PyFloat>()
                        .expect("Invalid value in initfunc return").value() as f64
                };
                let S = extract(0);
                let I = extract(1);
                let R = extract(2);
                (S,I,R)
            })
        })
    }

    // Expensive getters (ownership is transferred to Python, so we must clone)

    #[allow(non_snake_case)]
    #[getter]
    /// State vector of susceptible population
    /// (note: this is 1D, so you might want to reshape this before plotting)
    pub fn get_S<'py>(&self, py: Python<'py>) -> PyResult<&'py numpy::PyArray1<f64>> {
        Ok(self.state.S.clone().into_pyarray(py))
    }

    #[allow(non_snake_case)]
    #[getter]
    /// State vector of infected population
    /// (note: this is 1D, so you might want to reshape this before plotting)
    pub fn get_I<'py>(&self, py: Python<'py>) -> PyResult<&'py numpy::PyArray1<f64>> {
        Ok(self.state.I.clone().into_pyarray(py))
    }

    #[allow(non_snake_case)]
    #[getter]
    /// State vector of recovered population
    /// (note: this is 1D, so you might want to reshape this before plotting)
    pub fn get_R<'py>(&self, py: Python<'py>) -> PyResult<&'py numpy::PyArray1<f64>> {
        Ok(self.state.R.clone().into_pyarray(py))
    }
}

#[pyclass]
/// State vector for spatial SZ models in 2D
///
/// To create the initial state, a Grid2D as well as an initializer function initfunc are required. The initializer function takes the grid position x and y as its arguments and must return an array of two floats (!) representing the value of the S and Z fields at this point respectively
#[pyo3(text_signature = "(grid, initfunc)")]
pub struct SZStateSpatial2D {
    pub (crate) state: sir_ddft::SZStateSpatial2D
}

#[pymethods]
impl SZStateSpatial2D {
    #[allow(non_snake_case)]
    #[new]
    pub fn new(grid: &Grid2D, initfunc: &PyAny) -> PyResult<Self> {
        if !initfunc.is_callable() {
            return Err(PyErr::new::<exceptions::PyTypeError, _>("Initfunc is not callable"));
        }
        Ok(Self {
            state: sir_ddft::SZStateSpatial2D::new(grid.grid.clone(), |x,y| {
                let ret = initfunc.call1((x,y)).expect("Error calling initfunc in grid");
                let extract = |idx| {
                    ret.get_item(idx)
                        .expect("Missing value in initfunc return")
                        .downcast::<PyFloat>()
                        .expect("Invalid value in initfunc return").value() as f64
                };
                let S = extract(0);
                let Z = extract(1);
                (S,Z)
            })
        })
    }

    // Expensive getters (ownership is transferred to Python, so we must clone)

    #[allow(non_snake_case)]
    #[getter]
    /// State vector of susceptible population
    /// (note: this is 1D, so you might want to reshape this before plotting)
    pub fn get_S<'py>(&self, py: Python<'py>) -> PyResult<&'py numpy::PyArray1<f64>> {
        Ok(self.state.S.clone().into_pyarray(py))
    }

    #[allow(non_snake_case)]
    #[getter]
    /// State vector of infected population
    /// (note: this is 1D, so you might want to reshape this before plotting)
    pub fn get_Z<'py>(&self, py: Python<'py>) -> PyResult<&'py numpy::PyArray1<f64>> {
        Ok(self.state.Z.clone().into_pyarray(py))
    }
}