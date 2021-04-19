import numpy as np
import matplotlib.pyplot as plt
import matplotlib as mpl
from matplotlib.colors import LogNorm

def run_sim(solver, dt, frames, title):
    t = []
    S = []
    I = []
    R = []

    def store_result(result):
        t.append(result["time"])
        S.append(result["S"])
        I.append(result["I"])
        R.append(result["R"])

    store_result(solver.get_result())
    for i in range(frames):
        print(f"{i}/{frames}")
        solver.add_time(dt)
        solver.integrate()
        store_result(solver.get_result())

    S = np.clip(np.stack(S), 1e-15, None)
    I = np.clip(np.stack(I), 1e-15, None)
    R = np.clip(np.stack(R), 1e-15, None)

    fig, ax = plt.subplots()

    cax = ax.imshow(I, cmap="inferno", norm=LogNorm(1e-3, 1, True), aspect="auto",
        origin="lower")

    fig.colorbar(cax, label="Infected Population")

    x_ticks = np.linspace(0, S.shape[1], 5, endpoint=True)
    x_labels = [str(x/S.shape[1]) for x in x_ticks]
    plt.xticks(x_ticks, x_labels)

    t_ticks = np.linspace(0, S.shape[0], 5, endpoint=True)
    t_labels = [str(t_/S.shape[0] * np.max(t)) for t_ in t_ticks]
    plt.yticks(t_ticks, t_labels)

    fig.suptitle(title)
    ax.set_xlabel("x")
    ax.set_ylabel("t")

    plt.show()