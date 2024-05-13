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

use paste::paste;
use pyo3::prelude::*;

macro_rules! parameter_wrapper {
    ($typename: ident, $signature:literal, $($field:ident),*) => {
        #[allow(non_snake_case)]
        #[pyclass]
        #[pyo3(text_signature = $signature)]
        pub struct $typename {
            pub(crate) inner: sir_ddft::$typename
        }

        paste!{
            #[allow(non_snake_case)]
            #[pymethods]
            impl $typename {
                #[new]
                pub fn new($($field : f64, )*) -> Self {
                    Self {
                        inner: sir_ddft::$typename::new($($field, )*)
                    }
                }

                $(
                    #[getter]
                    pub fn [<get_ $field>](&self) -> PyResult<f64> {
                        Ok(self.inner.$field)
                    }

                    #[setter]
                    pub fn [<set_ $field>](&mut self, $field: f64) -> PyResult<()> {
                        self.inner.$field = $field;
                        Ok(())
                    }

                )*
            }
        }
    };
}

parameter_wrapper!(
    SIRParameters,
    "(infection_parameter, recovery_rate, mortality_rate)",
    infection_parameter,
    recovery_rate,
    mortality_rate
);

parameter_wrapper!(
    SIRDiffusionParameters,
    "(diffusivity_S, diffusivity_I, diffusivity_R)",
    diffusivity_S,
    diffusivity_I,
    diffusivity_R
);

parameter_wrapper!(
    SIRDDFTParameters,
    "(mobility_S, mobility_I, mobility_R, social_distancing_amplitude, social_distancing_range, \
        self_isolation_amplitude, self_isolation_range)",
    mobility_S,
    mobility_I,
    mobility_R,
    social_distancing_amplitude,
    social_distancing_range,
    self_isolation_amplitude,
    self_isolation_range
);

parameter_wrapper!(
    SZParameters,
    "(bite_parameter, kill_parameter)",
    bite_parameter,
    kill_parameter
);

parameter_wrapper!(
    SZDiffusionParameters,
    "(diffusivity_S, diffusivity_Z)",
    diffusivity_S,
    diffusivity_Z
);

parameter_wrapper!(
    SZDDFTParameters,
    "(mobility_S, mobility_Z, fear_amplitude, fear_range, hunger_amplitude, hunger_range)",
    mobility_S,
    mobility_Z,
    fear_amplitude,
    fear_range,
    hunger_amplitude,
    hunger_range
);
