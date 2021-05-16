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

//! Solver for the SIR model with diffusion (simple extension of SIR
//! model with spatial resolution) under periodic boundary conditions

#[cfg(not(target_arch = "wasm32"))]
use itertools::izip;

use crate::{
    SIRStateSpatial1D, SIRParameters, SIRDiffusionParameters, SIRDDFTParameters,
    Grid1D,
    helpers::*,
    ode::{ODEIVP, StopCondition},
    sir::{SIRStateSpatial1DBorrowed}
};

/// Initial value problem for the SIR-DDFT model in one spatial dimension
///
/// Note: The model is technically a PDE, but is transformed to a high-dimensional
/// ODE via the finite difference method.
pub struct SIRDDFT1DIVP {
    /// Flattened SIRStateSpatial1D (during integration ownership is passed to the
    /// integrator so state is None then!)
    state: Option<Vec<f64>>,
    /// Distance between gridpoints (currently only equidistant grids are supported!)
    dx: f64,
    /// Originally passed grid
    grid: Grid1D,
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
    kernel_sd: Vec<f64>,
    /// Precalculated self isolation kernel
    kernel_si: Vec<f64>,
    /// Thread pool for parallel convolution
    #[cfg(not(target_arch = "wasm32"))]
    thread_pool: scoped_threadpool::Pool
}



/// 1D convolution with the symmetric (!) kernel of the SIR DDFT model
/// Returns (in this order):
///   Convolution of sd kernel with S+R + convolution of si kernel with I
///   Convolution of si kernel with S+I+R
#[allow(non_snake_case)]
fn convolve_sirddft(S: &[f64], I: &[f64], R: &[f64], kernel_sd: &[f64], kernel_si: &[f64],
    amp_sd: f64, amp_si: f64, offset: usize, dx: f64) -> (f64,f64) 
{
    let n = S.len();
    let mut conv_sd_SR = 0.0;
    let mut conv_si_I = 0.0;
    let mut conv_si_SIR = 0.0;
    // Note: the auto vectorizer does not seem to see much vectorization opportunity here
    //       so this might call for some manual SIMD stuff
    // TODO: Fix this mess once inline closures become stable
    #[inline(always)]
    fn add_contrib(kidx: usize, idx:usize, kernel_sd: &[f64], kernel_si: &[f64], 
        conv_sd_SR: &mut f64, conv_si_I: &mut f64, conv_si_SIR: &mut f64,
        dx: f64, S: &[f64], I: &[f64], R: &[f64]) 
    {
        let kernel_sd_fac = kernel_sd[kidx];
        let kernel_si_fac = kernel_si[kidx];
        *conv_sd_SR += (S[idx] + R[idx]) * kernel_sd_fac * dx;
        *conv_si_I += I[idx] * kernel_si_fac * dx;
        *conv_si_SIR += (S[idx] + I[idx] + R[idx]) * kernel_si_fac * dx;
    }
    // Putting the add_contrib code directly into the macro is ~7% slower in the
    // benchmark (why?)
    macro_rules! add_contrib_macro {
        ($kidx: expr, $idx: expr) => {
            add_contrib($kidx, $idx, kernel_sd, kernel_si, &mut conv_sd_SR, &mut conv_si_I, &mut conv_si_SIR, dx, S, I, R);
        }
    }
    // -- end of mess --
    add_contrib_macro!(0,offset);
    for i in 1..(n/2) {
        let left = (offset + n - i) % n;
        let right = (offset + i) % n;
        add_contrib_macro!(i, left);
        add_contrib_macro!(i, right);
    }
    let i = n/2 + 1;
    let left = (offset + n - i) % n;
    let right = (offset + i) % n;
    add_contrib_macro!(i, left);
    if n % 2 == 1 {
        add_contrib_macro!(i, right);
    }
    (conv_sd_SR * amp_sd + conv_si_I * amp_si, conv_si_SIR * amp_si)

    // // Old version
    // let n = S.len();
    // let mut conv_sd_SR = 0.0;
    // let mut conv_si_I = 0.0;
    // let mut conv_si_SIR = 0.0;
    // for i in 0..(n/2)+1 {
    //     let left = (offset + n - i) % n;
    //     let right = (offset + i) % n;
    //     let kernel_sd_fac = kernel_sd[i];
    //     let kernel_si_fac = kernel_si[i];
    //     conv_sd_SR += (S[left] + R[left]) * kernel_sd_fac * dx;
    //     conv_si_I += I[left] * kernel_si_fac * dx;
    //     conv_si_SIR += (S[left] + I[left] + R[left]) * kernel_si_fac * dx;
    //     // Don't count the last field double if n is even
    //     if left != right {
    //         conv_si_I += I[right] * kernel_si_fac * dx;
    //         conv_sd_SR += (S[right] + R[right]) * kernel_sd_fac * dx;
    //         conv_si_SIR += (S[right] + I[right] + R[right]) * kernel_si_fac * dx;
    //     }
    // }
    // (conv_sd_SR * amp_sd + conv_si_I * amp_si, conv_si_SIR * amp_si)
}

