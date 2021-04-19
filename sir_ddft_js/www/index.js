// Main demo site code
'use strict'

// Get model labels
const SIR_LUMPED = document.getElementById("radio_sir").value;
const SIR_DIFFUSION = document.getElementById("radio_sir_diffusion").value;
const SIR_DDFT = document.getElementById("radio_sir_ddft").value;

const DIM_1D = document.getElementById("radio_1d").value;
const DIM_2D = document.getElementById("radio_2d").value;

const INF_CEFF = document.getElementById("radio_inf_ceff").value;
const INF_REFF = document.getElementById("radio_inf_Reff").value;

// Plot data (global)
let plot_data = null
const PLOT_LUMPED = 1;
const PLOT_1D = 2;
const PLOT_2D = 3;

// Plot parameters (also global)
const PLOT_FIELD_SUS = 1;
const PLOT_FIELD_INF = 2;
const PLOT_FIELD_REC = 3;
let plot_field = PLOT_FIELD_SUS; // which field to plot
const PLOT_SCALE_LIN = 1;
const PLOT_SCALE_LOG = 2;
let plot_scale = PLOT_SCALE_LIN; // which scale type to use
let plot_scale_log_min = -3;     // lower cutoff for log scale

// Current worker
let current_worker = null;

function readNumber(id) {
  return Number(document.getElementById(id).value)
}

// For printing times in labels
function prettifyNumber(x, precision = 3) {
  let first_pass = x.toPrecision(precision);
  if (first_pass.search(/\./g) >= 0) {
    return first_pass.replace(/0*$/, "").replace(/\.$/, "");
  }
  else {
    return first_pass;
  }
}

// For aggregating the 2d plots into 1d sum plots
function integrate_2d(frames, dx) {
  let S = [];
  let I = [];
  let R = [];
  let sum2d = (arr) => arr.reduce((acc, line) => acc + line.reduce((acc2, x) => acc2 + x, 0), 0);
  for (let frame of frames) {
    S.push(sum2d(frame.S)*dx*dx);
    I.push(sum2d(frame.I)*dx*dx);
    R.push(sum2d(frame.R)*dx*dx);
  }
  return [S,I,R];
}

// For aggregating the 2d plots into 1d sum plots
function integrate_1d(plot_data, dx) {
  let S = [];
  let I = [];
  let R = [];
  let sum1d = (arr) => arr.reduce((acc, x) => acc + x, 0);
  for (let i=0; i<plot_data.S.length; i++) {
    S.push(sum1d(plot_data.S[i])*dx);
    I.push(sum1d(plot_data.I[i])*dx);
    R.push(sum1d(plot_data.R[i])*dx);
  }
  return [S,I,R];
}

// Function collecting all setting values from the UI
function collectSettings() {
  return {
    model: document.querySelector('input[name="sir_model"]:checked').value,
    simulation_goal: {
      end_time: readNumber("input_end_time"),
      plot_points: readNumber("input_plot_points")
    },
    initial_conditions_lumped: {
      S: readNumber("input_init_S"),
      I: readNumber("input_init_I"),
      R: readNumber("input_init_R")
    },
    initial_conditions_1d2d: {
      grid: {
        dim: document.querySelector('input[name="dimensionality"]:checked').value,
        limits: [
          readNumber("input_grid_lo"),
          readNumber("input_grid_hi"),
        ],
        grid_points: readNumber("input_grid_num")
      },
      initfunc_src: document.getElementById("input_initfunc").value
    },
    sir_parameters: {
      infectivity: readNumber("input_infectivity"),
      recovery_rate: readNumber("input_recovery")
    },
    sir_diffusion_parameters: {
      diffusivities: {
        S: readNumber("input_diff_S"),
        I: readNumber("input_diff_I"),
        R: readNumber("input_diff_R")
      }
    },
    sir_ddft_parameters: {
        mobilities: {
          S: readNumber("input_mob_S"),
          I: readNumber("input_mob_I"),
          R: readNumber("input_mob_R"),
        },
        social_distancing: {
          amplitude: readNumber("input_amp_sd"),
          range: readNumber("input_range_sd")
        },
        self_isolation: {
          amplitude: readNumber("input_amp_si"),
          range: readNumber("input_range_si")
        }
    }
  };
}

function setInput(id, num) {
  document.getElementById(id).value = num;
}

