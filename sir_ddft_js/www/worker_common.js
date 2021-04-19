function buildSIRParameters(settings) {
    let infectivity = settings.sir_parameters.infectivity;
    let recovery_rate = settings.sir_parameters.recovery_rate;
    return new SIRParameters(infectivity, recovery_rate);
}

function buildSIRDiffusionParameters(settings) {
    let diffusivity_S = settings.sir_diffusion_parameters.diffusivities.S;
    let diffusivity_I = settings.sir_diffusion_parameters.diffusivities.I;
    let diffusivity_R = settings.sir_diffusion_parameters.diffusivities.R;
    return new SIRDiffusionParameters(
        diffusivity_S, diffusivity_I, diffusivity_R);
}

function buildSIRDDFTParameters(settings) {
    let mobility_S = settings.sir_ddft_parameters.mobilities.S;
    let mobility_I = settings.sir_ddft_parameters.mobilities.I;
    let mobility_R = settings.sir_ddft_parameters.mobilities.R;
    let social_distancing_amplitude = settings.sir_ddft_parameters.social_distancing.amplitude;
    let social_distancing_range = settings.sir_ddft_parameters.social_distancing.range;
    let self_isolation_amplitude = settings.sir_ddft_parameters.self_isolation.amplitude;
    let self_isolation_range = settings.sir_ddft_parameters.self_isolation.range;
    return new SIRDDFTParameters(mobility_S, mobility_I, mobility_R,
        social_distancing_amplitude, social_distancing_range,
        self_isolation_amplitude, self_isolation_range);
}
    
function buildGridpoints(settings) {
    let num_grid_points = settings.initial_conditions_1d2d.grid.grid_points;
    let x_lo = settings.initial_conditions_1d2d.grid.limits[0];
    let x_hi = settings.initial_conditions_1d2d.grid.limits[1];

    let grid_points = Array(num_grid_points);
    let dx = (x_hi-x_lo) / (num_grid_points - 1);
    for(let i=0; i<num_grid_points; i++) {
        grid_points[i] = x_lo + i*dx;
    }
    return grid_points;
}
