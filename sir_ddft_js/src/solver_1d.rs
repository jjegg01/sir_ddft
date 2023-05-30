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
    SIRDiffusion1DIVP, SIRDDFT1DIVP, SIRStateSpatial1DBorrowed
};

use crate::*;

fn result2js(time: f64, state: SIRStateSpatial1DBorrowed) -> JsValue {
    let result = js_sys::Object::new();
    js_sys::Reflect::set(&result, &"time".into(), &time.into()).unwrap();
    js_sys::Reflect::set(&result, &"S".into(), 
        &js_sys::Float64Array::from(state.S)).unwrap();
    js_sys::Reflect::set(&result, &"I".into(), 
        &js_sys::Float64Array::from(state.I)).unwrap();
    js_sys::Reflect::set(&result, &"R".into(), 
        &js_sys::Float64Array::from(state.R)).unwrap();
    result.into()
}

/* === SIR with diffusion === */

#[wasm_bindgen]
pub struct SIRDiffusion1DSolver {
    solver: RKF45Solver<SIRDiffusion1DIVP>,
    ivp: SIRDiffusion1DIVP
}

#[wasm_bindgen]
impl SIRDiffusion1DSolver {
    #[wasm_bindgen(constructor)]
    pub fn new(params: SIRParameters, diff_params: SIRDiffusionParameters,
        state: SIRStateSpatial1D) 
    -> Self {
        SIRDiffusion1DSolver {
            solver: RKF45Solver::<SIRDiffusion1DIVP>::new(),
            ivp: SIRDiffusion1DIVP::new(params.params, diff_params.diff_params, state.state)
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

/* === SIR-DDFT === */

#[wasm_bindgen]
pub struct SIRDDFT1DSolver {
    solver: RKF45Solver<SIRDDFT1DIVP>,
    ivp: SIRDDFT1DIVP
}

#[wasm_bindgen]
impl SIRDDFT1DSolver {
    #[wasm_bindgen(constructor)]
    pub fn new(params: SIRParameters, diff_params: SIRDiffusionParameters,
        ddft_params: SIRDDFTParameters, state: SIRStateSpatial1D) 
    -> Self {
        SIRDDFT1DSolver {
            solver: RKF45Solver::<SIRDDFT1DIVP>::new(),
            ivp: SIRDDFT1DIVP::new(params.params, diff_params.diff_params, 
                ddft_params.ddft_params, state.state, 1)
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