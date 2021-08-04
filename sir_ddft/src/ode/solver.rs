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

/// Stop condition for integration:
/// * Continue: Continue integration without bound in `t`
/// * ContinueUntil: Continue, but only until an upper bound for `t` is hit
/// * Stop: Stop integration now
pub enum StopCondition {
    Continue,
    ContinueUntil(f64),
    Stop
}

/// General trait implementing an initial value problem in the form of an
/// ordinary differential equation
///
/// Note: The generic type S can be used to pass additional information from
/// the solver to the post-step processing. To prevent infinite recursion on
/// trait bound checking, it is not constrained to ExplicitODESolver. Any
/// other type is useless in this context anyway.
pub trait ODEIVP<S> {
    /// Returns right hand side (i.e. the value of `f`) of IVP `y'=f(t,y)`
    fn rhs(&mut self, t : f64, y: &[f64], rhs: &mut[f64]);
    /// Returns initial state `(t_0, y_0)` such that `y(t_0) = y_0`
    fn initial_state(&mut self) -> (f64, Vec<f64>);
    /// Called at the end of each integration step (and once for `t_0`)
    fn end_step(&mut self, t : f64, y: &[f64], solver: &S) -> StopCondition;
    /// Called at the end of integration giving back the state taken in initial_state
    fn final_state(&mut self, t: f64, y: Vec<f64>);
}

/// Trait representing a minimal interface for an explicit solver for ODEs.
pub trait ExplicitODESolver : Sized {
    /// IVP to solve
    type Problem : ODEIVP<Self>;

    /// Integrate a given IVP with this integrator
    fn integrate(&mut self, p : &mut Self::Problem);
}