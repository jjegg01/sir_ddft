// Example of the SIR model with diffusion with plotters output

mod common;

use std::f64::consts::PI;
use std::process::ExitCode;

use sir_ddft::ode::{RKF45Solver, ExplicitODESolver};
use sir_ddft::{SIRParameters, SIRDiffusionParameters, Grid1D, SIRStateSpatial1D, SIRDiffusion1DIVP};

pub fn main() -> ExitCode {
    // Simulation length parameters
    const DT: f64 = 0.1;
    const STEPS: u64 = 400;
    const DOMAINSIZE: f64 = 1.0;
    const GRIDPOINTS: usize = 256;

    // Setup parameters and initial state
    let sir_params = SIRParameters::new(1.0, 0.1, 0.0);
    let diff_params = SIRDiffusionParameters::new(0.01, 0.01, 0.01);
    let grid = Grid1D::new_equidistant((0.,DOMAINSIZE), GRIDPOINTS);
    let variance = (50.0 as f64).powi(-2);
    let state = SIRStateSpatial1D::new(grid, |x| {
        // 1.0, 0.1*(-(x-0.5).powi(2) * 100.).exp(), 0.
        #[allow(non_snake_case)]
        let S = (-(x-0.5).powi(2) / (2.*variance)).exp() / (2.*PI*variance).sqrt();
        ( S, 0.001*S, 0. )
    });

    // Create the IVP and solver
    let mut ivp = SIRDiffusion1DIVP::new(sir_params, diff_params, state);
    let mut solver = RKF45Solver::<SIRDiffusion1DIVP,_>::new();

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
        // Debug code to verify that the SIR diffusion model is actually conservative
        // let sum : (f64,f64,f64) = (state.S.iter().sum(), state.I.iter().sum(), state.R.iter().sum());
        // dbg!(sum.0 + sum.1 + sum.2);
    }
    progress.finish();

    // -- Graphical output --

    if let Err(e) = common::plot(result.as_slice(), Some((0.,DOMAINSIZE)), "SIR model with diffusion in 1D", "sir_diffusion_1d") {
        println!("Error: {}", e);
        ExitCode::FAILURE
    }
    else {
        ExitCode::SUCCESS
    }
}