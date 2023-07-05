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

//! Solver for the SIR-DDFT model under periodic boundary conditions in
//! two spatial dimensions

use std::sync::Arc;

use rustfft::{FftPlanner, Fft, FftDirection};
use num_complex::Complex64;
use itertools::izip;

use crate::{
    SIRStateSpatial2D, SIRParameters, SIRDiffusionParameters, SIRDDFTParameters,
    Grid1D, Grid2D,
    helpers::*,
    ode::{ODEIVP, StopCondition},
    sir::{SIRStateSpatial2DBorrowed}
};

/// Initial value problem for the SIR-DDFT model in two spatial dimensions
///
/// Note: The model is technically a PDE, but is transformed to a high-dimensional
/// ODE via the finite difference method.
pub struct SIRDDFT2DIVP {
    /// Flattened SIRStateSpatial1D (during integration ownership is passed to the
    /// integrator so state is None then!)
    state: Option<Vec<f64>>,
    /// x distance between grid points 
    /// (currently only equidistant grids are supported!)
    dx: f64,
    /// y distance between grid points 
    /// (currently only equidistant grids are supported!)
    dy: f64,
    /// Number of grid points in x
    nx: usize,
    /// Number of grid points in y
    ny: usize,
    /// Originally passed grid
    grid: Grid2D,
    /// Model parameters for all SIR models
    sir_params: SIRParameters,
    /// Model parameters for diffusion
    diff_params: SIRDiffusionParameters,
    /// Model parameters specific to the SIR DDFT model
    ddft_params: SIRDDFTParameters,
    /// Current time of integration
    time: f64,
    /// Total duration of integration
    duration: f64,
    /// Precalculated social distancing kernel
    kernel_sd_fft: Vec<Complex64>,
    /// Precalculated self isolation kernel
    kernel_si_fft: Vec<Complex64>,
    /// Fourier transform
    fft: Arc<dyn Fft<f64>>,
    /// Inverse fourier transform
    ifft: Arc<dyn Fft<f64>>,
    /// Scratch space for 2D FFT
    scratch: Vec<Complex64>,
    /// Convolution buffer for S and R convolutions
    conv_sr: Vec<Complex64>,
    /// Convolution buffer for S and R convolutions
    conv_i: Vec<Complex64>,
    /// Thread pool for parallel convolution
    #[cfg(not(target_arch = "wasm32"))]
    thread_pool: scoped_threadpool::Pool
}

