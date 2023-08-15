// Example of the SIR DDFT model in 1D with plotters output

mod common;

use std::f64::consts::PI;
use std::process::ExitCode;

use sir_ddft::ode::{RKF45Solver, ExplicitODESolver};
use sir_ddft::{SIRParameters, SIRDiffusionParameters, SIRDDFTParameters,
     Grid1D, SIRStateSpatial1D, SIRDDFT1DIVP};

pub fn main() -> ExitCode {
    // Simulation length / resolution parameters
    const DT: f64 = 0.5;
    const STEPS: u64 = 400;
    const DOMAINSIZE: f64 = 1.0;
    const GRIDPOINTS: usize = 256;
    // Other parameters
    const NUM_THREADS: usize = 4;

    // Setup parameters and initial state
    let sir_params = SIRParameters::new(1.0, 0.1, 0.0);
    let diff_params = SIRDiffusionParameters::new(0.01, 0.01, 0.01);
    let ddft_params = SIRDDFTParameters::new(1.0, 1.0, 1.0, -5.0, 100.0, -10.0, 100.0);
    //let ddft_params = SIRDDFTParameters::new(1.0, 1.0, 1.0, 0.0, 100.0, 0.0, 100.0);
    let grid = Grid1D::new_equidistant((0.,DOMAINSIZE), GRIDPOINTS);
    let variance = (50.0 as f64).powi(-2);
    let state = SIRStateSpatial1D::new(grid, |x| {
        #[allow(non_snake_case)]
        let S = (-(x-0.5).powi(2) / (2.*variance)).exp() / (2.*PI*variance).sqrt();
        ( S, 0.001*S, 0. )
    } );

    // Create the IVP and solver
    let mut ivp = SIRDDFT1DIVP::new(sir_params, diff_params, ddft_params, state, NUM_THREADS);
    let mut solver = RKF45Solver::<SIRDDFT1DIVP,_>::new();

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
        // Debug code to verify that the SIR-DDFT model is actually conservative
        // let sum : (f64,f64,f64) = (state.S.iter().sum(), state.I.iter().sum(), state.R.iter().sum());
        // dbg!(sum.0 + sum.1 + sum.2);
    }
    progress.finish();

    // -- Graphical output --

    if let Err(e) = common::plot(result.as_slice(), Some((0.,DOMAINSIZE)), "SIR-DDFT model in 1D", "sir_ddft_1d.npz") {
        println!("Error: {}", e);
        ExitCode::FAILURE
    }
    else {
        ExitCode::SUCCESS
    }
}