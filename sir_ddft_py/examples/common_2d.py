import numpy as np
import matplotlib.pyplot as plt
import matplotlib as mpl
import matplotlib.animation as animation

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

    S = [np.clip(data, 1e-15, None) for data in S]
    I = [np.clip(data, 1e-15, None) for data in I]
    R = [np.clip(data, 1e-15, None) for data in R]

    fig, ax = plt.subplots()

    vmax = np.max(I)
    cax = ax.imshow(I[0], cmap="inferno", vmin=0, vmax=vmax, aspect="auto",
        origin="lower", extent=[0,S[0].shape[0],0,S[0].shape[1]])

    fig.colorbar(cax, label="Infected Population")

    x_ticks = np.linspace(0, S[0].shape[0], 5, endpoint=True)
    x_labels = [str(x/S[0].shape[0]) for x in x_ticks]
    plt.xticks(x_ticks, x_labels)

    y_ticks = np.linspace(0, S[0].shape[1], 5, endpoint=True)
    y_labels = [str(y/S[0].shape[1]) for y in y_ticks]
    plt.xticks(y_ticks, y_labels)

    fig.suptitle(title)
    ax.set_xlabel("x")
    ax.set_ylabel("y")

    label = ax.text(0.05,0.90, f"$t={t[0]}$", color="gray", transform=ax.transAxes)

    def update(data):
        curr_I, curr_t = data
        cax.set_array(curr_I)
        label.set_text("$t={:.1f}$".format(curr_t))
        return cax, label

    myAnimation = animation.FuncAnimation(fig, update, list(zip(I,t)), interval=40, repeat=True)

    plt.show()