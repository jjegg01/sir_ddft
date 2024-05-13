#!/usr/bin/env python3

import numpy as np

import os, os.path
import sys
sys.path.append(os.path.abspath(os.path.join(__file__, "../../../target/release")))
import sir_ddft

from common import run_sim

# Simulation time parameters
DT = 0.25
STEPS = 400

# Spatial simulation parameters
L = 1.0
GRIDPOINTS = 256

# Initial state:
# S: normalized Gaussian with the variance below
# I: same as S, but divided by 1000
# R: constant density of 0.0
VARIANCE = 50**(-2)

def initfunc(x):
    S = np.exp(-0.5 * (x - L/2)**2 / VARIANCE) / np.sqrt(2*np.pi*VARIANCE)
    return [S, S*0.001, 0.0]

# Initialize model parameters, state and solver
params = sir_ddft.SIRParameters(0.5, 0.1, 0.0)
diff_params = sir_ddft.SIRDiffusionParameters(0.01, 0.01, 0.01)
grid = sir_ddft.Grid1D.new_equidistant(0, L, GRIDPOINTS)
state  = sir_ddft.SIRStateSpatial1D(grid, initfunc)
solver = sir_ddft.SIRDiffusion1DSolver(params, diff_params, state)

# Run simulation (see common.py)
run_sim(solver, DT, STEPS, "SIR model with diffusion", "sir_diffusion_1d", xlim=[0,L])