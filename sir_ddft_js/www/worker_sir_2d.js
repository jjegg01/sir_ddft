// Worker code for SIR simulation
// This is only supposed to be called once!
'use strict'

importScripts("sir_ddft_js.js");
const { SIRParameters, SIRDiffusionParameters, SIRDDFTParameters,
    SIRStateSpatial2D, Grid2D, SIRDiffusion2DSolver,
    SIRDDFT2DSolver } = wasm_bindgen;
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

    let xy_lo = settings.initial_conditions_1d2d.grid.limits[0];
    let xy_hi = settings.initial_conditions_1d2d.grid.limits[1];
    let num_grid_points = settings.initial_conditions_1d2d.grid.grid_points;
    let grid = Grid2D.new_equidistant(xy_lo, xy_hi, xy_lo, xy_hi, num_grid_points, num_grid_points);
    let initfunc = eval(`(x,y) => { ${settings.initial_conditions_1d2d.initfunc_src} }`);
    let state = new SIRStateSpatial2D(grid, initfunc);
    let solver;
    if(is_ddft) {
        solver =new SIRDDFT2DSolver(params, diff_params, ddft_params, state)
    } 
    else {
        solver = new SIRDiffusion2DSolver(params, diff_params, state)
    }
    let plot_points = settings.simulation_goal.plot_points;

    let send_result = () => {
        let result = solver.get_result();
        let S = result.S.map((arr) => arr.buffer);
        let I = result.I.map((arr) => arr.buffer);
        let R = result.R.map((arr) => arr.buffer);
        self.postMessage({
            msgtype: "TICK",
            time: result.time,
            S: S,
            I: I,
            R: R
        }, S.concat(I).concat(R));
    }

    // Perform simulation
    self.postMessage({
        msgtype: "INIT",
        grid_points: buildGridpoints(settings),
        is_ddft: is_ddft
    })
    send_result();
    let dt = settings.simulation_goal.end_time / 
        settings.simulation_goal.plot_points;
    for(let i=0; i<plot_points; i++) {
        solver.add_time(dt);
        solver.integrate();
        send_result();
    }
    self.postMessage({
        msgtype: "END",
    });
}