// Example of the SIR model with plotters output

use std::env::args;

use plotters::prelude::*;
use gio::prelude::*;
use gtk::prelude::*;

use gtk::DrawingArea;

use sir_ddft::ode::{RKF45Solver, ExplicitODESolver};
use sir_ddft::{SIRParameters, SIRState, SIRODEIVP};

pub fn main() {
    // Setup parameters and initial state
    let params = SIRParameters::new(0.5, 0.1, 0.0);
    let state = SIRState::new(0.998, 0.002, 0.);
    // Create the IVP and solver
    let mut ivp = SIRODEIVP::new(params, state);
    let mut solver = RKF45Solver::<SIRODEIVP,_>::new();
    // Run solver and collect data for plotting
    let (t,state) = ivp.get_result();
    let mut result = vec![(t, *state)];
    for _ in 1..100 {
        ivp.add_time(0.5);
        solver.integrate(&mut ivp);
        let (t,state) = ivp.get_result();
        result.push((t, *state));
    }
    // -- Graphical output --

    // Create application
    let app = gtk::Application::new(
        Some("sir_ddft.examples.sir"), 
        Default::default()
    ).expect("Cannot initialize GTK application!");

    // Startup code (UI creation)
    app.connect_activate(move |app| {
        // Create a window with a drawing area
        const WIDTH: u32 = 800;
        const HEIGHT: u32 = 800;
        let window = gtk::ApplicationWindow::new(app);
        let drawing_area = DrawingArea::new();
        // This handler is Fn, so we cannot move a captured variable out again
        // to the drawing handler
        let result = result.clone();

        // Drawing handler
        drawing_area.connect_draw(move |_,ctx| {
            // Create backend
            let root = CairoBackend::new(ctx, (WIDTH, HEIGHT))
                .expect("Cannot create drawing backend!").into_drawing_area();

            // Clear
            root.fill(&WHITE).unwrap();
            // Build chart
            let mut chart = ChartBuilder::on(&root)
                .caption("SIR model", ("sans-serif",16))
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .build_ranged(
                    result[0].0..result.last().unwrap().0, 
                    0.0..result.iter()
                        .map(|(_,state)| state.S.max(state.I.max(state.R)))
                        .fold(0., f64::max))
                .unwrap();
            chart.configure_mesh().draw().expect("Drawing failed");
            // Draw SIR series
            chart.draw_series(LineSeries::new(
                result.iter().map(|(t,state)| {(*t,state.S)}).collect::<Vec<_>>(),
                &BLUE
            )).unwrap();
            chart.draw_series(LineSeries::new(
                result.iter().map(|(t,state)| {(*t,state.I)}).collect::<Vec<_>>(),
                &RED
            )).unwrap();
            chart.draw_series(LineSeries::new(
                result.iter().map(|(t,state)| {(*t,state.R)}).collect::<Vec<_>>(),
                &GREEN
            )).unwrap();
            Inhibit(false)
        });
        // Window properties
        window.set_default_size(WIDTH as i32, HEIGHT as i32);
        window.set_title("SIR model");
        window.set_icon_name(Some("video-display"));
        // Layout
        window.add(&drawing_area);
        // Show window
        window.show_all();
    });
    app.run(&args().collect::<Vec<_>>());
}