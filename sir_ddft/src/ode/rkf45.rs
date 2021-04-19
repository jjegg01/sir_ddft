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

use std::marker;
use super::{ExplicitODESolver, ODEIVP, StopCondition};

/// Classic Runge-Kutta-Fehlberg integrator with order 4,(5)
pub struct RKF45Solver<P: ODEIVP<Self>> {
    // Error threshold for step size adaption
    eps_0 : f64,
    // Safety factor for error threshold comparison
    beta : f64,
    // Initial
    dt : f64,
    // P is not actually saved as part of the solver, so we use PhantomMarker
    _marker: marker::PhantomData<P>
}

impl<P: ODEIVP<Self>> RKF45Solver<P> {
    /// Create a new RKF45 solver
    pub fn new() -> RKF45Solver<P> {
        RKF45Solver {
            // Sensible default values
            eps_0: 1e-5,
            beta: 0.95,
            dt: 0.1,
            _marker: marker::PhantomData
        }
    }

    /// Set error threshold for step size adaption (default: 10^-5)
    pub fn eps_0(&mut self, eps_0 : f64) -> &mut RKF45Solver<P> {
        self.eps_0 = eps_0;
        self
    }

    /// Set safety factor for error threshold comparison (default: 0.85)
    pub fn beta(&mut self, beta : f64) -> &mut RKF45Solver<P> {
        self.beta = beta;
        self
    }

    /// Initial stepsize (default: 0.1)
    pub fn dt(&mut self, dt : f64) -> &mut RKF45Solver<P> {
        self.dt = dt;
        self
    }
}

impl<P: ODEIVP<Self>> ExplicitODESolver for RKF45Solver<P> {
    type Problem = P;

    fn integrate(&self, p : &mut P) {
        // Get initial state
        let (t0, y0) = p.initial_state();
        // Get initial step size (either from solver parameters or from stop condition,
        // whichever is smaller)
        let mut stop = p.end_step(t0, &y0, &self);
        let mut dt = self.dt.min(
            match stop {
                StopCondition::ContinueUntil(t1) => t1-t0,
                _ => f64::INFINITY
            }
        );
        // Init time ans state with t0 and y0
        let mut t = t0;
        let mut y = y0;
        let dim = y.len();
        // Loop over all steps
        loop {
            // Get dt for next step or break out of loop if IVP says so
            dt = match stop {
                StopCondition::Stop => { break },
                StopCondition::Continue => dt ,
                StopCondition::ContinueUntil(t1) => {
                    if t >= t1 { break } // Is this stable?
                    dt.min(t1 - t)
                }
            };
            // -- RKF scheme --
            // Butcher tableau
            let tableau = [
                [0.,       f64::NAN,    f64::NAN,     f64::NAN,      f64::NAN,      f64::NAN, f64::NAN],
                [1./4.,    1./4.,       f64::NAN,     f64::NAN,      f64::NAN,      f64::NAN, f64::NAN],
                [3./8.,    3./32. ,     9./32.,       f64::NAN,      f64::NAN,      f64::NAN, f64::NAN],
                [12./13.,  1932./2197., -7200./2197., 7296./2197.,   f64::NAN,      f64::NAN, f64::NAN],
                [1.,       439./216.,   -8.,          3680./513.,    -845./4104.,   f64::NAN, f64::NAN],
                [1./2.,    -8./27.,     2.,           -3544./2565.,  1859./4104.,   -11./40., f64::NAN],
                [f64::NAN, 25./216.,    0.,           1408./2565.,   2197./4104.,   -1./5.,   0.],
                [f64::NAN, 16./135.,    0.,           6656./12825.,  28561./56430., -9./50.,  2./55.]
            ];
            // (Painfully) allocate 6 vectors for k_i
            let mut k: [Vec<f64>; 6] = [
                vec![0.;dim],vec![0.;dim],vec![0.;dim],vec![0.;dim],vec![0.;dim],vec![0.;dim]
            ];
            // Fill k_i vectors using tableau and RHS of problem
            for i in 0..6 {
                // Calculate k[i] via the usual RK scheme
                let tmp_t = t + dt*tableau[i][0];
                let mut tmp_y = vec![0.;dim];
                for j in 0..i {
                    tmp_y = tmp_y.into_iter()
                        .zip(k[j].iter())
                        .map(|(y,k)| y + k * tableau[i][j+1])
                        .collect();
                }
                tmp_y = tmp_y.into_iter()
                    .zip(y.iter())
                    .map(|(sum,y)| y + sum * dt)
                    .collect();
                k[i] = p.rhs(tmp_t, &tmp_y);
            }
            // Calculate two solutions for y(t+dt)
            let mut y1 = vec![0.;dim];
            let mut y2 = vec![0.;dim];
            for i in 0..dim {
                for j in 0..6 {
                    y1[i] += k[j][i] * tableau[6][j+1];
                    y2[i] += k[j][i] * tableau[7][j+1];
                }
            }
            // Calculate error
            let error = dt * y1.iter()
                .zip(y2.iter())
                .map(|(y1,y2)| (y1-y2).abs())
                .fold(0.0, |acc: f64,x| acc.max(x));
            // Compare error to threshold and step forward if below
            if error <= self.eps_0 {
                t += dt;
                y = y.iter().zip(y2).map(|(y,y2)| y + dt * y2).collect();
                dt = self.beta * dt * (self.eps_0 / error).powf(1./5.);
                stop = p.end_step(t, &y, &self);
            }
            else {
                dt = self.beta * dt * (self.eps_0 / error).powf(1./4.);
            }
        }
        p.final_state(t, y);
    }
}