function setRadioChecked(name, val) {
  document.querySelector(`input[name='${name}'][value='${val}']`).checked = true;
}

// Note to self: Next time use a reactive framework...
function importSettings(settings) {
  setRadioChecked("sir_model", settings.model);
  setInput("input_end_time", settings.simulation_goal.end_time);
  setInput("input_plot_points", settings.simulation_goal.plot_points);
  setInput("input_init_S", settings.initial_conditions_lumped.S);
  setInput("input_init_I", settings.initial_conditions_lumped.I);
  setInput("input_init_R", settings.initial_conditions_lumped.R);
  setRadioChecked("dimensionality", settings.initial_conditions_1d2d.grid.dim);
  setInput("input_grid_lo", settings.initial_conditions_1d2d.grid.limits[0]);
  setInput("input_grid_hi", settings.initial_conditions_1d2d.grid.limits[1]);
  setInput("input_grid_num", settings.initial_conditions_1d2d.grid.grid_points);
  setInput("input_initfunc", settings.initial_conditions_1d2d.initfunc_src);
  setInput("input_infectivity", settings.sir_parameters.infectivity);
  setInput("input_recovery", settings.sir_parameters.recovery_rate);
  setInput("input_diff_S", settings.sir_diffusion_parameters.diffusivities.S);
  setInput("input_diff_I", settings.sir_diffusion_parameters.diffusivities.I);
  setInput("input_diff_R", settings.sir_diffusion_parameters.diffusivities.R);
  setInput("input_mob_S", settings.sir_ddft_parameters.mobilities.S);
  setInput("input_mob_I", settings.sir_ddft_parameters.mobilities.I);
  setInput("input_mob_R", settings.sir_ddft_parameters.mobilities.R);
  setInput("input_amp_sd", settings.sir_ddft_parameters.social_distancing.amplitude);
  setInput("input_range_sd", settings.sir_ddft_parameters.social_distancing.range);
  setInput("input_amp_si", settings.sir_ddft_parameters.self_isolation.amplitude);
  setInput("input_range_si", settings.sir_ddft_parameters.self_isolation.range);
}

function defaultSettings() {
  return JSON.parse("{\"model\":\"SIRDDFT\",\"simulation_goal\":{\"end_time\":100,\"plot_points\":400},\"initial_conditions_lumped\":{\"S\":0.999,\"I\":0.001,\"R\":0},\"initial_conditions_1d2d\":{\"grid\":{\"dim\":\"2D\",\"limits\":[0,1],\"grid_points\":64},\"initfunc_src\":\"let S = 1.0;\\nlet I = 0.1*Math.exp(-(Math.pow((x-0.5),2)+Math.pow((y-0.5),2)) * 100.);\\nlet R = 0.0;\\n\\nreturn [S,I,R];\\n            \"},\"sir_parameters\":{\"infectivity\":0.5,\"recovery_rate\":0.1},\"sir_diffusion_parameters\":{\"diffusivities\":{\"S\":0.01,\"I\":0.01,\"R\":0.01}},\"sir_ddft_parameters\":{\"mobilities\":{\"S\":1,\"I\":1,\"R\":1},\"social_distancing\":{\"amplitude\":-10,\"range\":100},\"self_isolation\":{\"amplitude\":-30,\"range\":100}}}")
}

