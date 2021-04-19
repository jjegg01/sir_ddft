import numpy as np

import os, os.path
import sys
sys.path.append(os.path.abspath(os.path.join(__file__, "../../../target/release")))
import sir_ddft as sir_ddft

from common_1d import run_sim

params = sir_ddft.SIRParameters(0.5,0.1)
diff_params = sir_ddft.SIRDiffusionParameters(0.01,0.01,0.01)
ddft_params = sir_ddft.SIRDDFTParameters(1.0,1.0,1.0,-5,100,-10,100)
grid = sir_ddft.Grid1D.new_equidistant(0,1,256)
def initfunc(x):
    variance = 50**(-2)
    S = np.exp(-(x-0.5)**2/(2*variance)) / np.sqrt(2*np.pi*variance)
    return [S, S*0.001, 0.0]
state  = sir_ddft.SIRStateSpatial1D(grid, initfunc)
solver = sir_ddft.SIRDDFT1DSolver(params, diff_params, ddft_params, state, 4)

run_sim(solver, 0.25, 400, "SIR DDFT model")