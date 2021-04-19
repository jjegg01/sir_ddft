import numpy as np
from numpy import pi, sqrt

import os, os.path
import sys
sys.path.append(os.path.abspath(os.path.join(__file__, "../../../target/release")))
import sir_ddft

from common_2d import run_sim

L=10
N=512
params = sir_ddft.SIRParameters(1.0,0.1)
diff_params = sir_ddft.SIRDiffusionParameters(0.01,0.01,0.01)
ddft_params = sir_ddft.SIRDDFTParameters(1.0,1.0,1.0,-10,100,-30,100)
grid = sir_ddft.Grid2D.new_equidistant(0,L,0,L,N,N)
S = lambda x,y: np.exp(-1/(L*2.0*L/50.0)*((x-L/2)**2 + (y-L/2)**2)) * 2.832
I = lambda x,y: 0.001*S(x,y)
state  = sir_ddft.SIRStateSpatial2D(grid, lambda x,y: [S(x,y),I(x,y),0.0])
solver = sir_ddft.SIRDDFT2DSolver(params, diff_params, ddft_params, state, 6)

run_sim(solver, 0.1, 200, "SIR DDFT model")