function updatePlot() {
  // Read plot parameters
  plot_scale_log_min = readNumber("input_log_min");

  let choose_field = (s_field, i_field, r_field) => {
    switch(plot_field) {
      case PLOT_FIELD_SUS:
        return [s_field, "Susceptible"];
      case PLOT_FIELD_INF:
        return [i_field, "Infected"];
      case PLOT_FIELD_REC:
        return [r_field, "Recovered"];
    }
  }

  if(plot_data === null) {
    purgePlots();
    return;
  }
  if(plot_data.plot_type === PLOT_LUMPED) {
    plotLumped(plot_data);
  }
  else if (plot_data.plot_type === PLOT_1D) {
    // Choose one of the S,I,R datasets to plot
    let [chosen_field, title_ext] = choose_field(plot_data.S, plot_data.I, plot_data.R);
    plot1D(chosen_field, title_ext, plot_data);
    // Integral plots
    let dx = plot_data.grid_points[1] - plot_data.grid_points[0];
    let [S,I,R] = integrate_1d(plot_data, dx);
    let data = {
      S: S,
      I: I,
      R: R,
      times: plot_data.times
    }
    plotLumped(data, "plot2", "Totals over time");
    enablePlotControls();
  }
  else if (plot_data.plot_type === PLOT_2D) {
    if(plot_data.current_frame < 0)
      return;

    let [selmaxfunc, title_ext] = choose_field((f) => f.max_S, (f) => f.max_I, (f) => f.max_R);
    let max_val = Math.max.apply(null, plot_data.frames.map(selmaxfunc));

    let colorbar = {};
    let zmax = max_val;
    if(plot_scale === PLOT_SCALE_LOG) {
      colorbar = logPlotColorbar(max_val);
      zmax = 1;
    }
    // Create plot
    let layout = plot2DLayout(title_ext, plot_data);
    // If not complete: show only latest frame
    if(!plot_data.complete) {
      let frame = plot_data.frames[plot_data.current_frame];
      let [chosen_field, _] = choose_field(frame.S, frame.I, frame.R);
      if(plot_scale === PLOT_SCALE_LOG) {
        chosen_field = logPlot(chosen_field, max_val);
      }
      let trace = plot2DTrace(chosen_field, colorbar, zmax, plot_data);
      layout.title += ` at t=${prettifyNumber(frame.time)} days`;
      Plotly.react("plot", trace, layout);
    }
    else {
      let [selfieldfunc, _] = choose_field((f) => f.S, (f) => f.I, (f) => f.R);
      let frame_fields = plot_data.frames.map(selfieldfunc);
      if(plot_scale === PLOT_SCALE_LOG) {
        frame_fields = frame_fields.map((field) => logPlot(field, max_val));
      }
      let frames = frame_fields.map((field, i) => { 
        return {
          name: i, 
          data: [{ z: field }], 
          layout: { title: layout.title + ` at t=${prettifyNumber(plot_data.frames[i].time)} days` }
        } 
      });
      layout.updatemenus = [{
        x: 0.5,
        y: 1,
        yanchor: 'center',
        xanchor: 'center',
        showactive: false,
        direction: 'left',
        type: 'buttons',
        pad: {t: -120},
        buttons: [{
            method: 'animate',
            args: [null, {
              mode: 'immediate',
              fromcurrent: true,
              transition: {duration: 0},
              frame: {duration: 40, redraw: true}
            }],
            label: 'Play'
          }, {
            method: 'animate',
            args: [[null], {
              mode: 'immediate',
              transition: {duration: 0},
              frame: {duration: 0, redraw: true}
            }],
            label: 'Pause'
        }]
      }];
      let sliderSteps = [];
      for (let i = 0; i < frames.length; i++) {
        sliderSteps.push({
          method: 'animate',
          label: "",
          args: [[i], {
            mode: 'immediate',
            transition: {duration: 0},
            frame: {duration: 0, redraw: true},
          }]
        });
      }
      layout.sliders = [{
        x: 0.5,
        y: 1,
        xanchor: "center",
        yanchor: "center",
        ticklen: 0,
        minorticklen: 0,
        pad: {t: -50},
        currentvalue: {
          visible: false,
          xanchor: 'right',
        },
        steps: sliderSteps
      }]

      let trace = plot2DTrace(frame_fields[plot_data.current_frame], colorbar, zmax, plot_data);
      Plotly.newPlot("plot", trace, layout).then(() => {
        Plotly.addFrames("plot", frames).then(() => {
          Plotly.animate("plot", [plot_data.current_frame], {
            transition: { duration: 0 },
            frame: { duration: 0 }
          });
        })
      });

      // Integral plot
      let dx = plot_data.grid_points[1] - plot_data.grid_points[0];
      let [S,I,R] = integrate_2d(plot_data.frames, dx);
      let data = {
        S: S,
        I: I,
        R: R,
        times: plot_data.times
      }
      plotLumped(data, "plot2", "Totals over time");
    }
    enablePlotControls();
  }
}

// Build logarithmic field from 2D array as well as colorbar object for plotly
function logPlot(field, max_val) {
  let log_max = Math.ceil(Math.log10(max_val));
  let num_oom = log_max - plot_scale_log_min;
  let log_field = field.map((state) => state.map((x) => {
    let val = (Math.log10(x) - plot_scale_log_min) / num_oom;
    if(isNaN(val) || val < 0)
      val = 0;
    return val;
  }));
  return log_field;
}