impl<S> ODEIVP<S,f64> for SIRDDFT2DIVP {
    #[allow(non_snake_case)]
    fn rhs(&mut self, _ : f64, y: &[f64], rhs: &mut[f64]) {
        // Number of gridpoints
        let n = y.len() / 3;
        // Split state vector into S,I,R
        let (S,IR) = y.split_at(n);
        let (I,R) = IR.split_at(n);
        // Split RHS vector
        let (dS,dIR) = rhs.split_at_mut(n);
        let (dI,dR) = dIR.split_at_mut(n);
        // Shorthands for parameters
        let inf_param = self.sir_params.infection_parameter;
        let rec_rate = self.sir_params.recovery_rate;
        let mort_rate = self.sir_params.mortality_rate;
        let diff_S = self.diff_params.diffusivity_S;
        let diff_I = self.diff_params.diffusivity_I;
        let diff_R = self.diff_params.diffusivity_R;
        let mob_S = self.ddft_params.mobility_S;
        let mob_I = self.ddft_params.mobility_I;
        let mob_R = self.ddft_params.mobility_R;
        let dx = self.dx;
        let dy = self.dy;
        let nx = self.nx;
        let ny = self.ny;
        let kernel_sd_fft = self.kernel_sd_fft.as_slice();
        let kernel_si_fft = self.kernel_si_fft.as_slice();
        let fft = self.fft.clone();
        let ifft = self.ifft.clone();
        let scratch = self.scratch.as_mut_slice();
        // Platform dependant parts:
        #[allow(unused_variables)]
        let (thread_pool, num_threads);
        #[cfg(not(target_arch = "wasm32"))] 
        {
            thread_pool = &mut self.thread_pool;
            num_threads = thread_pool.thread_count();
        }
        // Stubs for WASM
        #[cfg(target_arch = "wasm32")]
        #[allow(unused_assignments)]
        {
            thread_pool = ();
            num_threads = 1;
        }
        
        // -- Calculate RHS --

        // Convolutions for S and R:
        let conv_sr = self.conv_sr.as_mut_slice();
        let tmp = self.conv_i.as_mut_slice(); // "Borrow" conv_i as temp storage

        for (conv_sr,tmp,S,I,R) in izip!(conv_sr.iter_mut(), tmp.iter_mut(), S,I,R) {
            *conv_sr = Complex64::new(S+R, 0.0);
            *tmp = Complex64::new(*I, 0.0);
        }
        convolve_2d_parallel(conv_sr, kernel_sd_fft, fft.clone(), ifft.clone(), scratch, thread_pool);
        convolve_2d_parallel(tmp, kernel_si_fft, fft.clone(), ifft.clone(), scratch, thread_pool);
        for (conv_sr,tmp) in conv_sr.iter_mut().zip(tmp.iter()) {
            *conv_sr += tmp;
        }
        // Convolution for I:
        let conv_i = self.conv_i.as_mut_slice();
        for (conv_i, S,I,R) in izip!(conv_i.iter_mut(), S,I,R) {
            *conv_i = Complex64::new(S + I + R, 0.0);
        }
        convolve_2d_parallel(conv_i, kernel_si_fft, fft.clone(), ifft.clone(), scratch, thread_pool);

        // Calculate RHS fields
        macro_rules! idx2 {
            ($ix: expr, $iy: expr) => {
                ($ix) + ($iy)*nx
            }
        }
        
        // Calculate contributions for a single row
        let add_contrib = |iy: usize, offset: usize, dS: &mut[f64], dI: &mut[f64], dR: &mut[f64]| {
            let [prevprev_y,prev_y,next_y,nextnext_y] = calc_indices(iy as i32, ny as i32);
            for ix in 0..nx {
                let [prevprev_x,prev_x,next_x,nextnext_x] = calc_indices(ix as i32, nx as i32);
                let curr = idx2!(ix,iy);
                // Discretized form of model equations
                macro_rules! ddft_term {
                    ($field:expr, $conv:expr) => {
                        (
                            grad_1d_val(
                                $field[idx2!(next_x,iy)] * grad_1d_val(
                                    $conv[idx2!(nextnext_x, iy)].re, 
                                    $conv[curr].re, dx),
                                $field[idx2!(prev_x,iy)] * grad_1d_val(
                                    $conv[curr].re,
                                    $conv[idx2!(prevprev_x, iy)].re, dx), dx) + 
                            grad_1d_val(
                                $field[idx2!(ix,next_y)] * grad_1d_val(
                                    $conv[idx2!(ix,nextnext_y)].re,
                                    $conv[curr].re, dy), 
                                $field[idx2!(ix,prev_y)] * grad_1d_val(
                                    $conv[curr].re,
                                    $conv[idx2!(ix,prevprev_y)].re, dy), dy)
                        )
                    }
                }
                dS[curr-offset] = diff_S * laplace_2d9(S, 
                    prev_x, ix, next_x, prev_y, iy, next_y, nx, dx, dy)
                    - inf_param * S[curr] * I[curr]
                    - mob_S * ddft_term!(S,conv_sr);
                dI[curr-offset] = diff_I * laplace_2d9(I, 
                    prev_x, ix, next_x, prev_y, iy, next_y, nx, dx, dy)
                    + inf_param * S[curr] * I[curr] - rec_rate * I[curr] - mort_rate * I[curr]
                    - mob_I * ddft_term!(I,conv_i);
                dR[curr-offset] = diff_R * laplace_2d9(R, 
                    prev_x, ix, next_x, prev_y, iy, next_y, nx, dx, dy)
                    + rec_rate * I[curr]
                    - mob_R * ddft_term!(R,conv_sr);
            }
        };

        // Single threaded?
        #[cfg(not(target_arch = "wasm32"))]
        if num_threads < 2 {
            for iy in 0..ny {
                add_contrib(iy,0,dS,dI,dR);
            }
        }
        // Else multi-threaded
        else {
            thread_pool.scoped(|s|{
                // Size of chunk in numbers of rows
                let chunk_size_y = ceil_div(ny, num_threads as usize);
                // Size of chunk in numbers of gridpoints
                let chunk_size = chunk_size_y * nx;
                // Split output slice into chunks
                let dS_chunks = dS.chunks_mut(chunk_size);
                let dI_chunks = dI.chunks_mut(chunk_size);
                let dR_chunks = dR.chunks_mut(chunk_size);
                // One thread per chunk will calculate all RHS values in said chunk
                for (i,dS,dI,dR) in izip![0..num_threads as usize, dS_chunks, dI_chunks, dR_chunks] {
                    s.execute(move || {
                        for iy in (i*chunk_size_y)..((i+1)*chunk_size_y).min(ny) {
                            add_contrib(iy, i*chunk_size, dS, dI, dR);
                        }
                    });
                }
            });
        }
        #[cfg(target_arch = "wasm32")] {
            for iy in 0..ny {
                add_contrib(iy,0,dS,dI,dR);
            }
        }
    }

    fn initial_state(&mut self) -> (f64, Vec<f64>) {
        (self.time, self.state.take().unwrap())
    }

    fn end_step(&mut self, _ : f64, _: &[f64], _: &S) -> StopCondition {
        StopCondition::ContinueUntil(self.duration)
    }

    fn final_state(&mut self, t: f64, y: Vec<f64>) {
        self.state = Some(y);
        self.time = t;
    }
}

