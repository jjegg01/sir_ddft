#!/usr/bin/env python3

import argparse
import os.path
import csv

import numpy as np
import matplotlib as mpl
mpl.rcParams['mathtext.fontset'] = 'cm'
mpl.rcParams['font.family'] = 'STIXGeneral'
mpl.rcParams['text.usetex'] = True
mpl.rcParams['text.latex.preamble'] = r"\usepackage{amsmath}"
mpl.rcParams['xtick.labelsize'] = '11'
mpl.rcParams['ytick.labelsize'] = '11'
mpl.rcParams['axes.titlesize'] = '11'
mpl.rcParams['font.size'] = '11'
import matplotlib.colors as colors
import matplotlib.pyplot as plt
import matplotlib.ticker as ticker
from matplotlib.lines import Line2D

parser = argparse.ArgumentParser()
parser.add_argument("jobfile")
parser.add_argument("datadir")
parser.add_argument("outfile")

args = parser.parse_args()

# Read scan parameters
jobs_parameters = {}
with open(args.jobfile, newline='') as csvfile:
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

# Build phasediagram buffers
kill_rates = set()
fear_amplitudes = set()

for (kill, fear) in jobs_parameters.values():
    kill_rates.add(kill)
    fear_amplitudes.add(fear)

kill_rates = np.asarray(sorted(kill_rates))
fear_amplitudes = np.asarray(sorted(fear_amplitudes))

pop_initial = np.full((len(kill_rates), len(fear_amplitudes)), np.nan)

S_final = np.full((len(kill_rates), len(fear_amplitudes)), np.nan)
Z_final = np.full((len(kill_rates), len(fear_amplitudes)), np.nan)
t_end   = np.full((len(kill_rates), len(fear_amplitudes)), np.nan)

# Threshold values for survivors / zombies: if the population
# fraction of either of these falls below the threshold, the 
# zombie epidemic is considered to be over
THRESHOLD_SURVIVORS = 5e-2
THRESHOLD_ZOMBIES   = 5e-5

# Fill phasediagram buffers
for job_id in jobs_parameters:
    datafile = os.path.join(args.datadir, f"{job_id}.npz")
    if os.path.exists(datafile):
        # Read simulation data
        data = np.load(datafile)
        # Find closest parameter combination (to avoid floating point errors!)
        kill_index = np.argmin(np.abs(kill_rates - data["kill_parameter"])) 
        fear_index = np.argmin(np.abs(fear_amplitudes - data["fear_amplitude"]))
        # Check that we have not seen this combination before
        assert(np.isnan(S_final[kill_index, fear_index]))
        assert(np.isnan(Z_final[kill_index, fear_index]))
        # Calculate initial and final (i.e. t -> oo) populations
        current_pop_initial = data["S"][0] + data["Z"][0]
        pop_initial[kill_index, fear_index] = current_pop_initial
        S_final[kill_index, fear_index] = data["S"][-1]
        Z_final[kill_index, fear_index] = data["Z"][-1]
        # Determine time at which the epidemic "ends"
        S = data["S"]
        Z = data["Z"]
        t = data["t"]
        end_idx = None
        if S[-1] / current_pop_initial > THRESHOLD_SURVIVORS and Z[-1] / current_pop_initial > THRESHOLD_ZOMBIES:
            end_idx = np.infty
            print(f"No convergence for job {job_id}")
        else:
            if S[-1] / current_pop_initial < THRESHOLD_SURVIVORS:
                target_field = S
                THRESHOLD = THRESHOLD_SURVIVORS
            if Z[-1] / current_pop_initial < THRESHOLD_ZOMBIES:
                target_field = Z
                THRESHOLD = THRESHOLD_ZOMBIES
            i = len(t)-2
            while i >= 0:
                if target_field[i] / current_pop_initial > THRESHOLD:
                    end_idx = i+1
                    break
                i = i-1
            assert(end_idx != None)
        t_end[kill_index, fear_index] = t[end_idx] if not np.isinf(end_idx) else np.infty

# Normalize relative to initial population
S_final=S_final/pop_initial
Z_final=Z_final/pop_initial

# For prettier labels
def try_to_int(x):
    return int(x) if abs(int(x) - x) < 1e-10 else x

# -- Plotting --
fig, (ax,ax2,ax3) = plt.subplots(ncols=3, sharey=True, figsize=(7,2.4))

norm = colors.LogNorm(vmin=0.0001, vmax=2000.0, clip=True)
aspect = (len(kill_rates)-3) / (len(fear_amplitudes)-2)
cax  = ax.imshow(S_final.T, cmap="inferno", vmin=0, vmax=1, aspect=aspect, origin="lower", interpolation="none")
cax2 = ax2.imshow(Z_final.T, cmap="inferno", vmin=0, vmax=1, aspect=aspect, origin="lower", interpolation="none")
# Colormap hack: clip extremely large values to infinity, so they don't show up
t_end[t_end > 40] = np.infty
cax3 = ax3.imshow(t_end.T, cmap="inferno", vmin=0, vmax=40, aspect=aspect, origin="lower", interpolation="none")

xticks = [0,10,20,30]
xticklabels = [f"${try_to_int(kill_rates[i])}$" for i in xticks]
yticks = [1,6,11,16]
yticklabels = [f"${try_to_int(fear_amplitudes[i])}$" for i in yticks]

for myax in [ax,ax2,ax3]:
    myax.set_xticks(xticks)
    myax.set_xticklabels(xticklabels)
    myax.set_xlabel(r"$\kappa$", labelpad=0)
    myax.set_xlim((0, len(kill_rates)-3))
    myax.set_ylim((1, len(fear_amplitudes)-1))

    myax.set_yticks(yticks)
    myax.set_yticklabels(yticklabels)
    if myax is ax:
        myax.set_ylabel(r"$C_\text{{sz}}$", labelpad=0)

ax.text(0.95, 0.05, r"$\overline{S}_\infty / N_0$",
    color="black", ha="right", va="bottom", transform=ax.transAxes)
ax.text(-0.09,1.132, r"$\boldsymbol{\mathrm{a}}$",
    ha="center", va="bottom", transform=ax.transAxes)
ax2.text(0.95, 0.05, r"$\overline{Z}_\infty / N_0$",
    color="white", ha="right", va="bottom", transform=ax2.transAxes)
ax2.text(-0.09,1.132, r"$\boldsymbol{\mathrm{b}}$",
    ha="center", va="bottom", transform=ax2.transAxes)
ax3.text(0.95, 0.05, r"$t_\text{end} / $h",
    color="white", ha="right", va="bottom", transform=ax3.transAxes)
ax3.text(-0.09,1.132, r"$\boldsymbol{\mathrm{c}}$",
    ha="center", va="bottom", transform=ax3.transAxes)

cbar = fig.colorbar(cax, fraction=0.0492, pad=0.03, ticks=[0,0.25,0.5,0.75,1], location="top")
cbar.ax.set_xticklabels(["0", "0.25", "0.5", "0.75", "1"])
cbar2 = fig.colorbar(cax2, fraction=0.0492, pad=0.03, ticks=[0,0.25,0.5,0.75,1], location="top")
cbar2.ax.set_xticklabels(["0", "0.25", "0.5", "0.75", "1"])
cbar3 = fig.colorbar(cax3,  fraction=0.0492, pad=0.03, ticks=[0,10,20,30,40], location="top")

for mycbar in [cbar, cbar2, cbar3]:
    mycbar.ax.tick_params("x", pad=2.0)

fig.savefig(args.outfile, bbox_inches="tight", pad_inches=0)
