#!/usr/bin/env python3

import argparse
import os.path

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
mpl.use("pdf")
import matplotlib.pyplot as plt
import matplotlib.colors as colors

parser = argparse.ArgumentParser()
parser.add_argument("infile1")
parser.add_argument("infile2")
parser.add_argument("infile3")
parser.add_argument("infile4")
parser.add_argument("outfile")

args = parser.parse_args()

ts = []
Ss = []
Zs = []
L = []
fears = []
hungers = []

for filename in [args.infile1, args.infile2, args.infile3, args.infile4]:
    data = np.load(filename)
    ts.append(data["t"])
    Ss.append(data["S"])
    Zs.append(data["Z"])
    L.append(data["L"])
    fears.append(data["fear_amplitude"])
    hungers.append(data["hunger_amplitude"])
    data.close()

for l in L:
    assert(l == L[0])
L = L[0]

# 1D time plot
initial_pops = [np.sum(S[0]) + np.sum(Z[0]) for S,Z in zip(Ss, Zs)]
S_sums = [np.sum(S, (1,2))/initial_pop for S, initial_pop in zip(Ss, initial_pops)]
Z_sums = [np.sum(Z, (1,2))/initial_pop for Z, initial_pop in zip(Zs, initial_pops)]

fig, ax = plt.subplots(figsize=(7,2.5))

linestyles = ["-", "--", ":", "-."]
# spacings = ["\quad\;\;\;\:", "\;", "\quad\;\;\;\:", "\;"]
labels = [rf"$C_\text{{sz}}={int(fear)},\;C_\text{{zs}}={int(hunger)}$" for fear, hunger in zip(fears, hungers)]
for S,Z,t,linestyle,label in zip(S_sums, Z_sums, ts, linestyles, labels):
    ax.plot(t, S, linestyle, c="red")
    ax.plot(t, Z, linestyle, c="blue")
    ax.plot([0,0],[0,0], linestyle, c="grey", label=label)
ax.text(0.28,0.55,
    r"$\overline{S}/N_0$", color="red",
    ha="center", transform=ax.transAxes)
ax.text(0.11,0.12,
    r"$\overline{Z}/N_0$", color="blue",
    ha="center", transform=ax.transAxes)

ax.set_xlim((np.min(ts), np.max(ts)))
ax.set_ylim((0,1))

ax.set_xlabel("$t$/h")
# ax.set_ylabel("relative population")

def set_ticks(ticks, set_ticks, set_tick_labels):
    def try_to_int(x):
        return round(x) if abs(round(x) - x) < 1e-10 else f"{x:.1f}"

    set_ticks(ticks)
    set_tick_labels([f"{try_to_int(tick)}" for tick in ticks])

set_ticks(np.linspace(0,20,9,endpoint=True), ax.set_xticks, ax.set_xticklabels)
set_ticks(np.linspace(0,1,6,endpoint=True), ax.set_yticks, ax.set_yticklabels)

legend = ax.legend(ncols=2, framealpha=1.0, handletextpad=0.4)
legend.get_frame().set_edgecolor("black")
legend.set_zorder(-9000)
fig.savefig(args.outfile, bbox_inches="tight", pad_inches=0)