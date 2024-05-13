// Example of the SZ DDFT model in 2D with plotters output

use std::process::ExitCode;

use sir_ddft::ode::{ExplicitODESolver, RKF45Solver};
use sir_ddft::{
    Grid1D, Grid2D, SZDDFTParameters, SZDiffusionParameters, SZParameters, SZStateSpatial2D,
    SZDDFT2DIVP,
};

mod common;

#[allow(non_snake_case)]
pub fn main() -> ExitCode {
    // Simulation length / resolution parameters
    const DT: f64 = 0.1;
    const STEPS: u64 = 300;
    const GRIDPOINTS: usize = 512;
    const DOMAINSIZE: f64 = 10.0;
    const DX: f64 = DOMAINSIZE / GRIDPOINTS as f64;
    // Other parameters
    const NUM_THREADS: usize = 4;
    // Setup initial state
    let grid = Grid2D::new_cartesian(
        Grid1D::new_equidistant((0., DOMAINSIZE), GRIDPOINTS),
        Grid1D::new_equidistant((0., DOMAINSIZE), GRIDPOINTS),
    );
    const VARIANCE: f64 = 75.;
    const MEAN_POPULATION: f64 = 0.25;
    let gaussian = |x: f64, y: f64| {
        (-1. / (2. * DOMAINSIZE * DOMAINSIZE / VARIANCE)
            * ((x - DOMAINSIZE / 2.).powi(2) + (y - DOMAINSIZE / 2.).powi(2)))
        .exp()
    };
    let gaussian_mean =
        grid.grid().map(|(x, y)| gaussian(x, y)).sum::<f64>() * DX * DX / (DOMAINSIZE * DOMAINSIZE);
    let state = SZStateSpatial2D::new(grid.clone(), |x, y| {
        // Normalize gaussian based on finite domain
        let gaussian_normalized = gaussian(x, y) / gaussian_mean * MEAN_POPULATION;
        (0.999 * gaussian_normalized, 0.001 * gaussian_normalized)
    });
    // Setup parameters
    let sz_params = SZParameters::new(5.5, 4.5);
    let diff_params = SZDiffusionParameters::new(0.01, 0.005);
    let ddft_params = SZDDFTParameters::new(1.0, 1.0, -300.0, 100.0, -100.0, 100.0);
    // Create the IVP and solver
    let mut ivp = SZDDFT2DIVP::new(sz_params, diff_params, ddft_params, state, NUM_THREADS);
    let mut solver = RKF45Solver::<SZDDFT2DIVP, _>::new();
    // Run solver and collect data for plotting
    let (t, state) = ivp.get_result();
    let mut result = vec![(t, state.to_owned())];
    println!("Running simulation...");
    let progress = indicatif::ProgressBar::new(STEPS);
    for _ in 1..STEPS {
        ivp.add_time(DT);
        solver.integrate(&mut ivp);
        let (t, state) = ivp.get_result();
        result.push((t, state.to_owned()));
        progress.inc(1);
    }
    progress.finish();
    // -- Graphical output --

    if let Err(e) = common::plot(
        result.as_slice(),
        Some((0., DOMAINSIZE)),
        "SZ-DDFT model in 2D",
        "sz_ddft_2d",
    ) {
        println!("Error: {}", e);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
