// Worker code for SIR simulation
// This is only supposed to be called once!
'use strict'

importScripts("sir_ddft_js.js");
const { SIRParameters, SIRState, SIRSolver } = wasm_bindgen;
importScripts("worker_common.js");

self.onmessage = async (ev) => {
    // Await WASM loading
    await wasm_bindgen("sir_ddft_js_bg.wasm");

    let settings = ev.data;
    let params = buildSIRParameters(settings);
    let state = new SIRState(
        settings.initial_conditions_lumped.S,
        settings.initial_conditions_lumped.I,
        settings.initial_conditions_lumped.R,
    );
    let solver = new SIRSolver(params, state);
    let times = []
    let result_S = [];
    let result_I = [];
    let result_R = [];
    let save_result = () => {
        let result = solver.get_result()
        times.push(result.time);
        result_S.push(result.S);
        result_I.push(result.I);
        result_R.push(result.R);
    }
    save_result();
    let dt = settings.simulation_goal.end_time / 
        settings.simulation_goal.plot_points;
    for(let i=0; i<settings.simulation_goal.plot_points; i++) {
        solver.add_time(dt);
        solver.integrate();
        save_result();
    }
    self.postMessage({
        times: times,
        S: result_S,
        I: result_I,
        R: result_R
    });
}

