// Example of the SIR DDFT model in 2D with plotters output

use std::env::args;
use std::sync::{Arc, Mutex};

use plotters::prelude::*;
use gio::prelude::*;
use gtk::prelude::*;

use gtk::DrawingArea;

use sir_ddft::ode::{RKF45Solver, ExplicitODESolver};
use sir_ddft::{SIRParameters, SIRDiffusionParameters, SIRDDFTParameters,
    Grid2D, Grid1D, SIRStateSpatial2D, SIRDDFT2DIVP};

mod inferno_map;
use inferno_map::{colormap_inferno, f64_rgb_to_u8_rgb};

#[allow(non_snake_case)]
pub fn main() {
    const NUM_THREADS: usize = 4;
    let L = 10.0;
    let N = 512;
    let dx = L / N as f64;
    // Setup parameters and initial state
    let sir_params = SIRParameters::new(1.0, 0.1);
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
    let solver = RKF45Solver::<SIRDDFT2DIVP>::new();
    // Run solver and collect data for plotting
    let (t,state) = ivp.get_result();
    let mut states = vec![state.I.to_vec()];
    let mut times = vec![t];
    const FRAMES: usize = 100;
    for _ in 1..FRAMES {
        ivp.add_time(0.1);
        solver.integrate(&mut ivp);
        let (t,state) = ivp.get_result();
        states.push(state.I.to_vec());
        times.push(t);
        dbg!(t);
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
        let states = states.clone();
        let times = times.clone();
        let grid = grid.clone();

        // Drawing handler
        let state_to_draw = Arc::new(Mutex::new(0));
        let state_to_draw_clone = state_to_draw.clone();
        drawing_area.connect_draw(move |_,ctx| {
            // Create backend
            let root = CairoBackend::new(ctx, (WIDTH, HEIGHT))
                .expect("Cannot create drawing backend!").into_drawing_area();
            // Clear
            root.fill(&WHITE).unwrap();

            // Get data
            let state_to_draw = *state_to_draw.lock().unwrap();
            let state = &states[state_to_draw];
            let time = times[state_to_draw];
            // Build chart
            let mut chart = ChartBuilder::on(&root)
                .caption(format!("SIR DDFT model - number of infected at t = {:.2}", time),
                    ("sans-serif",16))
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .build_ranged(
                    0.0..L, 0.0..L)
                .unwrap();
            chart.configure_mesh().draw().expect("Drawing failed");
            // Draw one SIR plot
            let grid = match &grid {
                Grid2D::Cartesian(grid) => grid,
                #[allow(unreachable_patterns)]
                _ => unreachable!()
            };
            let points = grid.grid().collect::<Vec<(f64,f64)>>();
            let dx = match &grid.grid_x{
                Grid1D::Equidistant(grid) => grid.delta()
            };
            let dy = match &grid.grid_y{
                Grid1D::Equidistant(grid) => grid.delta()
            };
            let mut series = vec![];
            for ((x,y),val) in points.iter().zip(state.iter()) {
                let rect = [(x-dx/2., y-dy/2.),(x+dx/2., y+dy/2.)];
                let color = f64_rgb_to_u8_rgb(colormap_inferno(*val));
                series.push(Rectangle::new(
                    rect, RGBColor(color.0, color.1, color.2).filled())
                )
            }
            chart.draw_series(series).unwrap();
            // Draw next state next time
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
        // Add animation timeout
        gtk::timeout_add(1000/24, move || {
            window.queue_draw();
            let mut counter = state_to_draw_clone.lock().unwrap();
            *counter = (*counter + 1) % FRAMES;
            Continue(true)
        });
    });
    app.run(&args().collect::<Vec<_>>());
}