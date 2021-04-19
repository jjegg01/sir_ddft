/// Profiler friendly version of sir_ddft.rs

use std::default::Default;
use criterion::{criterion_group, criterion_main, Criterion};

use sir_ddft::ode::{RKF45Solver, ExplicitODESolver};
use sir_ddft::{SIRParameters, SIRDiffusionParameters, SIRDDFTParameters,
     Grid1D, Grid2D, SIRStateSpatial2D, SIRDDFT2DIVP};

fn my_benchmark(c: &mut Criterion) {
    const NUM_THREADS: usize = 4;
    // Setup parameters and initial state
    let sir_params = SIRParameters::new(1.0, 0.1);
    let diff_params = SIRDiffusionParameters::new(0.01, 0.01, 0.01);
    let ddft_params = SIRDDFTParameters::new(1.0, 1.0, 1.0, -5.0, 100.0, -10.0, 100.0);
    let grid = Grid2D::new_cartesian(
        Grid1D::new_equidistant((0.,1.), 512),
        Grid1D::new_equidistant((0.,1.), 512),
    );
    let state = SIRStateSpatial2D::new(grid.clone(), |x,y| (
        1.0, 0.1*(-((x-0.5).powi(2) + (y-0.5).powi(2)) * 100.).exp(), 0.
    ));
    // Create the IVP and solver
    let mut ivp = SIRDDFT2DIVP::new(sir_params, diff_params, ddft_params, state, NUM_THREADS);
    let solver = RKF45Solver::<SIRDDFT2DIVP>::new();

    let initial_state = ivp.clone_state();

    c.bench_function("integrate_sir_ddft_2d", |b| b.iter(|| {
        ivp.add_time(0.01);
        solver.integrate(&mut ivp);
        ivp.set_state(&initial_state);
    }));
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = my_benchmark
}
criterion_main!(benches);