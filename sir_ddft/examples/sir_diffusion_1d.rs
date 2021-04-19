// Example of the SIR model with diffusion with plotters output

use std::env::args;

use plotters::prelude::*;
use gio::prelude::*;
use gtk::prelude::*;

use gtk::DrawingArea;

use sir_ddft::ode::{RKF45Solver, ExplicitODESolver};
use sir_ddft::{SIRParameters, SIRDiffusionParameters, Grid1D, SIRStateSpatial1D, SIRDiffusion1DIVP};

mod inferno_map;
use inferno_map::{colormap_inferno, f64_rgb_to_u8_rgb};

pub fn main() {
    // Setup parameters and initial state
    let sir_params = SIRParameters::new(1.0, 0.1);
    let diff_params = SIRDiffusionParameters::new(0.01, 0.01, 0.01);
    let grid = Grid1D::new_equidistant((0.,1.), 256);
    let state = SIRStateSpatial1D::new(grid.clone(), |x| (
        1.0, 0.1*(-(x-0.5).powi(2) * 100.).exp(), 0.
    ));
    // Create the IVP and solver
    let mut ivp = SIRDiffusion1DIVP::new(sir_params, diff_params, state);
    let solver = RKF45Solver::<SIRDiffusion1DIVP>::new();
    // Run solver and collect data for plotting
    let (t,state) = ivp.get_result();
    let mut state_matrix = vec![state.I.to_vec()];
    let mut times = vec![t];
    for _ in 1..200 {
        ivp.add_time(0.2);
        solver.integrate(&mut ivp);
        let (t,state) = ivp.get_result();
        state_matrix.push(state.I.to_vec());
        times.push(t);
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
        let state_matrix = state_matrix.clone();
        let times = times.clone();
        let grid = grid.clone();

        // Drawing handler
        drawing_area.connect_draw(move |_,ctx| {
            // Create backend
            let root = CairoBackend::new(ctx, (WIDTH, HEIGHT))
                .expect("Cannot create drawing backend!").into_drawing_area();

            // Clear
            root.fill(&WHITE).unwrap();
            // Build chart
            let mut chart = ChartBuilder::on(&root)
                .caption("SIR model with diffusion - number of infected", ("sans-serif",16))
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .build_ranged(
                    0.0..1.0, times[0]..*times.last().unwrap())
                .unwrap();
            chart.configure_mesh().draw().expect("Drawing failed");
            // Draw SIR series
            let x = match &grid {
                Grid1D::Equidistant(grid) => grid.grid().collect::<Vec<f64>>(),
                #[allow(unreachable_patterns)]
                _ => unreachable!()
            };
            let dx = x[1] - x[0];
            let dt = times[1] - times[0];
            let mut series = vec![];
            for (t,state) in times.iter().zip(state_matrix.iter()) {
                for (x,val) in x.iter().zip(state.iter()) {
                    let rect = [(x-dx/2., t-dt/2.),(x+dx/2., t+dt/2.)];
                    let color = f64_rgb_to_u8_rgb(colormap_inferno(*val));
                    series.push(Rectangle::new(
                        rect, RGBColor(color.0, color.1, color.2).filled())
                    )
                }
            }
            chart.draw_series(series).unwrap();
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