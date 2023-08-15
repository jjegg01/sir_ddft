// Example of the SIR model with diffusion with plotters output

use std::process::ExitCode;

use sir_ddft::ode::{RKF45Solver, ExplicitODESolver};
use sir_ddft::{SIRParameters, SIRDiffusionParameters, Grid2D, Grid1D, SIRStateSpatial2D, SIRDiffusion2DIVP};

mod common;

#[allow(non_snake_case)]
pub fn main() -> ExitCode {
    // Simulation length / resolution parameters
    const DT: f64 = 0.1;
    const STEPS: u64 = 400;
    const GRIDPOINTS: usize = 256;
    const DOMAINSIZE: f64 = 10.0;
    const DX: f64 = DOMAINSIZE / GRIDPOINTS as f64;
    // Setup initial state
    let grid = Grid2D::new_cartesian(
        Grid1D::new_equidistant((0.,DOMAINSIZE), GRIDPOINTS),
        Grid1D::new_equidistant((0.,DOMAINSIZE), GRIDPOINTS),
    );
    const VARIANCE: f64 = 50.;
    const MEAN_POPULATION: f64 = 0.3543165399952919;
    let gaussian = |x: f64,y: f64| {
        (-1./(2. * DOMAINSIZE*DOMAINSIZE / VARIANCE) * ((x - DOMAINSIZE/2.).powi(2) + (y - DOMAINSIZE/2.).powi(2))).exp()
    };
    let gaussian_mean = grid.grid()
        .map(|(x,y)| gaussian(x,y))
        .sum::<f64>() * DX * DX / (DOMAINSIZE * DOMAINSIZE);
    let state = SIRStateSpatial2D::new(grid.clone(), |x,y| {
        // Normalize gaussian based on finite domain
        let gaussian_normalized = gaussian(x,y) / gaussian_mean * MEAN_POPULATION;
        (0.999 * gaussian_normalized, 0.001 * gaussian_normalized, 0.0)
    });
    // Setup parameters
    let sir_params = SIRParameters::new(1.0, 0.1, 0.0);
    let diff_params = SIRDiffusionParameters::new(0.01, 0.01, 0.01);
    // Create the IVP and solver
    let mut ivp = SIRDiffusion2DIVP::new(sir_params, diff_params, state);
    let mut solver = RKF45Solver::<SIRDiffusion2DIVP,_>::new();
    // Run solver and collect data for plotting
    let (t,state) = ivp.get_result();
    let mut result = vec![(t, state.to_owned())];
    println!("Running simulation...");
    let progress = indicatif::ProgressBar::new(STEPS);
    for _ in 1..STEPS {
        ivp.add_time(DT);
        solver.integrate(&mut ivp);
        let (t,state) = ivp.get_result();
        result.push((t,state.to_owned()));
        progress.inc(1);
    }
    progress.finish();

    // -- Graphical output --

    if let Err(e) = common::plot(result.as_slice(), Some((0.,DOMAINSIZE)), "SIR model with diffusion in 2D", "sir_diffusion_2d.npz") {
        println!("Error: {}", e);
        ExitCode::FAILURE
    }
    else {
        ExitCode::SUCCESS
    }
}