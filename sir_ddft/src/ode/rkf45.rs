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
use num_traits::{Zero, Float, NumCast};

use super::{ExplicitODESolver, ODEIVP, StopCondition};

/// Classic Runge-Kutta-Fehlberg integrator with order 4,(5)
pub struct RKF45Solver<P: ODEIVP<Self, T>, T: ComplexFloat> {
    // Error threshold for step size adaption
    eps_0 : T::Real,
    // Safety factor for error threshold comparison
    beta : T::Real,
    // Initial step size
    dt_0 : f64,
    // P is not actually saved as part of the solver, so we use PhantomMarker
    _marker: marker::PhantomData<P>
}

impl<P: ODEIVP<Self,T>, T: ComplexFloat> RKF45Solver<P,T> {
    /// Create a new RKF45 solver
    pub fn new() -> RKF45Solver<P,T> {
        RKF45Solver {
            // Sensible default values
            eps_0: <T::Real as NumCast>::from(1e-5).unwrap(),
            beta: <T::Real as NumCast>::from(0.95).unwrap(),
            dt_0: 0.1,
            _marker: marker::PhantomData,
        }
    }

    /// Set error threshold for step size adaption (default: 10^-5)
    pub fn eps_0(&mut self, eps_0 : T::Real) -> &mut RKF45Solver<P,T> {
        self.eps_0 = eps_0;
        self
    }

    /// Set safety factor for error threshold comparison (default: 0.85)
    pub fn beta(&mut self, beta : T::Real) -> &mut RKF45Solver<P,T> {
        self.beta = beta;
        self
    }

    /// Initial stepsize (default: 0.1)
    pub fn dt(&mut self, dt : f64) -> &mut RKF45Solver<P,T> {
        self.dt_0 = dt;
        self
    }
}

impl<T: ComplexFloat + Mul<f64, Output=T>, P: ODEIVP<Self,T>> ExplicitODESolver<T> for RKF45Solver<P,T> {
    type Problem = P;

    fn integrate(&mut self, p : &mut P) {
        // Get initial state
        let (t0, y0) = p.initial_state();
        // Get initial step size (either from solver parameters or from stop
        // condition, whichever is smaller)
        let mut stop = p.end_step(t0, &y0, &self);
        let mut dt = self.dt_0.min(
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
        let mut k: [Vec<T>; 6] = Default::default();
        k.iter_mut().for_each(|k_i| k_i.resize(dim, T::zero()));
        let mut tmp_y = vec![T::zero();dim];
        let mut y1 = vec![T::zero();dim];
        let mut y2 = vec![T::zero();dim];
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
            // -- RKF scheme --
            // Butcher tableau for RKF45
            const TABLEAU: [[f64;7];8] = [
                [0.,       f64::NAN,    f64::NAN,     f64::NAN,      f64::NAN,      f64::NAN, f64::NAN],
                [1./4.,    1./4.,       f64::NAN,     f64::NAN,      f64::NAN,      f64::NAN, f64::NAN],
                [3./8.,    3./32. ,     9./32.,       f64::NAN,      f64::NAN,      f64::NAN, f64::NAN],
                [12./13.,  1932./2197., -7200./2197., 7296./2197.,   f64::NAN,      f64::NAN, f64::NAN],
                [1.,       439./216.,   -8.,          3680./513.,    -845./4104.,   f64::NAN, f64::NAN],
                [1./2.,    -8./27.,     2.,           -3544./2565.,  1859./4104.,   -11./40., f64::NAN],
                [f64::NAN, 25./216.,    0.,           1408./2565.,   2197./4104.,   -1./5.,   0.      ],
                [f64::NAN, 16./135.,    0.,           6656./12825.,  28561./56430., -9./50.,  2./55.  ]
            ];
            // Fill k_i vectors using tableau and RHS of the IVP
            for i in 0..6 {
                // Calculate k[i] via the usual RK scheme
                let tmp_t = t + dt*TABLEAU[i][0];
                tmp_y.fill(T::zero());
                for j in 0..i {
                    tmp_y = tmp_y.into_iter()
                        .zip(k[j].iter())
                        .map(|(sum,k)| sum + *k * TABLEAU[i][j+1])
                        .collect();
                }
                tmp_y = tmp_y.into_iter()
                    .zip(y.iter())
                    .map(|(sum,y_old)| *y_old + sum * dt)
                    .collect();
                p.rhs(tmp_t, &tmp_y, k[i].as_mut_slice());
            }
            // Calculate two solutions for (y(t+dt) - y(t)) / h
            y1.fill(T::zero());
            y2.fill(T::zero());
            for j in 0..dim {
                for i in 0..6 {
                    y1[j] = y1[j] + k[i][j] * TABLEAU[6][i+1];
                    y2[j] = y2[j] + k[i][j] * TABLEAU[7][i+1];
                }
            }
            // Calculate error
            let error = y1.iter()
                .zip(y2.iter())
                .map(|(y1,y2)| (*y1-*y2).abs())
                .fold(T::Real::zero(), |acc: T::Real,x| acc.max(x))
                * <T::Real as NumCast>::from(dt).unwrap();
            // Compare error to threshold and step forward if below
            dt = <f64 as NumCast>::from(if error <= self.eps_0 
                {
                    t += dt;
                    y = y.iter()
                        .zip(y2.iter())
                        .map(|(y,y2)| *y + *y2 * dt)
                        .collect();
                    stop = p.end_step(t, &y, &self);
                    self.beta * <T::Real as NumCast>::from(dt).unwrap() * Float::powf(self.eps_0 / error, <T::Real as NumCast>::from(1./5.).unwrap())
                }
                else {
                    self.beta * <T::Real as NumCast>::from(dt).unwrap() * Float::powf(self.eps_0 / error, <T::Real as NumCast>::from(1./4.).unwrap())
                }).unwrap()
        } // end of loop
        p.final_state(t, y);
    }
}