impl SIRDDFT2DIVP {
    /// Creates a new IVP for the SIR DDFT model
    ///
    /// **Note that for now only square grid (i.e. `n x n` grid points) with equal lattice spacing
    /// in x and y are supported!**
    pub fn new(sir_params: SIRParameters, diff_params: SIRDiffusionParameters, 
        ddft_params: SIRDDFTParameters, state: SIRStateSpatial2D, num_threads: usize) 
    -> Self {
        // Validate grid  
        // TODO: proper errors
        let ((dx,nx,lx),(dy,ny,_ly)) = match &state.grid {
            Grid2D::Cartesian(cart_grid) => {
                (
                    match &cart_grid.grid_x {
                        Grid1D::Equidistant(grid) => { (grid.delta(), grid.n, grid.xlim.1 - grid.xlim.0) },
                        #[allow(unreachable_patterns)]
                        _ => { unimplemented!("Only equidistant grids in x are supported for now") }
                    },
                    match &cart_grid.grid_y {
                        Grid1D::Equidistant(grid) => { (grid.delta(), grid.n, grid.xlim.1 - grid.xlim.0) },
                        #[allow(unreachable_patterns)]
                        _ => { unimplemented!("Only equidistant grids in y are supported for now") }
                    },
                )
            },
            #[allow(unreachable_patterns)]
            _ => unimplemented!("Only cartesian grids are supported for now")
        };
        if nx < 3 || ny < 3 {
            panic!("Must have at least 3 grid points in every direction");
        }
        if nx != ny {
            panic!("Lattice must be square, i.e. have the same number of grid points in x and y");
        }
        if dx != dy {
            panic!("Lattice spacing must be equal in x and y");
        }
        if nx > i32::MAX as usize || ny > i32::MAX as usize {
            panic!("nx and ny must fit in a i32 variable");
        }
        // Threading is not available (yet) in WASM
        #[cfg(target_arch = "wasm32")]
        {
            if num_threads > 1 {
                panic!("Multithreading not supported in WASM");
            }
        }
        // Copy state into flattened state vector
        let state_vector = [state.S, state.I, state.R].concat();
        // Create Fourier transforms
        let mut fftplanner = FftPlanner::new();
        let fft = fftplanner.plan_fft(nx, FftDirection::Forward);
        let ifft = fftplanner.plan_fft(nx, FftDirection::Inverse);
        let scratch = vec![Complex64::new(0.0, 0.0); nx*ny];
        // Generate kernels
        let kernel_sd_fft = Self::generate_kernel_fft(ddft_params.social_distancing_range,
            ddft_params.social_distancing_amplitude, dx, nx, lx, fft.clone());
        let kernel_si_fft = Self::generate_kernel_fft(ddft_params.self_isolation_range,
            ddft_params.self_isolation_amplitude, dx, nx, lx, fft.clone());
        // Allocate convolution buffers
        let conv_sr = vec![Complex64::new(0.0,0.0); nx*ny];
        let conv_i = vec![Complex64::new(0.0,0.0); nx*ny];
        Self {
            state: Some(state_vector),
            dx,dy,
            nx,ny,
            grid: state.grid,
            sir_params,
            diff_params,
            ddft_params,
            time: 0.,
            duration: 0.,
            kernel_sd_fft,
            kernel_si_fft,
            fft, ifft,
            scratch,
            conv_i, conv_sr,
            #[cfg(not(target_arch = "wasm32"))]
            thread_pool: scoped_threadpool::Pool::new(num_threads as u32)
        }
    }

