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

use pyo3::prelude::*;

#[pyclass]
#[text_signature = "(infection_parameter, recovery_rate)"]
/// Parameters of all SIR models
pub struct SIRParameters {
    pub(crate) params: sir_ddft::SIRParameters
}

#[pymethods]
impl SIRParameters {
    #[new]
    pub fn new(infection_parameter: f64, recovery_rate: f64) -> Self {
        Self {
            params: sir_ddft::SIRParameters::new(infection_parameter, recovery_rate)
        }
    }

    #[getter]
    pub fn get_infection_parameter(&self) -> PyResult<f64> {
        Ok(self.params.infection_parameter)
    }

    #[setter]
    pub fn set_infection_parameter(&mut self, infection_parameter: f64) -> PyResult<()> {
        self.params.infection_parameter = infection_parameter;
        Ok(())
    }

    #[getter]
    pub fn get_recovery_rate(&self) -> PyResult<f64> {
        Ok(self.params.recovery_rate)
    }

    #[setter]
    pub fn set_recovery_rate(&mut self, recovery_rate: f64) -> PyResult<()> {
        self.params.recovery_rate = recovery_rate;
        Ok(())
    }
}

#[pyclass]
#[text_signature = "(diffusivity_S, diffusivity_I, diffusivity_R)"]
/// Parameters of all SIR models with diffusion
pub struct SIRDiffusionParameters {
    pub(crate) diff_params: sir_ddft::SIRDiffusionParameters
}

#[pymethods]
impl SIRDiffusionParameters {
    #[allow(non_snake_case)]
    #[new]
    pub fn new(diffusivity_S : f64, diffusivity_I : f64, diffusivity_R : f64) -> Self {
        Self {
            diff_params: sir_ddft::SIRDiffusionParameters::new(
                diffusivity_S, diffusivity_I, diffusivity_R)
        }
    }

    #[allow(non_snake_case)]
    #[getter]
    pub fn get_diffusivity_S(&self) -> PyResult<f64> {
        Ok(self.diff_params.diffusivity_S)
    }

    #[allow(non_snake_case)]
    #[setter]
    pub fn set_diffusivity_S(&mut self, diffusivity_S: f64) -> PyResult<()> {
        self.diff_params.diffusivity_S = diffusivity_S;
        Ok(())
    }

    #[allow(non_snake_case)]
    #[getter]
    pub fn get_diffusivity_I(&self) -> PyResult<f64> {
        Ok(self.diff_params.diffusivity_I)
    }

    #[allow(non_snake_case)]
    #[setter]
    pub fn set_diffusivity_I(&mut self, diffusivity_I: f64) -> PyResult<()> {
        self.diff_params.diffusivity_I = diffusivity_I;
        Ok(())
    }

    #[allow(non_snake_case)]
    #[getter]
    pub fn get_diffusivity_R(&self) -> PyResult<f64> {
        Ok(self.diff_params.diffusivity_R)
    }

    #[allow(non_snake_case)]
    #[setter]
    pub fn set_diffusivity_R(&mut self, diffusivity_R: f64) -> PyResult<()> {
        self.diff_params.diffusivity_R = diffusivity_R;
        Ok(())
    }
}

#[pyclass]
#[text_signature = "(mobility_S, mobility_I, mobility_R, social_distancing_amplitude, \
social_distancing_range, self_isolation_amplitude, self_isolation_range)"]
/// Parameters of the SIR DDFT model
pub struct SIRDDFTParameters {
    pub(crate) ddft_params: sir_ddft::SIRDDFTParameters
}

#[pymethods]
impl SIRDDFTParameters {
    #[allow(non_snake_case)]
    #[new]
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

    #[allow(non_snake_case)]
    #[getter]
    pub fn get_mobility_S(&self) -> PyResult<f64> {
        Ok(self.ddft_params.mobility_S)
    }

    #[allow(non_snake_case)]
    #[setter]
    pub fn set_mobility_S(&mut self, mobility_S: f64) -> PyResult<()> {
        self.ddft_params.mobility_S = mobility_S;
        Ok(())
    }

    #[allow(non_snake_case)]
    #[getter]
    pub fn get_mobility_I(&self) -> PyResult<f64> {
        Ok(self.ddft_params.mobility_I)
    }

    #[allow(non_snake_case)]
    #[setter]
    pub fn set_mobility_I(&mut self, mobility_I: f64) -> PyResult<()> {
        self.ddft_params.mobility_I = mobility_I;
        Ok(())
    }

        #[allow(non_snake_case)]
    #[getter]
    pub fn get_mobility_R(&self) -> PyResult<f64> {
        Ok(self.ddft_params.mobility_R)
    }

    #[allow(non_snake_case)]
    #[setter]
    pub fn set_mobility_R(&mut self, mobility_R: f64) -> PyResult<()> {
        self.ddft_params.mobility_R = mobility_R;
        Ok(())
    }
}