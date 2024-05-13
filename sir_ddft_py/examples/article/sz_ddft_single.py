#!/usr/bin/env python3

import argparse

import numpy as np
from numpy import pi, sqrt

import os, os.path
import sys
import sir_ddft

parser = argparse.ArgumentParser()
parser.add_argument("--kill-rate", type=float, default=4.5)
parser.add_argument("--fear-amplitude", type=float, default=-300)
parser.add_argument("--hunger-amplitude", type=float, default=-100)
parser.add_argument("outfile")

args = parser.parse_args()

# Simulation parameters
L = 10
N = 512
MEAN_DENSITY = 0.25
initial_variance = L**2/75
params = sir_ddft.SZParameters(5.5,args.kill_rate)
diff_params = sir_ddft.SZDiffusionParameters(0.01,0.005)
ddft_params = sir_ddft.SZDDFTParameters(1.0,1.0,args.fear_amplitude,100,args.hunger_amplitude,100)

TOTAL_TIME = 20 # Simulate twenty days of the zombie apocalypse
DUMP_STEP = 0.05 # Interval for dumping the simulation state
NUM_STEPS = round(TOTAL_TIME / DUMP_STEP)

NTHREADS = 6 # number of threads to use

# Initial distribution (Gaussian)
grid = sir_ddft.Grid2D.new_equidistant(0,L,0,L,N,N)
S_nonorm = lambda x,y: np.exp(-1/(2.0*initial_variance)*((x-L/2)**2 + (y-L/2)**2))
X,Y = np.meshgrid(np.linspace(0,L,N), np.linspace(0,L,N))
dx = L/(N-1)
norm_factor = MEAN_DENSITY / (np.sum(S_nonorm(X,Y)) * dx * dx / (L * L))
S = lambda x,y: 0.999 * norm_factor * S_nonorm(x,y)
Z = lambda x,y: 0.001*S(x,y)

# Create solver
state  = sir_ddft.SZStateSpatial2D(grid, lambda x,y: [S(x,y),Z(x,y)])
solver = sir_ddft.SZDDFT2DSolver(params, diff_params, ddft_params, state, NTHREADS)

# Initialize result vectors
t = []
S = []
Z = []

def store_result(result):
    t.append(result["time"])
    S.append(result["S"])
    Z.append(result["Z"])

store_result(solver.get_result())
for i in range(NUM_STEPS):
    print(f"{i}/{NUM_STEPS}")
    solver.add_time(DUMP_STEP)
    solver.integrate()
    store_result(solver.get_result())

t = np.asarray(t)
S = np.asarray(S)
Z = np.asarray(Z)

with open(args.outfile, "wb") as f:
    np.savez(f, t=t, S=S, Z=Z,
        kill_rate=args.kill_rate,
        fear_amplitude=args.fear_amplitude,
        hunger_amplitude=args.hunger_amplitude,
        L=L
    )