function logPlotColorbar(max_val) {
  let log_max = Math.ceil(Math.log10(max_val));
  let num_oom = log_max - plot_scale_log_min;
  let colorbar = {
    tickmode: "array",
    tickvals: [...Array(num_oom+1).keys()].map((i) => i/(num_oom)),
    ticktext: [...Array(num_oom+1).keys()].map((i) => `10^${i+plot_scale_log_min}`)
  };
  return colorbar;
}

function plotLumped({times, S, I, R}, plot_id="plot", title="SIR model") {
  // Plot
  let trace1 = {
    x: times,
    y: S,
    mode: "lines",
    name: "Susceptible"
  }
  let trace2 = {
    x: times,
    y: I,
    mode: "lines",
    name: "Infected"
  }
  let trace3 = {
    x: times,
    y: R,
    mode: "lines",
    name: "Recovered"
  }
  let data = [trace1, trace2, trace3];
  let layout = {
    title: title,
    xaxis: {
      title: {
        text: 't in days'
      },
    },
    yaxis: {
      title: {
        text: 'Proportion of population'
      },
    },
  }
  Plotly.newPlot(plot_id, data, layout);
}

function plot1D(chosen_field, title_ext, {times, grid_points, is_ddft}) {
  // Log scale
  let colorbar = {};
  let zmax = undefined;
  let plot_field = chosen_field;
  if(plot_scale === PLOT_SCALE_LOG) {
    let max_val = Math.max.apply(null, chosen_field.map((row) => Math.max.apply(null, row)));
    plot_field = logPlot(chosen_field, max_val);
    colorbar = logPlotColorbar(max_val);
    zmax = 1.0;
  }
  colorbar.title = "Population density";
  // Plot
  let data = [{
    type: "heatmap",
    x: grid_points,
    y: times,
    z: plot_field,
    colorscale: "Hot",
    zmin: 0.0,
    zmax: zmax,
    zauto: false,
    colorbar: colorbar
  }];
  let layout = {
    title: is_ddft ? `1D SIR DDFT model - ${title_ext}` : `1D SIR model with diffusion - ${title_ext}`,
    xaxis: {
      title: {
        text: 'x'
      },
      linecolor: 'black',
      linewidth: 1,
      mirror: true
    },
    yaxis: {
      title: {
        text: 't in days'
      },
      linecolor: 'black',
      linewidth: 1,
      mirror: true
    },
  }
  Plotly.newPlot("plot", data, layout);
}

function plot2DLayout(title_ext, {is_ddft}) {
  let layout = {
    title: is_ddft ? `2D SIR DDFT model - ${title_ext}` : `2D SIR model with diffusion - ${title_ext}`,
    xaxis: {
      title: {
        text: 'x'
      },
      linecolor: 'black',
      linewidth: 1,
      mirror: true
    },
    yaxis: {
      title: {
        text: 'y'
      },
      linecolor: 'black',
      linewidth: 1,
      mirror: true,
      scaleanchor: 'x',
      scaleratio: 1
    },
    margin: {
      t: 130
    },
    width: document.getElementById("plot").clientHeight
  }
  return layout;
}

function plot2DTrace(field, colorbar, zmax, {grid_points}) {
  let data = [{
    type: "heatmap",
    x: grid_points,
    y: grid_points,
    z: field,
    colorscale: "Hot",
    zmin: 0.0,
    zmax: zmax,
    zauto: false,
    colorbar: colorbar
  }];
  return data;
}

// Purge all plot elements
function purgePlots() {
  Plotly.purge("plot");
  Plotly.purge("plot2");
  document.getElementById("plot_controls").style.display = "none";
  plot_data = null;
}
purgePlots();

// Clear plots and show progress bar with 0%
function showProgress(label) {
  document.getElementById("progress_container").style.display = "block";
  let progress = document.getElementById("progress");
  progress.style.width = "0";
  progress.innerHTML = label;
}

// Update progress bar with a given completion percentage
function updateProgress(label, completion) {
  let progress = document.getElementById("progress");
  progress.style.width = `${completion * 100}%`;
  progress.innerHTML = label;
}

function hideProgress() {
  document.getElementById("progress_container").style.display = "none";
}

