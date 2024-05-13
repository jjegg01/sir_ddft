#!/usr/bin/env python3

import numpy as np
from numpy import pi, sqrt

import os, os.path
import sir_ddft

from common import run_sim

# Simulation time parameters
DT = 0.1
STEPS = 300

# Spatial simulation parameters (per dimension)
L = 10.0
GRIDPOINTS = 512

# Initial state:
# - Gaussian distribution with variance below and normalized to achieve a given
#   mean population density
# - Initially, 0.1 percent of the total population is infected, the rest is
#   susceptible
VARIANCE = L*L/50
MEAN_DENSITY = 0.3543165399952919

def gaussian(x,y):
    return np.exp(-0.5 / VARIANCE * ((x - L/2)**2 + (y - L/2)**2))
X,Y = np.meshgrid(np.linspace(0,L,GRIDPOINTS), np.linspace(0,L,GRIDPOINTS))
dx = L/(GRIDPOINTS-1)
norm_factor = MEAN_DENSITY / (np.sum(gaussian(X,Y)) * dx * dx / (L * L))
def initfunc(x,y):
    total_pop = norm_factor * gaussian(x,y)
    return [0.999 * total_pop, 0.001 * total_pop, 0.0]

# Initialize model parameters, state and solver
params = sir_ddft.SIRParameters(1.0, 0.1, 0.0)
diff_params = sir_ddft.SIRDiffusionParameters(0.01, 0.01, 0.01)
ddft_params = sir_ddft.SIRDDFTParameters(1.0, 1.0, 1.0, -10, 100, -30, 100)
grid = sir_ddft.Grid2D.new_equidistant(0, L, 0, L, GRIDPOINTS, GRIDPOINTS)
state  = sir_ddft.SIRStateSpatial2D(grid, initfunc)
solver = sir_ddft.SIRDDFT2DSolver(params, diff_params, ddft_params, state, 6)

# Run simulation (see common.py)
run_sim(solver, DT, STEPS, "SIR DDFT model", "sir_ddft_2d", [0,L])