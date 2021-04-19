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

//! 0D (lumped) model state vector

/// State of the SIR model (SIR ODE)
#[allow(non_snake_case)]
#[derive(Clone,Copy)]
pub struct SIRState {
    /// Susceptible population
    pub S: f64,
    /// Infected population
    pub I: f64,
    /// Recovered population
    pub R: f64
}

impl SIRState {
    /// Create a new SIR state
    #[allow(non_snake_case)]
    pub fn new(S: f64, I: f64, R: f64) -> SIRState {
        SIRState {S, I, R}
    }

    /// Single call to update S, I and R
    #[allow(non_snake_case)]
    pub fn update(&mut self, S: f64, I: f64, R: f64) {
        self.S = S;
        self.I = I;
        self.R = R;
    }
}