// Get elements
let run_btn = document.getElementById("button_run");
let abort_btn = document.getElementById("button_abort");
let export_btn = document.getElementById("button_export");
let import_btn = document.getElementById("button_import");
let reset_btn = document.getElementById("button_reset");
let button_plot_S = document.getElementById("button_plot_S");
let button_plot_I = document.getElementById("button_plot_I");
let button_plot_R = document.getElementById("button_plot_R");
let button_plot_linscale = document.getElementById("button_plot_linscale");
let button_plot_logscale = document.getElementById("button_plot_logscale");
let range_frame_select = document.getElementById("frame_select");
let button_check_initfunc = document.getElementById("button_check_initfunc");
let button_wizard_initfunc = document.getElementById("button_wizard_initfunc");
let button_wizard_finish = document.getElementById("button_wizard_finish");
let infectivity_tooltip = document.getElementById("infectivity_tooltip");
let button_infectivity_finish = document.getElementById("button_infectivity_finish");
let button_importjson_finish = document.getElementById("button_importjson_finish");

// Enable the SIR buttons above the plots to switch between plots
function enablePlotControls() {
  document.getElementById("plot_controls").style.display = "block";
}

// Plot field buttons
button_plot_S.onclick = () => {
  plot_field = PLOT_FIELD_SUS;
  updatePlot();
}
button_plot_I.onclick = () => {
  plot_field = PLOT_FIELD_INF;
  updatePlot();
}
button_plot_R.onclick = () => {
  plot_field = PLOT_FIELD_REC;
  updatePlot();
}

// Plot scale buttons
button_plot_linscale.onclick = () => {
  plot_scale = PLOT_SCALE_LIN;
  updatePlot();
}
button_plot_logscale.onclick = () => {
  plot_scale = PLOT_SCALE_LOG;
  updatePlot();
}

// Busy state
let busy = false;
abort_btn.disabled = true;
function busyStart() {
  busy = true;
  run_btn.innerHTML = "Running..."
  run_btn.disabled = true;
  abort_btn.disabled = false;
}
function busyAbort() {
  abort_btn.disabled = true;
  run_btn.disabled = true;
  run_btn.innerHTML = "Aborting..."
  busy = true;
}
function busyEnd() {
  abort_btn.disabled = true;
  run_btn.disabled = false;
  run_btn.innerHTML = "Run simulation"
  busy = false;
}

function validate_initfunc(settings) {
  let {initfunc_src} = settings.initial_conditions_1d2d;
  let {dim, limits} = settings.initial_conditions_1d2d.grid;
  let initfunc;
  let result_lo;
  let result_hi;
  try {
    if(dim === DIM_1D) {
      initfunc = eval(`(x) => { ${initfunc_src} }`);
      result_lo = initfunc(limits[0]);
      result_hi = initfunc(limits[1]);
    }
    else {
      initfunc = eval(`(x,y) => { ${initfunc_src} }`);
      result_lo = initfunc(limits[0], limits[0]);
      result_hi = initfunc(limits[1], limits[1]);
    }
  } catch (error) {
    alert(`Error while validating initializer function:\n${error}\nTry running the wizard to create a working initialization function.`);
    return false;
  }
  let checkfunc = (sir, x, y=undefined) => {
    let prefix = `Evaluating initfunc at x=${x}` + (typeof(y) !== "undefined" ? ` y=${y}: ` : ": ");
    if(!Array.isArray(sir)) {
      alert(prefix + `Expected array but got ${typeof(sir)}`);
      return false;
    }
    if(sir.length != 3) {
      alert(prefix + `Expected 3 elements for [S,I,R] but got ${sir.length}`);
      return false;
    }
    for(let i=0; i<3; i++) {
      let val = sir[i];
      if(typeof(val) != "number" || isNaN(val) || !isFinite(val)) {
        alert(prefix + `Invalid entry at index ${i}: ${val} (type: ${typeof(val)})`);
        return false;
      }
    }
    return true;
  }
  if(!checkfunc(result_lo, limits[0], dim === DIM_2D ? limits[0] : undefined))
    return false;
  if(!checkfunc(result_hi, limits[1], dim === DIM_2D ? limits[1] : undefined))
    return false;

  return true;
}

function startLumpedSim(settings) {
  let worker = new Worker("worker_sir.js");
  current_worker = worker;
  worker.onmessage = (ev) => {
    worker.terminate();
    // Update plot data
    plot_data = {
      plot_type: PLOT_LUMPED,
      ...ev.data
    }
    updatePlot();
    busyEnd();
  };
  worker.postMessage(settings);
}

