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

//! Definitions of common elements for different SIR models

/// Parameters for the most simplistic SIR model (ODE)

#[derive(Clone)]
pub struct SIRParameters {
    /// Infection parameter
    ///
    /// Note: In the SIR model this parameter is the effective contact rate (*c*<sub>eff</sub>)
    /// (i.e. has the dimension of an inverse time), while in the 1D and 2D models 
    /// this parameter (*c*) has a dimension of length per time (1D) or area per time (2D)
    /// respectively.
    pub infection_parameter: f64,
    /// Recovery rate (i.e. rate at which infected transition to the recovered population)
    pub recovery_rate: f64,
    /// Mortality rate (i.e. rate at which infected decrease without a transition to the recovered population)
    pub mortality_rate: f64
}

impl SIRParameters {
    /// Create a new set of rate parameters for SIR models
    pub fn new(infection_parameter: f64, recovery_rate: f64, mortality_rate: f64) -> SIRParameters {
        SIRParameters { infection_parameter, recovery_rate, mortality_rate }
    }
}

/// Additional parameters for the SIR model with diffusion
#[allow(non_snake_case)]
#[derive(Clone)]
pub struct SIRDiffusionParameters {
    /// Diffusion constant for S field
    pub diffusivity_S : f64,
    /// Diffusion constant for I field
    pub diffusivity_I : f64,
    /// Diffusion constant for R field
    pub diffusivity_R : f64,
}

impl SIRDiffusionParameters {
    /// Create a new set of diffusion parameters for a spatial SIR model
    #[allow(non_snake_case)]
    pub fn new(diffusivity_S : f64, diffusivity_I : f64, diffusivity_R : f64) 
        -> SIRDiffusionParameters
    {
        SIRDiffusionParameters { diffusivity_S, diffusivity_I, diffusivity_R }
    }
}

/// Additional parameters for the SIR DDFT / PFC model
#[allow(non_snake_case)]
#[derive(Clone)]
pub struct SIRDDFTParameters {
    /// Mobility parameter for S field
    pub mobility_S : f64,
    /// Mobility parameter for I field
    pub mobility_I : f64,
    /// Mobility parameter for R field
    pub mobility_R : f64,
    /// Amplitude of Gaussian interaction kernel for social distancing
    pub social_distancing_amplitude: f64,
    /// Range of Gaussian interaction kernel for social distancing
    pub social_distancing_range: f64,
    /// Amplitude of Gaussian interaction kernel for self isolation
    pub self_isolation_amplitude: f64,
    /// Amplitude of Gaussian interaction kernel for self isolation
    pub self_isolation_range: f64,
}

impl SIRDDFTParameters {
    #[allow(non_snake_case)]
    pub fn new(mobility_S: f64, mobility_I: f64, mobility_R: f64,
        social_distancing_amplitude: f64, social_distancing_range: f64,
        self_isolation_amplitude: f64, self_isolation_range: f64
    ) -> Self {
        Self {
            mobility_S, mobility_I, mobility_R,
            social_distancing_amplitude, social_distancing_range,
            self_isolation_amplitude, self_isolation_range
        }
    }
}

#[derive(Clone)]
pub struct SZParameters {
    /// Parameter controlling the probability of a zombie biting a human
    pub bite_parameter: f64,
    /// Parameter controlling the probability of a human killing a zombie
    pub kill_parameter: f64
}

impl SZParameters {
    /// Create a new set of diffusion parameters for a spatial SIR model
    #[allow(non_snake_case)]
    pub fn new(bite_parameter : f64, kill_parameter: f64) 
        -> Self
    {
        SZParameters { bite_parameter, kill_parameter }
    }
}

/// Additional parameters for the SIR model with diffusion
#[allow(non_snake_case)]
#[derive(Clone)]
pub struct SZDiffusionParameters {
    /// Diffusion constant for S field
    pub diffusivity_S : f64,
    /// Diffusion constant for Z field
    pub diffusivity_Z : f64
}

impl SZDiffusionParameters {
    /// Create a new set of diffusion parameters for a spatial SIR model
    #[allow(non_snake_case)]
    pub fn new(diffusivity_S : f64, diffusivity_Z: f64) 
        -> Self
    {
        SZDiffusionParameters { diffusivity_S, diffusivity_Z }
    }
}

#[allow(non_snake_case)]
#[derive(Clone)]
pub struct SZDDFTParameters {
    /// Mobility parameter for S field
    pub mobility_S : f64,
    /// Mobility parameter for I field
    pub mobility_Z : f64,
    /// Amplitude of Gaussian interaction kernel for humans fearing zombies
    pub fear_amplitude: f64,
    /// Range of Gaussian interaction kernel for humans fearing zombies
    pub fear_range: f64,
    /// Amplitude of Gaussian interaction kernel for zombies hungering after humans
    pub hunger_amplitude: f64,
    /// Amplitude of Gaussian interaction kernel for zombies hungering after humans
    pub hunger_range: f64,
}

impl SZDDFTParameters {
    #[allow(non_snake_case)]
    pub fn new(mobility_S: f64, mobility_Z: f64,
        fear_amplitude: f64, fear_range: f64,
        hunger_amplitude: f64, hunger_range: f64
    ) -> Self {
        Self {
            mobility_S, mobility_Z,
            fear_amplitude, fear_range,
            hunger_amplitude, hunger_range
        }
    }
}

