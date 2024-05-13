#!/usr/bin/env python3

import numpy as np
from numpy import pi, sqrt

import os, os.path
import sys
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
# - Initially, 0.1 percent of the total population are zombies, the rest is alive
VARIANCE = L*L/75
MEAN_DENSITY = 0.25

def gaussian(x,y):
    return np.exp(-0.5 / VARIANCE * ((x - L/2)**2 + (y - L/2)**2))
X,Y = np.meshgrid(np.linspace(0,L,GRIDPOINTS), np.linspace(0,L,GRIDPOINTS))
dx = L/(GRIDPOINTS-1)
norm_factor = MEAN_DENSITY / (np.sum(gaussian(X,Y)) * dx * dx / (L * L))
def initfunc(x,y):
    total_pop = norm_factor * gaussian(x,y)
    return [0.999 * total_pop, 0.001 * total_pop]

# Initialize model parameters, state and solver
params = sir_ddft.SZParameters(5.5,4.5)
diff_params = sir_ddft.SZDiffusionParameters(0.01,0.005)
ddft_params = sir_ddft.SZDDFTParameters(1.0,1.0,-300,100,-100,100)
grid = sir_ddft.Grid2D.new_equidistant(0, L, 0, L, GRIDPOINTS, GRIDPOINTS)
state  = sir_ddft.SZStateSpatial2D(grid, initfunc)#lambda x,y: [S(x,y),Z(x,y)])
solver = sir_ddft.SZDDFT2DSolver(params, diff_params, ddft_params, state, 4)

# Run simulation (see common.py)
run_sim(solver, DT, STEPS, "SZ DDFT model", "sz_ddft_2d", [0,L])