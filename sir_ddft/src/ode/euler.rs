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

use std::{marker, ops::Mul};
use num_complex::ComplexFloat;
// use rustfft::num_traits::{Zero, Float, NumCast};

use super::{ExplicitODESolver, ODEIVP, StopCondition};

/// Classic Runge-Kutta-Fehlberg integrator with order 4,(5)
pub struct EulerSolver<T: ComplexFloat, P: ODEIVP<Self, T>> {
    // Step size
    dt : f64,
    // P is not actually saved as part of the solver, so we use PhantomMarker
    _marker: marker::PhantomData<T>,
    _marker2: marker::PhantomData<P>
}

impl<T: ComplexFloat, P: ODEIVP<Self,T>> EulerSolver<T,P> {
    /// Create a new RKF45 solver
    pub fn new() -> EulerSolver<T,P> {
        Self {
            // Sensible default values
            dt: 0.001,
            _marker: marker::PhantomData,
            _marker2: marker::PhantomData,
        }
    }

    /// Initial stepsize (default: 0.1)
    pub fn dt(&mut self, dt : f64) -> &mut EulerSolver<T,P> {
        self.dt = dt;
        self
    }
}

impl<T: ComplexFloat + Mul<f64, Output=T>, P: ODEIVP<Self,T>> ExplicitODESolver<T> for EulerSolver<T,P> {
    type Problem = P;

    fn integrate(&mut self, p : &mut P) {
        // Get initial state
        let (t0, y0) = p.initial_state();
        // Get initial step size (either from solver parameters or from stop
        // condition, whichever is smaller)
        let mut stop = p.end_step(t0, &y0, &self);
        let mut dt = self.dt.min(
            match stop {
                StopCondition::ContinueUntil(t1) => t1-t0,
                _ => f64::INFINITY
            }
        );
        // Initialize time and state with t0 and y0
        let mut t = t0;
        let mut y = y0;
        // Dimensionality of the problem
        let dim = y.len();
        // Allocate vectors for k_i, a temporary state vector tmp_y and the
        // two solutions y1 and y2
        // let mut k: [Vec<T>; 6] = Default::default();
        // k.iter_mut().for_each(|k_i| k_i.resize(dim, T::zero()));
        // let mut tmp_y = vec![T::zero();dim];
        // let mut y1 = vec![T::zero();dim];
        let mut rhs = vec![T::zero();dim];
        // Loop over all steps
        loop {
            // Get dt for next step or break out of loop if IVP says so
            dt = match stop {
                StopCondition::Stop => { break },
                StopCondition::Continue => dt ,
                StopCondition::ContinueUntil(t1) => {
                    if t - t1 >= -t * f64::EPSILON { break }
                    dt.min(t1 - t)
                }
            };
            // Calculate two solutions for (y(t+dt) - y(t)) / h
            p.rhs(t, y.as_slice(), rhs.as_mut_slice());
            for i in 0..dim {
                y[i] = y[i] + rhs[i] * dt ;
            }
            t = t + dt;
            stop = p.end_step(t, &y, &self);
        } // end of loop
        p.final_state(t, y);
    }
}