function start1DSim(settings) {
  let worker = new Worker("worker_sir_1d.js");
  current_worker = worker;
  let progress = 0;
  let plot_points = settings.simulation_goal.plot_points;
  showProgress(`0 / ${plot_points}`);
  worker.onmessage = (ev) => {
    if(ev.data === "TICK") {
      progress += 1;
      updateProgress(`${progress} / ${plot_points}`, progress / plot_points);
      return;
    }
    hideProgress();
    worker.terminate();
    // Function to split coalesced states into rows for each time step
    let state_num = ev.data.state_num;
    let grid_points = ev.data.grid_points;
    let split_states = (buf) => {
      let result = Array(state_num);
      for(let i=0; i<state_num; i++) {
        result[i] = new Float64Array(buf,
          i*grid_points.length * Float64Array.BYTES_PER_ELEMENT, grid_points.length);
      }
      return result;
    }
    // Update plot data
    let {S,I,R, ...data} = ev.data;
    plot_data = {
      plot_type: PLOT_1D,
      S: split_states(S),
      I: split_states(I),
      R: split_states(R),
      ...data
    }
    updatePlot();
    busyEnd();
  };
  worker.postMessage({
    settings: settings,
    is_ddft: settings.model === SIR_DDFT
  });
}

function start2DSim(settings) {
  let worker = new Worker("worker_sir_2d.js");
  current_worker = worker;
  worker.onmessage = (ev) => {
    let data = ev.data;
    if(data.msgtype === "INIT") {
      plot_data = {
        plot_type: PLOT_2D,
        grid_points: data.grid_points,
        id_ddft: data.is_ddft,
        frames: [],
        current_frame: -1,
        complete: false
      }
      return;
    }
    if(data.msgtype === "TICK") {
      let [S,I,R] = [data.S, data.I, data.R]
        .map((field) => field.map((buf) => new Float64Array(buf)));
      let [max_S, max_I, max_R] = [S,I,R]
        .map((field) => Math.max.apply(null, field.map((row) => Math.max.apply(null, row))));
      plot_data.frames.push({
        time: data.time,
        S: S,
        I: I,
        R: R,
        max_S: max_S,
        max_I: max_I,
        max_R: max_R
      });
      plot_data.current_frame = plot_data.frames.length - 1;
      updatePlot();
      return;
    }
    plot_data.complete = true;
    worker.terminate();
    busyEnd();
    updatePlot();
  }
  worker.postMessage({
    settings: settings,
    is_ddft: settings.model === SIR_DDFT
  });
}

run_btn.onclick = () => {
  // Validate initfunc if necessary
  let settings = collectSettings();
  if(settings.model === SIR_DIFFUSION || settings.model === SIR_DDFT) {
    if(!validate_initfunc(settings))
      return;
  }
  // Set busy flag
  if(busy) {
    return;
  }
  busyStart();
  // Cleanup old plot
  purgePlots();
  // Switch to correct simulation
  if(settings.model === SIR_LUMPED) {
    startLumpedSim(settings);
  }
  else if(settings.model === SIR_DIFFUSION || settings.model === SIR_DDFT) {
    if (settings.initial_conditions_1d2d.grid.dim == DIM_1D) {
      start1DSim(settings);
    }
    else {
      start2DSim(settings);
    }
  }
}

abort_btn.onclick = () => {
  busyAbort();
  // Kill worker
  if (!(current_worker === null)) {
    current_worker.terminate();
  }
  // Try to reset state
  if(plot_data.plot_type === PLOT_2D) {
    plot_data.complete = true;
    busyEnd();
    updatePlot();
  }
  else {
    hideProgress();
    plot_data = null;
    purgePlots();
    busyEnd();
  }
}

reset_btn.onclick = () => {
  importSettings(defaultSettings());
}

// Initfunc checking and wizard

MicroModal.init();

button_check_initfunc.onclick = () => {
  if(validate_initfunc(collectSettings()))
    alert("Initializer function is valid.");
};

button_wizard_initfunc.onclick = () => {
  MicroModal.show("modal-initfunc")
}

