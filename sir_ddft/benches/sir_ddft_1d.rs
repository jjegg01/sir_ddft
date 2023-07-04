/// Profiler friendly version of sir_ddft.rs

use std::default::Default;
use criterion::{criterion_group, criterion_main, Criterion};

use std::f64::consts::PI;

use sir_ddft::ode::{RKF45Solver, ExplicitODESolver};
use sir_ddft::{SIRParameters, SIRDiffusionParameters, SIRDDFTParameters,
     Grid1D, SIRStateSpatial1D, SIRDDFT1DIVP};

fn integrate() {
    const NUM_THREADS: usize = 4;
    // Setup parameters and initial state
    let sir_params = SIRParameters::new(1.0, 0.1);
    let diff_params = SIRDiffusionParameters::new(0.01, 0.01, 0.01);
    let ddft_params = SIRDDFTParameters::new(1.0, 1.0, 1.0, -5.0, 100.0, -10.0, 100.0);
    let grid = Grid1D::new_equidistant((0.,1.), 256);
    let variance = (50.0 as f64).powi(-2);
    let state = SIRStateSpatial1D::new(grid.clone(), |x| {
        #[allow(non_snake_case)]
        let S = (-(x-0.5).powi(2) / (2.*variance)).exp() / (2.*PI*variance).sqrt();
        ( S, 0.001*S, 0. )
    } );
    // Create the IVP and solver
    let mut ivp = SIRDDFT1DIVP::new(sir_params, diff_params, ddft_params, state, NUM_THREADS);
    let mut solver = RKF45Solver::<SIRDDFT1DIVP,_>::new();
    // Run solver and collect data for plotting
    for _ in 1..3 {
        ivp.add_time(0.5);
        solver.integrate(&mut ivp);
    }
}

fn my_benchmark(c: &mut Criterion) {
    c.bench_function("integrate_sir_ddft_1d", |b| b.iter(|| integrate()));
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = my_benchmark
}
criterion_main!(benches);