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

/* === Lumped === */

#[allow(non_snake_case)]
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct SIRState {
    pub(crate) state: sir_ddft::SIRState
}

#[wasm_bindgen]
impl SIRState {
    #[allow(non_snake_case)]
    #[wasm_bindgen(constructor)]
    pub fn new(S: f64, I: f64, R: f64) -> Self {
        SIRState { state: sir_ddft::SIRState::new(S, I, R) }
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen(getter)]
    pub fn S(&self) -> f64 {
        self.state.S
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen(getter)]
    pub fn I(&self) -> f64 {
        self.state.I
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen(getter)]
    pub fn R(&self) -> f64 {
        self.state.R
    }
}

/* === 1D === */

#[wasm_bindgen]
pub struct Grid1D {
    grid: sir_ddft::Grid1D
}

#[wasm_bindgen]
impl Grid1D {
    #[wasm_bindgen]
    pub fn new_equidistant(xlo: f64, xhi: f64, n: usize) -> Self {
        Self {
            grid: sir_ddft::Grid1D::new_equidistant((xlo, xhi), n)
        }
    }
}

#[wasm_bindgen]
pub struct SIRStateSpatial1D {
    pub(crate) state: sir_ddft::SIRStateSpatial1D
}

#[wasm_bindgen]
impl SIRStateSpatial1D {
    #[allow(non_snake_case)]
    #[wasm_bindgen(constructor)]
    pub fn new(grid: Grid1D, initfunc: &js_sys::Function) -> Self {
        let this = JsValue::null();
        Self {
            state: sir_ddft::SIRStateSpatial1D::new(grid.grid, |x| {
                let x = JsValue::from(x);
                let ret = initfunc.call1(&this, &x).expect("Error in initfunc for grid!");
                let ret = js_sys::Array::from(&ret);
                let S = ret.get(0).as_f64().expect("Invalid value for S returned!");
                let I = ret.get(1).as_f64().expect("Invalid value for I returned!");
                let R = ret.get(2).as_f64().expect("Invalid value for R returned!");
                (S,I,R)
            })
        }
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen(getter)]
    pub fn S(&self) -> JsValue {
        js_sys::Float64Array::from(self.state.S.as_slice()).into()
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen(getter)]
    pub fn I(&self) -> JsValue {
        js_sys::Float64Array::from(self.state.I.as_slice()).into()
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen(getter)]
    pub fn R(&self) -> JsValue {
        js_sys::Float64Array::from(self.state.R.as_slice()).into()
    }
}

/* === 2D === */

pub(crate) fn reshape_state_vec(nx: usize, state_vec: &[f64]) -> JsValue {
    let result = js_sys::Array::new();
    let chunks = state_vec.chunks(nx);
    for (i,chunk) in chunks.enumerate() {
        result.set(i as u32, js_sys::Float64Array::from(chunk).into());
    }
    result.into()
}

#[wasm_bindgen]
pub struct Grid2D {
    grid: sir_ddft::Grid2D
}

#[wasm_bindgen]
impl Grid2D {
    #[wasm_bindgen]
    pub fn new_equidistant(xlo: f64, xhi: f64, ylo: f64, yhi: f64, nx: usize, ny: usize) -> Self {
        Self {
            grid: sir_ddft::Grid2D::new_cartesian(
                sir_ddft::Grid1D::new_equidistant((xlo, xhi), nx),
                sir_ddft::Grid1D::new_equidistant((ylo, yhi), ny),
            )
        }
    }
}

#[wasm_bindgen]
pub struct SIRStateSpatial2D {
    pub(crate) state: sir_ddft::SIRStateSpatial2D,
    pub(crate) nx: usize
}

#[wasm_bindgen]
impl SIRStateSpatial2D {
    #[allow(non_snake_case)]
    #[wasm_bindgen(constructor)]
    pub fn new(grid: Grid2D, initfunc: &js_sys::Function) -> Self {
        let this = JsValue::null();
        #[allow(unreachable_patterns)]
        let nx = match &grid.grid { 
            sir_ddft::Grid2D::Cartesian(grid) => match &grid.grid_x {
                sir_ddft::Grid1D::Equidistant(grid) => grid.n,
                _ => panic!("Only equidistant grids allowed (probably a bug)")
            },
            _ => panic!("Only cartesian grids allowed (probably a bug)")
        };
        Self {
            state: sir_ddft::SIRStateSpatial2D::new(grid.grid, |x,y| {
                let x = JsValue::from(x);
                let y = JsValue::from(y);
                let ret = initfunc.call2(&this, &x, &y).expect("Error in initfunc for grid!");
                let ret = js_sys::Array::from(&ret);
                let S = ret.get(0).as_f64().expect("Invalid value for S returned!");
                let I = ret.get(1).as_f64().expect("Invalid value for I returned!");
                let R = ret.get(2).as_f64().expect("Invalid value for R returned!");
                (S,I,R)
            }),
            nx
        }
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen(getter)]
    pub fn S(&self) -> JsValue {
        reshape_state_vec(self.nx, self.state.S.as_slice())
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen(getter)]
    pub fn I(&self) -> JsValue {
        reshape_state_vec(self.nx, self.state.I.as_slice())
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen(getter)]
    pub fn R(&self) -> JsValue {
        reshape_state_vec(self.nx, self.state.R.as_slice())
    }
}