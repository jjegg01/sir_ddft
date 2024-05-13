#!/usr/bin/env python3

# Perform a parameter scan for the SZ-DDFT model

import numpy as np
from numpy import pi, sqrt

import joblib

import os, os.path
import argparse
import sys
import csv

parser = argparse.ArgumentParser(description = "Perform a parameter scan in fear amplitude and kill rate of the SZ-DDFT model")
parser.add_argument("jobfile")
parser.add_argument("outdir")
parser.add_argument("--jobs", type=int, default=4)
parser.add_argument("--threads-per-job", type=int, default=2)

args = parser.parse_args()

# Read parameter map
jobs_parameters = {}
with open('phasediagram_jobs.csv', newline='') as csvfile:
    reader = csv.reader(csvfile, delimiter=',')
    reader.__next__()
    for row in reader:
        job_id, kill, fear = row
        job_id = int(job_id)
        kill = float(kill)
        fear = float(fear)
        assert(not (kill, fear) in jobs_parameters.values())
        assert(not job_id in jobs_parameters)
        jobs_parameters[job_id] = (kill, fear)

def job_result_path(job_id):
    return os.path.join(args.outdir, f"{job_id}.npz")

def run_job(job_id, kill_rate, fear_amplitude):

    # Imports
    import sir_ddft
    import numpy as np
    import os.path
    from numpy import pi, sqrt

    if os.path.exists(job_result_path(job_id)):
        print(f"Skipping job {job_id} (data already present)")
    else:
        print(f"Running job {job_id}...")

    # Simulation parameters
    L = 10
    N = 256
    MEAN_DENSITY = 0.25
    initial_variance = L**2/75
    params = sir_ddft.SZParameters(5.5,kill_rate)
    diff_params = sir_ddft.SZDiffusionParameters(0.01,0.005)
    ddft_params = sir_ddft.SZDDFTParameters(1.0,1.0,fear_amplitude,100,-100,100)

    DT = 0.05        # Time step size
    MIN_TIME = 5     # Minimum simulation time
    TIMEOUT = 2000   # Maximum simulation time
    THRESHOLD = 5e-4 # The simulation stops early, if the total population fraction of S or Z falls below this value

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
    solver = sir_ddft.SZDDFT2DSolver(params, diff_params, ddft_params, state, args.threads_per_job)

    # Initialize result vectors
    t = []
    S = []
    Z = []

    def store_result(result):
        t.append(result["time"])
        S.append(np.sum(result["S"]))
        Z.append(np.sum(result["Z"]))
    store_result(solver.get_result())

    # Run until there is either no zombies or no survivors left
    while t[-1] < MIN_TIME or (
        Z[-1] / (N*N) / MEAN_DENSITY > THRESHOLD and
        S[-1] / (N*N) / MEAN_DENSITY > THRESHOLD and
        t[-1] < TIMEOUT):
        # print(f"t={t[-1]}  Z={Z[-1] / (N*N)}  S={S[-1] / (N*N)}")
        solver.add_time(DT)
        solver.integrate()
        store_result(solver.get_result())

    # Save result
    np.savez(job_result_path(job_id), t=t, S=S, Z=Z, kill_parameter=kill_rate, fear_amplitude=fear_amplitude)
    print(f"Finished job {job_id}!")

jobs = []
for job_id in jobs_parameters:
    if not os.path.exists(job_result_path(job_id)):
        kill, fear = jobs_parameters[job_id]
        jobs.append((job_id, kill, fear))

if len(jobs) == 0:
    print("Nothing to do.")

joblib.Parallel(n_jobs=args.jobs)(joblib.delayed(run_job)(job_id, kill, fear) for (job_id, kill, fear) in jobs)