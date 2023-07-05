import numpy as np

import os, os.path
import sys
sys.path.append(os.path.abspath(os.path.join(__file__, "../../../target/release")))
import sir_ddft

from common_2d import run_sim

params = sir_ddft.SIRParameters(0.5,0.1,0.0)
diff_params = sir_ddft.SIRDiffusionParameters(0.001,0.001,0.001)
grid = sir_ddft.Grid2D.new_equidistant(0,1,0,1,64,64)
state  = sir_ddft.SIRStateSpatial2D(grid, lambda x,y: [1.0,0.1*np.exp(-((x-0.5)**2 + (y-0.5)**2)*100),0.0])
solver = sir_ddft.SIRDiffusion2DSolver(params, diff_params, state)

run_sim(solver, 0.25, 400, "SIR model with diffusion")