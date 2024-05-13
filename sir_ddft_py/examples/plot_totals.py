#!/usr/bin/env python3

# Example plotting code for sir_ddft_2d_scan.py

import numpy as np
import matplotlib.pyplot as plt

import argparse

parser = argparse.ArgumentParser(description="Plot S,I and R totals over time")
parser.add_argument("infile")

args = parser.parse_args()

data = np.load(args.infile)
S,I,R,t = [data[label] for label in ["S", "I", "R", "t"]]

fig, ax = plt.subplots()

ax.plot(t,S, label="Susceptible")
ax.plot(t,I, label="Infected")
ax.plot(t,R, label="Recovered")

ax.legend()

plt.show()