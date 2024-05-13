#!/usr/bin/env python3

# This is a trivial script to generate figure labels in the same
# font as the rest of the figure

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
parser.add_argument("outfile")

args = parser.parse_args()

fig, ax = plt.subplots(figsize=(7,5), dpi=400)

ax.text(0.2,0.2,
        r"$\boldsymbol{\mathrm{a}}$",
        ha="left", va="bottom", transform=ax.transAxes)
ax.text(0.4,0.2,
        r"$\boldsymbol{\mathrm{b}}$",
        ha="left", va="bottom", transform=ax.transAxes)
ax.text(0.6,0.2,
        r"$\boldsymbol{\mathrm{c}}$",
        ha="left", va="bottom", transform=ax.transAxes)

fig.savefig(args.outfile, bbox_inches="tight", pad_inches=0)