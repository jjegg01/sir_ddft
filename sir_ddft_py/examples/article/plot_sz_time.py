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

def find_lowest_divisor(N, start):
    for i in range(start, N):
        if N % i == 0:
            return i
    return N

def gen_ticks(N):
    #l = len(data) - 1
    divisor = find_lowest_divisor(N, 2)
    return np.array([N/divisor * i for i in range(divisor+1)]), divisor

def set_ticks(ax, N, y=False):
    if y:
        set_ticks = ax.set_yticks
        set_ticklabels = ax.set_yticklabels
    else:
        set_ticks = ax.set_xticks
        set_ticklabels = ax.set_xticklabels
    ticks, divisor = gen_ticks(N)
    ticks = ticks - 0.5
    set_ticks(ticks)
    set_ticklabels([f"{try_to_int(L * i / divisor)}" for i in range(len(ticks))])

def try_to_int(x):
    return int(x) if abs(int(x) - x) < 1e-10 else x

# 2D time plots

fig, axes = plt.subplots(ncols=5, nrows=4, sharey=True, sharex=True, figsize=(7,5), dpi=400)

norm = colors.LogNorm(vmin=0.001, vmax=10.0, clip=True)

for i, (axes_row, ts_row, Zs_row, Ss_row, fear, hunger) in enumerate(zip(axes, ts, Zs, Ss, fears, hungers)):
    # Plot zombies
    T = np.max(ts_row) - np.min(ts_row)
    plot_times = np.asarray([0,5,10,20])
    data_indices = [np.argmin(np.abs(ts - plot_time)) for plot_time in plot_times]
    for ax, data_index, t in zip(axes_row[:4], data_indices, plot_times):
        Z = Zs_row[data_index]
        cax = ax.imshow(Z, cmap="inferno", norm=norm, origin="lower")
        ax.text(0.83,0.05,"$Z$",color="white", transform=ax.transAxes)
        if i == 3:
            set_ticks(ax, Z.shape[0], y=False)
            ax.set_xlabel("$x$", labelpad=0)
        if i == 0:
            ax.text(0.5,1.1,f"$t={try_to_int(t)}\,$h",
                ha="center", transform=ax.transAxes)
    # Plot survivors at simulation end
    ax = axes_row[-1]
    S = Ss_row[data_indices[-1]]
    t = plot_times[-1]
    cax = ax.imshow(S, cmap="inferno", norm=norm, origin="lower")
    ax.text(0.83,0.05,"$S$",color="white", transform=ax.transAxes)
    if i == 0:
        ax.text(0.5,1.1,f"$t={try_to_int(t)}\,$h",
            ha="center", transform=ax.transAxes)
    if i == 3:
        set_ticks(ax, Z.shape[0], y=False)
        ax.set_xlabel("$x$", labelpad=0)
    if i == 0:
        label_y_off = 0.2
    else:
        label_y_off = 0.0
    ax.text(1.15,label_y_off,
        rf"$C_\text{{sz}}={int(fear)}$,",
        ha="center", va="bottom", rotation="vertical", transform=ax.transAxes)
    ax.text(1.35,label_y_off,
        rf"$C_\text{{zs}}={int(hunger)}$",
        ha="center", va="bottom", rotation="vertical", transform=ax.transAxes)
    # Fix y ticks
    ax = axes_row[0]
    set_ticks(ax, Z.shape[1], y=True)
    ax.set_ylabel("$y$", labelpad=0)
    if i < 3:
        rowlabel = "i" * (i+1)
    elif i == 3:
        rowlabel = "iv"
    ax.text(-0.49,0.9,
        rf"$\boldsymbol{{\mathrm{{{rowlabel}}}}}$",
        ha="left", va="bottom", transform=ax.transAxes)
    # ax = axes_row[-1]

cbar = fig.colorbar(cax, ax=axes, fraction=0.0365, pad=0.08)#, ticks=[0,0.5,1.0,1.5,2.0])
# cbar.ax.set_ylabel("Population")

#fig.tight_layout(h_pad=1, w_pad=0.2, pad=0, rect=(0,0.04,1,1))
fig.savefig(args.outfile, bbox_inches="tight", pad_inches=0)

# # 1D time plot
# if args.outfile2:
#     print(Ss[0].shape)
#     Ss = [np.sum(S, (1,2)) for S in Ss]
#     Is = [np.sum(I, (1,2)) for I in Is]
#     Rs = [np.sum(R, (1,2)) for R in Rs]

#     fig, ax = plt.subplots(figsize=(6,3))

#     linestyles = [":", "--", "-"]
#     for S,I,R,t,linestyle in zip(Ss, Is, Rs, ts, linestyles):
#         ax.plot(ts, S, linestyle, c="blue")
#         ax.plot(ts, I, linestyle, c="red")
#         ax.plot(ts, R, linestyle, c="green")

#     fig.savefig(args.outfile2, bbox_inches="tight", pad_inches=0)