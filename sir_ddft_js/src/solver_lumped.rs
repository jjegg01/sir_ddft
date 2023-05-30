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

use wasm_bindgen::prelude::*;

use sir_ddft::{
    ode::{RKF45Solver, ExplicitODESolver}, 
    SIRODEIVP
};

use crate::*;

fn result2js(time: f64, state: &sir_ddft::SIRState) -> JsValue {
    let result = js_sys::Object::new();
    js_sys::Reflect::set(&result, &"time".into(), &time.into()).unwrap();
    js_sys::Reflect::set(&result, &"S".into(), &state.S.into()).unwrap();
    js_sys::Reflect::set(&result, &"I".into(), &state.I.into()).unwrap();
    js_sys::Reflect::set(&result, &"R".into(), &state.R.into()).unwrap();
    result.into()
}

#[wasm_bindgen]
pub struct SIRSolver {
    solver: RKF45Solver<SIRODEIVP>,
    ivp: sir_ddft::SIRODEIVP
}

#[wasm_bindgen]
impl SIRSolver {
    #[wasm_bindgen(constructor)]
    pub fn new(params: SIRParameters, state: SIRState) -> Self {
        SIRSolver {
            solver: RKF45Solver::<SIRODEIVP>::new(),
            ivp: SIRODEIVP::new(params.params, state.state)
        }
    }

    pub fn add_time(&mut self, time: f64) {
        self.ivp.add_time(time);
    }    
    
    pub fn integrate(&mut self) {
        self.solver.integrate(&mut self.ivp);
    }

    pub fn get_result(&self) -> JsValue {
        let (time, state) = self.ivp.get_result();
        result2js(time, state)
    }
}