    fn generate_kernel_fft(range: f64, amp: f64, dx: f64, nx: usize, lx: f64, fft: Arc<dyn Fft<f64>>) -> Vec<Complex64> {
        // Assume square grid
        let ny = nx;
        let _dy = dx;
        // Create kernel
        let mut kernel = Vec::with_capacity(nx*ny);
        for iy in 0..ny {
            for ix in 0..nx {
                let dist_top_left = ix.pow(2) + iy.pow(2);
                let dist_top_right = (nx-ix).pow(2) + iy.pow(2);
                let dist_bottom_left = ix.pow(2) + (ny-iy).pow(2);
                let dist_bottom_right = (nx-ix).pow(2) + (ny-iy).pow(2);
                let dist = dist_top_left.min(dist_top_right.min(
                    dist_bottom_left.min(dist_bottom_right)));
                kernel.push(Complex64::new(amp*(-range * dist as f64 * dx*dx).exp(), 0.0));
            }
        }
        // Fourier transform kernel
        let mut scratch = vec![Complex64::new(0.0, 0.0); nx*ny];
        fft.process_with_scratch(kernel.as_mut_slice(), scratch.as_mut_slice());
        transpose_2d(kernel.as_mut_slice(), nx);
        fft.process_with_scratch(kernel.as_mut_slice(), scratch.as_mut_slice());
        // Normalize kernel FFT
        for x in &mut kernel {
            *x /= (nx as f64).powi(4) / (lx*lx); // Bake all normalization factors into kernel (why is this n**4 and not n**3?)
        }
        kernel
    }

    /// Increase integration time
    pub fn add_time(&mut self, time: f64) {
        assert!(time >= 0.);
        self.duration += time;
    }

    /// Get current time and state
    ///
    /// Note that the type of the return value is not SIRStateSpatial2D, but a
    /// similar construct with references
    #[allow(non_snake_case)]
    pub fn get_result(&self) -> (f64, SIRStateSpatial2DBorrowed) {
        let state = self.state.as_ref().unwrap();
        (self.time, SIRStateSpatial2DBorrowed::from_vec(state, &self.grid))
    }

    /// Raw read access to the state (used in profiling)
    pub fn clone_state(&self) -> Vec<f64> {
        self.state.as_ref().unwrap().clone()
    }

    /// Raw write access to the state (used in profiling)
    pub fn set_state(&mut self, state: &[f64]) {
        self.state.as_mut().unwrap().copy_from_slice(state);
    }
}