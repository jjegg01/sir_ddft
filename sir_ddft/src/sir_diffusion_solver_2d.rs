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
//! in two spatial dimensions

use crate::{
    SIRStateSpatial2D, SIRParameters, SIRDiffusionParameters, Grid2D, Grid1D,
    helpers::*,
    ode::{ODEIVP, StopCondition},
    sir::{SIRStateSpatial2DBorrowed}
};

/// Initial value problem for the SIR model with diffusion in two spatial dimensions
///
/// Note: The model is technically a PDE, but is transformed to a high-dimensional
/// ODE via the finite difference method.
pub struct SIRDiffusion2DIVP {
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
    /// Current time of integration
    time: f64,
    /// Total duration of integration
    duration: f64
}

impl<S> ODEIVP<S> for SIRDiffusion2DIVP {
    #[allow(non_snake_case)]
    fn rhs(&mut self, _ : f64, y: &[f64]) -> Vec<f64> {
        // Number of grid points
        let n = y.len() / 3;
        // Split state vector into S,I,R
        let (S,IR) = y.split_at(n);
        let (I,R) = IR.split_at(n);
        // Allocate and split RHS vector
        let mut rhs = vec![0.;n*3];
        let (dS,dIR) = rhs.split_at_mut(n);
        let (dI,dR) = dIR.split_at_mut(n);
        // Shorthands for parameters
        let nx = self.nx;
        let ny = self.ny;
        let inf_param = self.sir_params.infection_parameter;
        let rec_rate = self.sir_params.recovery_rate;
        let diff_S = self.diff_params.diffusivity_S;
        let diff_I = self.diff_params.diffusivity_I;
        let diff_R = self.diff_params.diffusivity_R;
        // Calculate RHS
        let mut prev_y = ny - 1;
        let mut next_y = 1;
        for iy in 0..self.ny {
            let mut prev_x = nx - 1;
            let mut next_x = 1;
            for ix in 0..self.nx {
                let i = ix + iy*nx;
                // Discretized form of model equations
                dS[i] = diff_S * laplace_2d(S, 
                    prev_x, ix, next_x, prev_y, iy, next_y, nx, self.dx, self.dy)
                    - inf_param * S[i] * I[i];
                dI[i] = diff_I * laplace_2d(I, 
                    prev_x, ix, next_x, prev_y, iy, next_y, nx, self.dx, self.dy)
                    + inf_param * S[i] * I[i] - rec_rate * I[i];
                dR[i] = diff_R * laplace_2d(R, 
                    prev_x, ix, next_x, prev_y, iy, next_y, nx, self.dx, self.dy)
                    + rec_rate * I[i];
                // Update with periodicity
                prev_x = ix;
                next_x = (next_x + 1) % nx;
            }
            // Update with periodicity
            prev_y = iy;
            next_y = (next_y + 1) % ny;
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

impl SIRDiffusion2DIVP {
    /// Creates a new IVP for the SIR diffusion model
    pub fn new(sir_params: SIRParameters, diff_params: SIRDiffusionParameters, 
        state: SIRStateSpatial2D) 
    -> Self {
        let ((dx,nx),(dy,ny)) = match &state.grid {
            Grid2D::Cartesian(cart_grid) => {
                (
                    match &cart_grid.grid_x {
                        Grid1D::Equidistant(grid) => { (grid.delta(), grid.n) },
                        #[allow(unreachable_patterns)]
                        _ => { unimplemented!("Only equidistant grids in x are supported for now!") }
                    },
                    match &cart_grid.grid_y {
                        Grid1D::Equidistant(grid) => { (grid.delta(), grid.n) },
                        #[allow(unreachable_patterns)]
                        _ => { unimplemented!("Only equidistant grids in y are supported for now!") }
                    },
                )
            },
            #[allow(unreachable_patterns)]
            _ => unimplemented!("Only cartesian grids are supported for now")
        };
        if nx < 3 || ny < 3 {
            panic!("Must have at least 3 gridpoints in every direction!"); // TODO: proper errors
        }
        // Copy state into flattened state vector
        let state_vector = [state.S, state.I, state.R].concat();
        Self {
            state: Some(state_vector),
            dx, dy,
            nx, ny,
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
    /// Note that the type of the return value is not `SIRStateSpatial2D`, but a
    /// similar construct with references
    #[allow(non_snake_case)]
    pub fn get_result(&self) -> (f64, SIRStateSpatial2DBorrowed) {
        let state = self.state.as_ref().unwrap();
        (self.time, SIRStateSpatial2DBorrowed::from_vec(state, &self.grid))
    }
}