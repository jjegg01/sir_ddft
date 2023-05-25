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

//! Solver for the SIR ODE (i.e. the SIR model without spatial resolution)

use super::ode::{ODEIVP, StopCondition};
use super::{SIRParameters, SIRState};

/// Initial value problem for the SIR model 
pub struct SIRODEIVP {
    /// Parameters of the SIR model
    param: SIRParameters,
    /// State vector of the SIR model
    state: SIRState,
    /// Current time of integration
    time: f64,
    /// Total duration of integration
    duration: f64
}

impl<S> ODEIVP<S> for SIRODEIVP {
    #[inline(always)]
    fn rhs(&mut self, _ : f64, y: &[f64], rhs: &mut[f64]) {
        let c = self.param.infection_parameter;
        let w = self.param.recovery_rate;
        #[allow(non_snake_case)]
        let (S,I) = (y[0], y[1]);
        rhs[0] = -c*S*I;
        rhs[1] = c*S*I - w*I;
        rhs[2] = w*I;
    }

    #[inline(always)]
    fn initial_state(&mut self) -> (f64, Vec<f64>) {
        let state = &self.state;
        (self.time, vec![state.S, state.I, state.R])
    }

    #[inline(always)]
    fn end_step(&mut self, _ : f64, _: &[f64], _ : &S) -> crate::ode::StopCondition {
        StopCondition::ContinueUntil(self.duration)
    }

    #[inline(always)]
    fn final_state(&mut self, t: f64, y: Vec<f64>) {
        #[allow(non_snake_case)]
        let (S,I,R) = (y[0], y[1], y[2]);
        self.state.update(S, I, R);
        self.time = t;
    }
}

impl SIRODEIVP {
    /// Create a new IVP of the SIR model with a given set of parameters and an
    /// initial state
    pub fn new(param: SIRParameters, state: SIRState) -> SIRODEIVP {
        SIRODEIVP {
            param,
            state,
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
    pub fn get_result(&self) -> (f64, &SIRState) {
        (self.time, &self.state)
    }
}