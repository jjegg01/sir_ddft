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

//! 1D grid and state vector

/// Types of one-dimensional grids. 
///
/// For now we only support equidistant grids but this might change in future.
#[derive(Clone)]
pub enum Grid1D {
    /// Equidistant gridpoints
    Equidistant(EquidistantGrid1D)
}

impl Grid1D {
    /// Shorthand to create a new equidistant grid
    pub fn new_equidistant(xlim: (f64,f64), n: usize) -> Self {
        Grid1D::Equidistant(EquidistantGrid1D{xlim,n})
    }

    /// Shorthand for getting an iterator for this grid
    pub fn grid(&self) -> Box<dyn Iterator<Item=f64>> {
        #[allow(unreachable_patterns)]
        match self {
            Grid1D::Equidistant(grid) => Box::new(grid.grid()),
            _ => unimplemented!()
        }
    }
}

/// Equidistant grid in 1D starting at `xlim.0` and ending at `xlim.1`
#[derive(Clone)]
pub struct EquidistantGrid1D {
    /// Grid bounds
    pub xlim: (f64,f64),
    /// Number of grid points
    pub n: usize
}

impl EquidistantGrid1D {
    /// Get a grid iterator (might change to generator (or `Box<dyn Iterator<...>>`) in future)
    pub fn grid(&self) -> EquidistantGrid1DIter { // TODO: Decide on return type
        EquidistantGrid1DIter {
            i: 0,
            n: self.n,
            dx: self.delta(),
            offset: self.xlim.0
        }
    }

    /// Get gridpoint separation
    pub fn delta(&self) -> f64 {
        (self.xlim.1 - self.xlim.0) / (self.n - 1) as f64
    }
}

/// Iterator over 1D equidistant grid (workaround until generators are stable)
pub struct EquidistantGrid1DIter {
    i: usize,
    n: usize,
    dx: f64,
    offset: f64
}

impl Iterator for EquidistantGrid1DIter {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.n {
            None
        }
        else {
            let res = self.offset + self.dx * self.i as f64;
            self.i += 1;
            Some(res)
        }
    }
}

/// State of spatially resolved SIR models in one dimension (SIR PDEs)
#[allow(non_snake_case)]
#[derive(Clone)]
pub struct SIRStateSpatial1D {
    /// Susceptible population for all grid points
    pub S: Vec<f64>,
    /// Infected population for all grid points
    pub I: Vec<f64>,
    /// Recovered population for all grid points
    pub R: Vec<f64>,
    /// Spatial grid
    pub grid: Grid1D
}

impl SIRStateSpatial1D {
    /// Create a new state for a spatial SIR model in 2D.
    ///
    /// The state will be initialized on the given grid by calling `initfunc(x)`
    /// at each grid point `(x)`.
    #[allow(non_snake_case)]
    pub fn new<F>(grid: Grid1D, initfunc: F) -> Self
        where F: Fn(f64) -> (f64,f64,f64)
    {
        let mut S = vec![];
        let mut I = vec![];
        let mut R = vec![];
        for x in grid.grid() {
            let result = initfunc(x);
            S.push(result.0);
            I.push(result.1);
            R.push(result.2);
        }
        Self {
            S,I,R,
            grid
        }
    }
}

/// Borrowed view of a [SIRStateSpatial1D]
#[allow(non_snake_case)]
pub struct SIRStateSpatial1DBorrowed<'a> {
    pub S: &'a[f64],
    pub I: &'a[f64],
    pub R: &'a[f64],
    pub grid: &'a Grid1D,
}

impl<'a> SIRStateSpatial1DBorrowed<'a> {
    /// Create a `SIRStateSpatial1DBorrowed` from a contiguous vector of `f64`
    /// values (first all S values, then all I values, finally all R values)
    #[allow(non_snake_case)]
    pub(crate) fn from_vec(v: &'a Vec<f64>, grid: &'a Grid1D) -> Self {
        let n = v.len() / 3;
        let (S,IR) = v.split_at(n);
        let (I,R) = IR.split_at(n);
        Self {
            S,I,R,grid
        }
    }

    pub fn to_owned(&self) -> SIRStateSpatial1D {
        SIRStateSpatial1D {
            S: self.S.to_owned(),
            I: self.I.to_owned(),
            R: self.R.to_owned(),
            grid: self.grid.clone()
        }
    }
}