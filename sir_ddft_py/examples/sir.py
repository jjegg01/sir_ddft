import numpy as np
import matplotlib.pyplot as plt
import matplotlib as mpl

import os, os.path
import sys
sys.path.append(os.path.abspath(os.path.join(__file__, "../../../target/release")))
import sir_ddft

FIGSIZE = (10,6)
DPI = 110
FONTSIZE = 14
mpl.rcParams.update({"font.size" : FONTSIZE})

params = sir_ddft.SIRParameters(1.0,0.2,0.0)
state  = sir_ddft.SIRState(0.999,0.001,0.0)
solver = sir_ddft.SIRSolver(params, state)

t = []
S = []
I = []
R = []
def store_result(result):
    global S,I,R
    t.append(result["time"])
    S.append(result["S"])
    I.append(result["I"])
    R.append(result["R"])

dt = 0.3
store_result(solver.get_result())
for i in range(100):
    solver.add_time(dt)
    solver.integrate()
    store_result(solver.get_result())

fig, ax = plt.subplots(figsize=FIGSIZE, dpi=DPI)

ax.plot(t,S, label="S")
ax.plot(t,I, label="I")
ax.plot(t,R, label="R")

ax.grid(True)
fig.suptitle("SIR model")
ax.set_xlabel("t")
ax.set_ylabel("Population")
ax.legend()

plt.show()