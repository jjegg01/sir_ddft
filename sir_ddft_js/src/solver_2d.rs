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
    SIRDiffusion2DIVP, SIRDDFT2DIVP, SIRStateSpatial2DBorrowed
};

use crate::*;

fn result2js(time: f64, state: SIRStateSpatial2DBorrowed, nx: usize) -> JsValue {
    let result = js_sys::Object::new();
    js_sys::Reflect::set(&result, &"time".into(), &time.into()).unwrap();
    js_sys::Reflect::set(&result, &"S".into(), &reshape_state_vec(nx, state.S)).unwrap();
    js_sys::Reflect::set(&result, &"I".into(), &reshape_state_vec(nx, state.I)).unwrap();
    js_sys::Reflect::set(&result, &"R".into(), &reshape_state_vec(nx, state.R)).unwrap();
    result.into()
}

/* === SIR with diffusion === */

#[wasm_bindgen]
pub struct SIRDiffusion2DSolver {
    solver: RKF45Solver<SIRDiffusion2DIVP>,
    ivp: SIRDiffusion2DIVP,
    nx: usize
}

#[wasm_bindgen]
impl SIRDiffusion2DSolver {
    #[wasm_bindgen(constructor)]
    pub fn new(params: SIRParameters, diff_params: SIRDiffusionParameters,
        state: SIRStateSpatial2D) 
    -> Self {
        let nx = state.nx;
        SIRDiffusion2DSolver {
            solver: RKF45Solver::<SIRDiffusion2DIVP>::new(),
            ivp: SIRDiffusion2DIVP::new(params.params, diff_params.diff_params, state.state),
            nx
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
        result2js(time, state, self.nx)
    }
}

/* === SIR-DDFT === */

#[wasm_bindgen]
pub struct SIRDDFT2DSolver {
    solver: RKF45Solver<SIRDDFT2DIVP>,
    ivp: SIRDDFT2DIVP,
    nx: usize
}

#[wasm_bindgen]
impl SIRDDFT2DSolver {
    #[wasm_bindgen(constructor)]
    pub fn new(params: SIRParameters, diff_params: SIRDiffusionParameters,
        ddft_params: SIRDDFTParameters, state: SIRStateSpatial2D) 
    -> Self {
        let nx = state.nx;
        SIRDDFT2DSolver {
            solver: RKF45Solver::<SIRDDFT2DIVP>::new(),
            ivp: SIRDDFT2DIVP::new(params.params, diff_params.diff_params, 
                ddft_params.ddft_params, state.state, 1),
            nx
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
        result2js(time, state, self.nx)
    }
}