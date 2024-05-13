// Example of the SIR model with plotters output

mod common;

// use std::env::args;

// use plotters::prelude::*;
// use gio::prelude::*;
// use gtk::prelude::*;

// use gtk::DrawingArea;

use std::process::ExitCode;

use sir_ddft::ode::{RKF45Solver, ExplicitODESolver};
use sir_ddft::{SIRParameters, SIRState, SIRODEIVP};

pub fn main() -> ExitCode {
    // Simulation length parameters
    const DT: f64 = 0.5;
    const STEPS: u64 = 100;
    
    // Setup parameters and initial state
    let params = SIRParameters::new(0.5, 0.1, 0.0);
    let state = SIRState::new(0.998, 0.002, 0.);

    // Create the IVP and solver
    let mut ivp = SIRODEIVP::new(params, state);
    let mut solver = RKF45Solver::<SIRODEIVP,_>::new();

    // Run solver and collect data for plotting
    let (t,state) = ivp.get_result();
    let mut result = vec![(t, *state)];
    println!("Running simulation...");
    let progress = indicatif::ProgressBar::new(STEPS);
    for _ in 1..STEPS {
        ivp.add_time(DT);
        solver.integrate(&mut ivp);
        let (t,state) = ivp.get_result();
        result.push((t, *state));
        progress.inc(1);
    }
    progress.finish();

    // -- Graphical output --

    if let Err(e) = common::plot(result.as_slice(), None, "SIR model", "sir") {
        println!("Error: {}", e);
        ExitCode::FAILURE
    }
    else {
        ExitCode::SUCCESS
    }
}