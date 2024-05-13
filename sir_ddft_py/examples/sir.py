#!/usr/bin/env python3

import numpy as np

import os, os.path
import sir_ddft
import plot
from common import run_sim

# Simulation time parameters
DT = 0.3
STEPS = 100

# Initialize model parameters, state and solver
params = sir_ddft.SIRParameters(1.0,0.2,0.0)
state  = sir_ddft.SIRState(0.999,0.001,0.0)
solver = sir_ddft.SIRSolver(params, state)

# Run simulation (see common.py)
run_sim(solver, DT, STEPS, "SIR model", "sir")