impl<S> ODEIVP<S> for SIRDDFT1DIVP {
    #[allow(non_snake_case)]
    fn rhs(&mut self, _ : f64, y: &[f64]) -> Vec<f64> {
        // Number of gridpoints
        let n = y.len() / 3;
        // Split state vector into S,I,R
        let (S,IR) = y.split_at(n);
        let (I,R) = IR.split_at(n);
        // Allocate and split RHS vector
        let mut rhs = vec![0.;n*3];
        let (dS,dIR) = rhs.split_at_mut(n);
        let (dI,dR) = dIR.split_at_mut(n);
        // Shorthands for parameters
        let inf_param = self.sir_params.infection_parameter;
        let rec_rate = self.sir_params.recovery_rate;
        let diff_S = self.diff_params.diffusivity_S;
        let diff_I = self.diff_params.diffusivity_I;
        let diff_R = self.diff_params.diffusivity_R;
        let mob_S = self.ddft_params.mobility_S;
        let mob_I = self.ddft_params.mobility_I;
        let mob_R = self.ddft_params.mobility_R;
        let amp_sd = self.ddft_params.social_distancing_amplitude;
        let amp_si = self.ddft_params.self_isolation_amplitude;
        let dx = self.dx;
        let kernel_sd = self.kernel_sd.as_ref();
        let kernel_si = self.kernel_si.as_ref();
        
        // Direct convolution (less efficient than FFT, but can be easily run in parallel)
        let calc_rhs = |start: usize, end: usize, dS: &mut [f64], dI: &mut [f64], dR: &mut[f64]| {
            // We can save time by reusing evaluations of the convolution in the next step, so we
            // only need to calculate one convolution tuple per cell
            // First step calculations:
            let mut conv_prevprev = 
                convolve_sirddft(S, I, R, kernel_sd, kernel_si, amp_sd, amp_si, (start+n-2)%n, dx);
            let mut conv_prev = 
                convolve_sirddft(S, I, R, kernel_sd, kernel_si, amp_sd, amp_si, (start+n-1)%n, dx);
            let mut conv_curr = 
                convolve_sirddft(S, I, R, kernel_sd, kernel_si, amp_sd, amp_si, start, dx);
            let mut conv_next = 
                convolve_sirddft(S, I, R, kernel_sd, kernel_si, amp_sd, amp_si, (start+1)%n, dx);
            // Calculate RHS
            for i in start..end {
                let prev = i as i64 - 1;
                let mut next = i+1;
                // Periodic boundary conditions
                let prev = if prev < 0 { n-1 } else {
                    // Only need to check if prev is not < 0
                    if next >= n {
                        next = 0;
                    }
                    prev as usize 
                };
                // Calculate next convolution:
                let conv_nextnext = 
                    convolve_sirddft(S, I, R, kernel_sd, kernel_si, amp_sd, amp_si, (i+2) % n, dx);
                // Semantic naming:
                let (conv_SR_prevprev, conv_I_prevprev) = conv_prevprev;
                let (conv_SR_curr, conv_I_curr) = conv_curr;
                let (conv_SR_nextnext, conv_I_nextnext) = conv_nextnext;
                // Calculate convolution gradients
                let conv_grad_SR_prev = grad_1d_val(conv_SR_prevprev, conv_SR_curr, dx);
                let conv_grad_SR_next = grad_1d_val(conv_SR_curr, conv_SR_nextnext, dx);
                let conv_grad_I_prev = grad_1d_val(conv_I_prevprev, conv_I_curr, dx);
                let conv_grad_I_next = grad_1d_val(conv_I_curr, conv_I_nextnext, dx);

                // Discretized form of model equations
                dS[i-start] = diff_S * laplace_1d(S, prev, i, next, dx)
                    - inf_param * S[i] * I[i]
                    - mob_S * (grad_1d_val(S[prev] * conv_grad_SR_prev, S[next] * conv_grad_SR_next, dx));
                dI[i-start] = diff_I * laplace_1d(I, prev, i, next, dx)
                    + inf_param * S[i] * I[i] - rec_rate * I[i]
                    - mob_I * (grad_1d_val(I[prev] * conv_grad_I_prev, I[next] * conv_grad_I_next, dx));
                dR[i-start] = diff_R * laplace_1d(R, prev, i, next, dx)
                    + rec_rate * I[i]
                    - mob_R * (grad_1d_val(R[prev] * conv_grad_SR_prev, R[next] * conv_grad_SR_next, dx));

                // Shift all stored convolutions
                conv_prevprev = conv_prev; 
                conv_prev = conv_curr;
                conv_curr = conv_next;
                conv_next = conv_nextnext;
            }
        };
        #[cfg(not(target_arch = "wasm32"))]
        {
            let num_threads = self.thread_pool.thread_count() as usize;
            self.thread_pool.scoped(|s| {
                let chunk_size = ceil_div(n,num_threads);
                let dS_chunks = dS.chunks_mut(chunk_size);
                let dI_chunks = dI.chunks_mut(chunk_size);
                let dR_chunks = dR.chunks_mut(chunk_size);
                for (i,dS,dI,dR) in izip!(0..num_threads, dS_chunks, dI_chunks, dR_chunks) {
                    s.execute(move || {
                        calc_rhs(i*chunk_size, ((i+1)*chunk_size).min(n), dS, dI, dR);
                    });
                }
            });
        }
        #[cfg(target_arch = "wasm32")]
        {
            // TODO: Switch to FFT based convolution
            calc_rhs(0,n,dS,dI,dR);
        }
        rhs
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

impl SIRDDFT1DIVP {
    /// Creates a new IVP for the SIR diffusion model
    pub fn new(sir_params: SIRParameters, diff_params: SIRDiffusionParameters, 
        ddft_params: SIRDDFTParameters, state: SIRStateSpatial1D, num_threads: usize) 
    -> Self {
        // Check grid length
        let length = state.S.len();
        if length < 3 {
            panic!("Must have at least 3 gridpoints!"); // TODO: proper errors
        }
        let dx = match &state.grid {
            Grid1D::Equidistant(grid) => { grid.delta() },
            #[allow(unreachable_patterns)]
            _ => { unimplemented!("Only equidistant grids are supported for now!") }
        };
        // Threading is not available (yet) in WASM
        #[cfg(target_arch = "wasm32")]
        {
            if num_threads > 1 {
                panic!("Multithreading not supported in WASM");
            }
        }
        // Copy state into flattened state vector
        let state_vector = [state.S, state.I, state.R].concat();
        // Generate kernels
        let kernel_sd = Self::generate_kernel(ddft_params.social_distancing_range, dx, length);
        let kernel_si = Self::generate_kernel(ddft_params.self_isolation_range, dx, length);
        Self {
            state: Some(state_vector),
            dx,
            grid: state.grid,
            sir_params,
            diff_params,
            ddft_params,
            time: 0.,
            duration: 0.,
            kernel_sd,
            kernel_si,   
            #[cfg(not(target_arch = "wasm32"))]
            thread_pool: scoped_threadpool::Pool::new(num_threads as u32)
        }
    }

    fn generate_kernel(range: f64, dx: f64, length: usize) -> Vec<f64> {
        (0..length).map(|i| (-range * dx*dx * (i*i) as f64).exp()).collect()
    }

    /// Increase integration time
    pub fn add_time(&mut self, time: f64) {
        assert!(time >= 0.);
        self.duration += time;
    }

    /// Get current time and state
    ///
    /// Note that the type of the return value is not SIRStateSpatial1D, but a
    /// similar construct with references
    #[allow(non_snake_case)]
    pub fn get_result(&self) -> (f64, SIRStateSpatial1DBorrowed) {
        let state = self.state.as_ref().unwrap();
        (self.time, SIRStateSpatial1DBorrowed::from_vec(state, &self.grid))
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