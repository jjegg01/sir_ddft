//! State conversion from Rust to Python

use numpy::{PyArray1, PyArray, PyArray2, PyArray3};
use pyo3::Python;
use sir_ddft::{SIRState, SIRStateSpatial1D, SIRStateSpatial2D};

/// Trait for converting a time series of states to a set of multidimensional
/// Numpy arrays for time, S, I and R
pub(crate) trait GenericSIRState: Sized {
    type D;

    fn into_pyarrays<'py>(py: Python<'py>, data: &[(f64,Self)]) -> (&'py PyArray1<f64>, &'py PyArray<f64, Self::D>, &'py PyArray<f64, Self::D>, &'py PyArray<f64, Self::D>);
}

impl GenericSIRState for SIRState {
    type D = numpy::Ix1;

    #[allow(non_snake_case)]
    fn into_pyarrays<'py>(py: Python<'py>, data: &[(f64,Self)]) -> (&'py PyArray1<f64>, &'py PyArray<f64, Self::D>, &'py PyArray<f64, Self::D>, &'py PyArray<f64, Self::D>) {
        let t = PyArray1::<f64>::zeros(py, (data.len(),), false);
        let S = PyArray1::<f64>::zeros(py, (data.len(),), false);
        let I = PyArray1::<f64>::zeros(py, (data.len(),), false);
        let R = PyArray1::<f64>::zeros(py, (data.len(),), false);
        {
            macro_rules! pyarray_to_slice {
                ($name: ident) => {
                    let mut $name = $name.readwrite();
                    let $name = $name.as_slice_mut().unwrap();
                };
            }
            pyarray_to_slice!(t);
            pyarray_to_slice!(S);
            pyarray_to_slice!(I);
            pyarray_to_slice!(R);
            for (i,(ti, state)) in data.iter().enumerate() {
                t[i] = *ti;
                S[i] = state.S;
                I[i] = state.I;
                R[i] = state.R;
            }
        }
        (t,S,I,R)
    }
}

impl GenericSIRState for SIRStateSpatial1D {
    type D = numpy::Ix2;

    #[allow(non_snake_case)]
    fn into_pyarrays<'py>(py: Python<'py>, data: &[(f64,Self)]) -> (&'py PyArray1<f64>, &'py PyArray<f64, Self::D>, &'py PyArray<f64, Self::D>, &'py PyArray<f64, Self::D>) {
        if data.len() < 1 {
            panic!("Cannot convert state data to Python arrays since there is no state data!");
        }
        let nx = data[0].1.S.len();
        let t = PyArray1::<f64>::zeros(py, (data.len(),), false);
        let S = PyArray2::<f64>::zeros(py, (data.len(), nx), false);
        let I = PyArray2::<f64>::zeros(py, (data.len(), nx), false);
        let R = PyArray2::<f64>::zeros(py, (data.len(), nx), false);
        {
            macro_rules! pyarray_to_slice {
                ($name: ident) => {
                    let mut $name = $name.readwrite();
                    let $name = $name.as_slice_mut().unwrap();
                };
            }
            pyarray_to_slice!(t);
            pyarray_to_slice!(S);
            pyarray_to_slice!(I);
            pyarray_to_slice!(R);
            for (i,(ti, state)) in data.iter().enumerate() {
                t[i] = *ti;
                S[i*nx..((i+1)*nx)].copy_from_slice(&state.S);
                I[i*nx..((i+1)*nx)].copy_from_slice(&state.I);
                R[i*nx..((i+1)*nx)].copy_from_slice(&state.R);
            }
        }
        (t,S,I,R)
    }
}

impl GenericSIRState for SIRStateSpatial2D {
    type D = numpy::Ix3;

    #[allow(non_snake_case)]
    fn into_pyarrays<'py>(py: Python<'py>, data: &[(f64,Self)]) -> (&'py PyArray1<f64>, &'py PyArray<f64, Self::D>, &'py PyArray<f64, Self::D>, &'py PyArray<f64, Self::D>) {
        if data.len() < 1 {
            panic!("Cannot convert state data to Python arrays since there is no state data!");
        }
        let nx = (data[0].1.S.len() as f64).sqrt() as usize;
        let t = PyArray1::<f64>::zeros(py, (data.len(),), false);
        let S = PyArray3::<f64>::zeros(py, (data.len(), nx, nx), false);
        let I = PyArray3::<f64>::zeros(py, (data.len(), nx, nx), false);
        let R = PyArray3::<f64>::zeros(py, (data.len(), nx, nx), false);
        {
            macro_rules! pyarray_to_slice {
                ($name: ident) => {
                    let mut $name = $name.readwrite();
                    let $name = $name.as_slice_mut().unwrap();
                };
            }
            pyarray_to_slice!(t);
            pyarray_to_slice!(S);
            pyarray_to_slice!(I);
            pyarray_to_slice!(R);
            for (i,(ti, state)) in data.iter().enumerate() {
                t[i] = *ti;
                S[i*nx*nx..((i+1)*nx*nx)].copy_from_slice(&state.S);
                I[i*nx*nx..((i+1)*nx*nx)].copy_from_slice(&state.I);
                R[i*nx*nx..((i+1)*nx*nx)].copy_from_slice(&state.R);
            }
        }
        (t,S,I,R)
    }
}