button_wizard_finish.onclick = () => {
  let initfunc_src = "";
  let settings = collectSettings();
  let {dim, limits} = settings.initial_conditions_1d2d.grid;
  let mu = (limits[0] + limits[1]) / 2;
  let hasDistSqr = false;
  for(infix of ["S", "I", "R"]) {
    let selection = document.querySelector(`input[name="wiz_${infix}"]:checked`).value;
    if(selection === WIZ_CONST) {
      let val = document.getElementById(`input_wiz_${infix}_const`).value;
      initfunc_src += `let ${infix} = ${val};\n`
    }
    else {
      let amplitude = document.getElementById(`input_wiz_${infix}_gauss_amp`).value;
      let variance = document.getElementById(`input_wiz_${infix}_gauss_var`).value;
      if(!hasDistSqr) {
        if(dim === DIM_1D)
          initfunc_src += `let distSqr = Math.pow((x-${mu}),2);\n`;
        if(dim === DIM_2D)
          initfunc_src += `let distSqr = Math.pow((x-${mu}),2) + Math.pow((y-${mu}),2);\n`;
        hasDistSqr = true;
      }
      initfunc_src += `let ${infix} = (${amplitude}) * Math.exp(-0.5*distSqr/(${variance})) / Math.sqrt(2*Math.PI*(${variance}));\n`
    }
    initfunc_src += "\n";
  }
  initfunc_src += `return [S,I,R];`;

  document.getElementById("input_initfunc").value = initfunc_src;
  MicroModal.close("modal-initfunc");
}

// Infectivity wizard

infectivity_tooltip.onclick = () => {
  if (validate_initfunc(collectSettings()))
    MicroModal.show("modal-infectivity")
}

button_infectivity_finish.onclick = () => {
  let settings = collectSettings();
  // Get c_eff from user input
  let effective_value = readNumber("infectivity_wiz_value");
  let effective_parameter = document.querySelector('input[name="wiz_inf"]:checked').value;
  // Build grid and integrate numerically
  let gridpoints = buildGridpoints(settings);
  let dim = settings.initial_conditions_1d2d.grid.dim;
  let limits = settings.initial_conditions_1d2d.grid.limits;
  let integral, S_mean;
  if (dim == DIM_1D) {
    let initfunc = eval(`(x) => { ${settings.initial_conditions_1d2d.initfunc_src} }`);
    let SI = gridpoints.map((x) => initfunc(x).slice(0,2));
    let L = limits[1] - limits[0];
    integral = 0;
    S_mean = 0;
    let I_mean = 0;
    for (let i=0; i<SI.length; i++) {
      let S = SI[i][0];
      let I = SI[i][1];
      integral += S*I;
      S_mean += S;
      I_mean += I;
    }
    S_mean /= SI.length;
    I_mean /= SI.length;
    integral *= L / SI.length;
    integral /= S_mean * I_mean;
  }
  if (dim == DIM_2D) {
    let initfunc = eval(`(x,y) => { ${settings.initial_conditions_1d2d.initfunc_src} }`);
    let A = (limits[1] - limits[0]) * (limits[1] - limits[0]);
    integral = 0;
    S_mean = 0;
    let I_mean = 0;
    for (let i=0; i<gridpoints.length; i++) {
      for(let j=0; j<gridpoints.length; j++) {
        let x = gridpoints[i];
        let y = gridpoints[j];
        let SI = initfunc(x,y);
        let S = SI[0];
        let I = SI[1];
        integral += S*I;
        S_mean += S;
        I_mean += I;
      }
    }
    S_mean /= gridpoints.length * gridpoints.length;
    I_mean /= gridpoints.length * gridpoints.length;
    integral *= A / (gridpoints.length * gridpoints.length);
    integral /= S_mean * I_mean;
  }
  // Get c_eff
  let c_eff;
  if (effective_parameter == INF_REFF) {
    let w = settings.sir_parameters.recovery_rate;
    c_eff = effective_value * w / S_mean;
  }
  else {
    c_eff = effective_value;
  }
  // Calculate c
  let c = c_eff / integral;
  document.getElementById("input_infectivity").value = c;
  MicroModal.close("modal-infectivity")
}

// Import and export functions

export_btn.onclick = () => {
  let text = JSON.stringify(collectSettings());
  navigator.clipboard.writeText(text);
  alert("Model parameters have been copied to the clipboard");
}

import_btn.onclick = () => {
  MicroModal.show("modal-importjson")
}

button_importjson_finish.onclick = () => {
  let json = document.getElementById("input_importjson").value;
  let settings;
  try {
    settings = JSON.parse(json);
    importSettings(settings);
  } catch (error) {
    alert("Error while parsing config (maybe try to remove newlines?): " + error)
  }
  finally {
    MicroModal.close("modal-importjson")
  }
}