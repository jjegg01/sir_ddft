/// Profiler friendly version of sir_ddft_2d

use sir_ddft::ode::{RKF45Solver, ExplicitODESolver};
use sir_ddft::{SIRParameters, SIRDiffusionParameters, SIRDDFTParameters,
    Grid2D, Grid1D, SIRStateSpatial2D, SIRDDFT2DIVP};

#[allow(non_snake_case)]
pub fn main() {
    const NUM_THREADS: usize = 4;
    let L = 10.0;
    let N = 512;
    let dx = L / N as f64;
    // Setup parameters and initial state
    let sir_params = SIRParameters::new(1.0, 0.1, 0.0);
    let diff_params = SIRDiffusionParameters::new(0.01, 0.01, 0.01);
    let ddft_params = SIRDDFTParameters::new(1.0, 1.0, 1.0, -10.0, 100.0, -30.0, 100.0);
    let grid = Grid2D::new_cartesian(
        Grid1D::new_equidistant((0.,L), N),
        Grid1D::new_equidistant((0.,L), N),
    );
    let S_mean = grid.grid()
        .map(|(x,y)| (-1.0/(L*2.0*L/50.0) * ((x-L/2.0).powi(2) + (y-L/2.0).powi(2))).exp())
        .sum::<f64>();
    let state = SIRStateSpatial2D::new(grid.clone(), |x,y| {
        let mut S = (-1.0/(2.0*L*L/50.0) * ((x-L/2.0).powi(2) + (y-L/2.0).powi(2))).exp();
        S = S / (dx*dx * S_mean / (L*L)) * 0.3543165399952919;
        let I = 0.001 * S;
        S = S - I;
        (S,I,0.0)
    });
    // Create the IVP and solver
    let mut ivp = SIRDDFT2DIVP::new(sir_params, diff_params, ddft_params, state, NUM_THREADS);
    let mut solver = RKF45Solver::<SIRDDFT2DIVP,_>::new();
    // Run solver and collect data for plotting
    let (t,state) = ivp.get_result();
    let mut states = vec![state.I.to_vec()];
    let mut times = vec![t];
    const FRAMES: usize = 3;
    for _ in 1..FRAMES {
        ivp.add_time(0.1);
        solver.integrate(&mut ivp);
        let (t,state) = ivp.get_result();
        states.push(state.I.to_vec());
        times.push(t);
        dbg!(t);
    }
}