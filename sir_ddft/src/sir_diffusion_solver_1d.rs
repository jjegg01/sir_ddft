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
//! in one spatial dimension

use crate::{
    SIRStateSpatial1D, SIRParameters, SIRDiffusionParameters, Grid1D,
    helpers::*,
    ode::{ODEIVP, StopCondition},
    sir::{SIRStateSpatial1DBorrowed}
};

/// Initial value problem for the SIR model with diffusion in one spatial dimension
///
/// Note: The model is technically a PDE, but is transformed to a high-dimensional
/// ODE via the finite difference method.
pub struct SIRDiffusion1DIVP {
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
    /// Current time of integration
    time: f64,
    /// Total duration of integration
    duration: f64
}

impl<S> ODEIVP<S> for SIRDiffusion1DIVP {
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
        let diff_S = self.diff_params.diffusivity_S;
        let diff_I = self.diff_params.diffusivity_I;
        let diff_R = self.diff_params.diffusivity_R;
        // Calculate RHS
        for i in 0..n {
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
            // Discretized form of model equations
            dS[i] = diff_S * laplace_1d(S, prev, i, next, self.dx)
                - inf_param * S[i] * I[i];
            dI[i] = diff_I * laplace_1d(I, prev, i, next, self.dx)
                + inf_param * S[i] * I[i] - rec_rate * I[i];
            dR[i] = diff_R * laplace_1d(R, prev, i, next, self.dx)
                + rec_rate * I[i];
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

impl SIRDiffusion1DIVP {
    /// Creates a new IVP for the SIR diffusion model
    pub fn new(sir_params: SIRParameters, diff_params: SIRDiffusionParameters, 
        state: SIRStateSpatial1D) 
    -> Self {
        if state.S.len() < 3 {
            panic!("Must have at least 3 gridpoints!"); // TODO: proper errors
        }
        let dx = match &state.grid {
            Grid1D::Equidistant(grid) => { grid.delta() },
            #[allow(unreachable_patterns)]
            _ => { unimplemented!("Only equidistant grids are supported for now!") }
        };
        // Copy state into flattened state vector
        let state_vector = [state.S, state.I, state.R].concat();
        Self {
            state: Some(state_vector),
            dx,
            grid: state.grid,
            sir_params,
            diff_params,
            time: 0.,
            duration: 0.,
        }
    }

    /// Increase integration time
    pub fn add_time(&mut self, time: f64) {
        assert!(time >= 0.);
        self.duration += time;
    }

    /// Get current time and state
    ///
    /// Note that the type of the return value is not `SIRStateSpatial1D`, but a
    /// similar construct with references
    #[allow(non_snake_case)]
    pub fn get_result(&self) -> (f64, SIRStateSpatial1DBorrowed) {
        let state = self.state.as_ref().unwrap();
        (self.time, SIRStateSpatial1DBorrowed::from_vec(state, &self.grid))
    }
}