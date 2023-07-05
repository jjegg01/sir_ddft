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

//! General components for the SIR model

use wasm_bindgen::prelude::*;

// Wrapper for SIRParameters
#[wasm_bindgen]
pub struct SIRParameters {
    pub(crate) params: sir_ddft::SIRParameters
}

#[wasm_bindgen]
impl SIRParameters {
    #[wasm_bindgen(constructor)]
    pub fn new(infection_parameter: f64, recovery_rate: f64, mortality_rate: f64) -> Self {
        Self {
            params: sir_ddft::SIRParameters::new(infection_parameter, recovery_rate, mortality_rate)
        }
    }
}

// Wrapper for SIRDiffusionParameters
#[wasm_bindgen]
pub struct SIRDiffusionParameters {
    pub(crate) diff_params: sir_ddft::SIRDiffusionParameters
}

#[wasm_bindgen]
impl SIRDiffusionParameters {
    #[allow(non_snake_case)]
    #[wasm_bindgen(constructor)]
    pub fn new(diffusivity_S : f64, diffusivity_I : f64, diffusivity_R : f64) -> Self {
        Self {
            diff_params: sir_ddft::SIRDiffusionParameters::new(
                diffusivity_S, diffusivity_I, diffusivity_R)
        }
    }
}

// Wrapper for SIRDDFTParameters
#[wasm_bindgen]
pub struct SIRDDFTParameters {
    pub(crate) ddft_params: sir_ddft::SIRDDFTParameters
}

#[wasm_bindgen]
impl SIRDDFTParameters {
    #[allow(non_snake_case)]
    #[wasm_bindgen(constructor)]
    pub fn new(mobility_S: f64, mobility_I: f64, mobility_R: f64,
        social_distancing_amplitude: f64, social_distancing_range: f64,
        self_isolation_amplitude: f64, self_isolation_range: f64
    ) -> Self {
        Self {
            ddft_params: sir_ddft::SIRDDFTParameters::new(
                mobility_S, mobility_I, mobility_R,
                social_distancing_amplitude, social_distancing_range,
                self_isolation_amplitude, self_isolation_range)
        }
    }
}