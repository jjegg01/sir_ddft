// Worker code for SIR simulation
// This is only supposed to be called once!
'use strict'

importScripts("sir_ddft_js.js");
const { SIRParameters, SIRDiffusionParameters, SIRDDFTParameters,
    SIRStateSpatial1D, Grid1D, SIRDiffusion1DSolver,
    SIRDDFT1DSolver } = wasm_bindgen;
importScripts("worker_common.js");

self.onmessage = async (ev) => {
    // Await WASM loading
    await wasm_bindgen("sir_ddft_js_bg.wasm");

    let {settings, is_ddft} = ev.data;

    // SIR parameters
    let params = buildSIRParameters(settings);
    // Diffusion parameters
    let diff_params = buildSIRDiffusionParameters(settings);
    // DDFT parameters
    let ddft_params = buildSIRDDFTParameters(settings);

    let x_lo = settings.initial_conditions_1d2d.grid.limits[0];
    let x_hi = settings.initial_conditions_1d2d.grid.limits[1];
    let num_grid_points = settings.initial_conditions_1d2d.grid.grid_points;
    let grid = Grid1D.new_equidistant(x_lo, x_hi, num_grid_points);
    let initfunc = eval(`(x) => { ${settings.initial_conditions_1d2d.initfunc_src} }`);
    let state = new SIRStateSpatial1D(grid, initfunc);
    let solver;
    if(is_ddft) {
        solver =new SIRDDFT1DSolver(params, diff_params, ddft_params, state)
    } 
    else {
        solver = new SIRDiffusion1DSolver(params, diff_params, state)
    }
    let times = []
    let plot_points = settings.simulation_goal.plot_points; 
    let result_S = new Float64Array((plot_points + 1) * num_grid_points); // +1 for initial state
    let result_I = new Float64Array((plot_points + 1) * num_grid_points);
    let result_R = new Float64Array((plot_points + 1) * num_grid_points);
    let save_result = (index) => {
        let result = solver.get_result()
        times.push(result.time);
        result_S.set(result.S, index*num_grid_points);
        result_I.set(result.I, index*num_grid_points);
        result_R.set(result.R, index*num_grid_points);
    }
    save_result(0);
    let dt = settings.simulation_goal.end_time / 
        settings.simulation_goal.plot_points;
    for(let i=0; i<plot_points; i++) {
        solver.add_time(dt);
        solver.integrate();
        save_result(i+1);
        self.postMessage("TICK");
    }
    self.postMessage({
        times: times,
        grid_points: buildGridpoints(settings), // Array of grid points
        state_num: plot_points + 1, // Number of saved states
        is_ddft: is_ddft,
        S: result_S.buffer,
        I: result_I.buffer,
        R: result_R.buffer
    }, [result_S.buffer, result_I.buffer, result_R.buffer]);
}