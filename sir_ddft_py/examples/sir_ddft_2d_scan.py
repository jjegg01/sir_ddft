#!/usr/bin/env python3

# Perform a parameter scan for the SIR-DDFT model (see Fig. 1a in the original
# publication) using DisPy

# MAKE SURE THAT sir_ddft IS IN YOUR PYTHONPATH ON THE DISPY SERVER

import numpy as np
from numpy import pi, sqrt

import dispy

import os, os.path
import argparse
import sys

parser = argparse.ArgumentParser(description = "Perform a parameter scan in C_si and C_si/C_sd of the SIR-DDFT model")
parser.add_argument("outdir")

args = parser.parse_args()

# Scan parameters
C_si = np.linspace(-30,-0, 31)
C_rel = np.linspace(1,3,21)

def sample(C_si, C_rel):
    # Imports
    import sir_ddft
    import numpy as np
    from numpy import pi, sqrt
    from dispy.dispynode import dispynode_logger

    # Constants
    NUM_THREADS = 2
    DT = 0.1
    MIN_TIME = 10
    TIMEOUT = 400
    THRESHOLD = 1e-4

    # Use model parameters from original publication
    C_sd = C_si / C_rel
    L = 10
    N = 512 # Powers of 2 are favorable for the internal FFT
    dx = L/N
    params = sir_ddft.SIRParameters(1.0,0.1)
    diff_params = sir_ddft.SIRDiffusionParameters(0.01,0.01,0.01)
    ddft_params = sir_ddft.SIRDDFTParameters(1.0,1.0,1.0,C_sd,100,C_si,100)
    x = np.linspace(0, L, N, endpoint=False) + dx/2
    x,y = np.meshgrid(x,x)
    S = lambda x,y: np.exp(-1/(L*2.0*L/50.0)*((x-L/2)**2 + (y-L/2)**2))
    norm_fac = N**2 / np.sum(S(x,y)) * sqrt(pi) / 5
    S = lambda x,y,S=S: S(x,y) * norm_fac
    I = lambda x,y: 0.001*S(x,y)
    total_SIR = np.sum(S(x,y)+I(x,y)) * dx*dx # Total population

    # Create initial state and solver
    grid = sir_ddft.Grid2D.new_equidistant(0,L,0,L,N,N)
    state  = sir_ddft.SIRStateSpatial2D(grid, lambda x,y: [S(x,y),I(x,y),0.0])
    solver = sir_ddft.SIRDDFT2DSolver(params, diff_params, ddft_params, state, NUM_THREADS)

    t = []
    S = []
    I = []
    R = []

    def store_result(result):
        t.append(result["time"])
        S.append(np.sum(result["S"]) * dx*dx)
        I.append(np.sum(result["I"]) * dx*dx)
        R.append(np.sum(result["R"]) * dx*dx)
    store_result(solver.get_result())

    while t[-1] < MIN_TIME or (I[-1] / total_SIR > THRESHOLD and t[-1] < TIMEOUT):
        solver.add_time(DT)
        solver.integrate()
        store_result(solver.get_result())
        dispynode_logger.info(t[-1])

    return (t,S,I,R)

cluster = dispy.JobCluster(sample)
jobs = []
parameter_table = []

for (i,c_si) in enumerate(C_si):
    for (j,c_rel) in enumerate(C_rel):
        job = cluster.submit(c_si, c_rel)
        job.id = i*C_rel.shape[0] + j
        jobs.append(job)
        parameter_table.append((c_si, c_rel))

np.save(os.path.join(args.outdir, "parameter_table"), parameter_table)
for job in jobs:
    job()
    if job.exception:
        print(f"Job {job.id} failed with exception")
        print(job.exception)
    else:
        t,S,I,R = job.result
        np.savez_compressed(os.path.join(args.outdir, f"run_{job.id}"), t=t, S=S, I=I, R=R)

