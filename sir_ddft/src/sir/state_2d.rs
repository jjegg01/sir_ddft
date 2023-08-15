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

//! 2D grid and state vector

use super::Grid1D;

/// Types of two-dimensional grids.
#[derive(Clone)]
pub enum Grid2D {
    /// Grid made from cartesian product of 1D grids
    Cartesian(CartesianGrid2D)
}

impl Grid2D {
    /// Shorthand for creating a cartesian grid
    pub fn new_cartesian(grid_x: Grid1D, grid_y: Grid1D,) -> Self {
        Grid2D::Cartesian(CartesianGrid2D { grid_x, grid_y })
    }

    /// Shorthand for getting an iterator for this grid
    pub fn grid<'a>(&'a self) -> Box<dyn Iterator<Item=(f64,f64)> + 'a> {
        #[allow(unreachable_patterns)]
        match self {
            Grid2D::Cartesian(grid) => Box::new(grid.grid()),
            _ => unimplemented!()
        }
    }
}

/// A 2D Cartesian grid, i.e. the Cartesian product of two 1D grids
#[derive(Clone)]
pub struct CartesianGrid2D {
    /// Grid in the x-direction (i.e. the first element of a point tuple)
    pub grid_x: Grid1D, 
    /// Grid in the y-direction (i.e. the second element of a point tuple)
    pub grid_y: Grid1D 
}

impl CartesianGrid2D {
    /// Get a new grid iterator with x as the fast changing axis
    pub fn grid<'a>(&'a self) -> CartesianGrid2DIterator<'a> {
        CartesianGrid2DIterator::new(&self.grid_x, &self.grid_y)
    }
}

/// Iterator over a 2D Cartesian grid
pub struct CartesianGrid2DIterator<'a> {
    grid_x: &'a Grid1D,
    y_iter: Box<dyn Iterator<Item=f64>>,
    x_iter: Box<dyn Iterator<Item=f64>>,
    curr_y: Option<f64>
}

impl<'a> CartesianGrid2DIterator<'a> {
    fn new(grid_x: &'a Grid1D, grid_y: &Grid1D) -> Self {
        let mut y_iter = grid_y.grid();
        let curr_y = y_iter.next();
        Self {
            grid_x,
            y_iter,
            x_iter: grid_x.grid(),
            curr_y
        }
    }
}

impl Iterator for CartesianGrid2DIterator<'_> {
    type Item = (f64,f64);

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr_y {
            Some(curr_y) => {
                let next_x = self.x_iter.next();
                match next_x {
                    Some(x) => Some((x, curr_y)),
                    None => {
                        let next_y = self.y_iter.next();
                        match next_y {
                            Some(y) => {
                                self.curr_y = Some(y);
                                self.x_iter = self.grid_x.grid();
                                let next_x = self.x_iter.next();
                                match next_x {
                                    Some(x) => Some((x,y)),
                                    None => None
                                }
                            },
                            None => None
                        }
                    }
                }
            },
            None => None
        }

    }
}

/// State of spatially resolved SIR models in two dimensions (SIR PDEs)
/// 
/// We store all 2D data in 1D arrays internally
#[allow(non_snake_case)]
#[derive(Clone)]
pub struct SIRStateSpatial2D {
    /// Susceptible population for all grid points
    pub S: Vec<f64>,
    /// Infected population for all grid points
    pub I: Vec<f64>,
    /// Recovered population for all grid points
    pub R: Vec<f64>,
    /// Spatial grid
    pub grid: Grid2D
}

impl SIRStateSpatial2D {
    /// Create a new state for a spatial SIR model in 2D.
    ///
    /// The state will be initialized on the given grid by calling `initfunc(x,y)`
    /// at each grid point `(x,y)`.
    #[allow(non_snake_case)]
    pub fn new<F>(grid: Grid2D, initfunc: F) -> Self
        where F: Fn(f64,f64) -> (f64,f64,f64)
    {
        let mut S = vec![];
        let mut I = vec![];
        let mut R = vec![];
        for (x,y) in grid.grid() {
            let result = initfunc(x,y);
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

/// Borrowed view of a [SIRStateSpatial2D]
#[allow(non_snake_case)]
pub struct SIRStateSpatial2DBorrowed<'a> {
    pub S: &'a[f64],
    pub I: &'a[f64],
    pub R: &'a[f64],
    pub grid: &'a Grid2D
}

impl<'a> SIRStateSpatial2DBorrowed<'a> {
    /// Create a `SIRStateSpatial2DBorrowed` from a contiguous vector of `f64`
    /// values (first all S values, then all I values, finally all R values;
    /// each are split into `ny` rows of length `nx`)
    #[allow(non_snake_case)]
    pub(crate) fn from_vec(v: &'a Vec<f64>, grid: &'a Grid2D) -> Self {
        let n = v.len() / 3;
        let (S,IR) = v.split_at(n);
        let (I,R) = IR.split_at(n);
        Self {
            S,I,R,grid
        }
    }

    pub fn to_owned(&self) -> SIRStateSpatial2D {
        SIRStateSpatial2D {
            S: self.S.to_owned(),
            I: self.I.to_owned(),
            R: self.R.to_owned(),
            grid: self.grid.clone()
        }
    }
}
