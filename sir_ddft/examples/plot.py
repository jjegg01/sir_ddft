import sys
import numpy as np

try:
    import matplotlib.pyplot as plt
    from matplotlib.colors import LogNorm
    from matplotlib.animation import FuncAnimation
    from matplotlib.animation import FFMpegWriter
    MATPLOTLIB_AVAILABLE = True
except ImportError:
    MATPLOTLIB_AVAILABLE = False

# Router for different plotting routines
def plot(sir_data, outfile=None):
    t = sir_data["t"]
    S = sir_data["S"]
    I = sir_data["I"]
    R = sir_data["R"]
    title = sir_data["title"]

    if len(S.shape) == 1:
        plot_sir(t, S, I, R, title, outfile)
    elif len(S.shape) == 2:
        xlim = sir_data["xlim"]
        plot_sir_1d(t, S, I, R, xlim, title, outfile)
    elif len(S.shape) == 3:
        xlim = sir_data["xlim"]
        plot_sir_2d(t, S, I, R, xlim, title, outfile)
    else:
        raise TypeError("invalid shape for plotting")

# Plot standard SIR model results
def plot_sir(t, S, I, R, title, outfile=None):
    fig, ax = plt.subplots()

    ax.plot(t,S, label="S")
    ax.plot(t,I, label="I")
    ax.plot(t,R, label="R")

    ax.grid(True)
    ax.legend()

    fig.suptitle("SIR model")
    ax.set_xlabel("t")
    ax.set_ylabel("Population")

    if outfile:
        fig.savefig(outfile, dpi=300)
    else:
        plt.show()

# Plot space-time plots for spatial 1D models
def plot_sir_1d(t, S, I, R, xlim, title, outfile=None):
    fig, axes = plt.subplots(ncols=3, sharey=True)
    ax_S, ax_I, ax_R = axes

    vmin = 1e-3
    vmax = np.max([S,I,R])
    norm = LogNorm(vmin=vmin, vmax=vmax, clip=True)

    imshow_kwargs = {
        "origin": "lower",
        "norm": norm,
        "extent": [*xlim, np.min(t), np.max(t)],
        "aspect": (xlim[1] - xlim[0])/(np.max(t) - np.min(t))*1.5,
        "interpolation":"nearest"
    }
    ax_S.imshow(S, label="S", **imshow_kwargs)
    ax_I.imshow(I, label="I", **imshow_kwargs)
    im = ax_R.imshow(R, label="R", **imshow_kwargs)

    ax_S.title.set_text("S")
    ax_I.title.set_text("I")
    ax_R.title.set_text("R")

    fig.suptitle(title)
    for ax in axes:
        ax.set_xlabel("x")
    ax_S.set_ylabel("t")

    cax = fig.add_axes([0.1, 0.05, 0.8, 0.05])
    cbar = fig.colorbar(im, cax=cax, orientation="horizontal", label="Population density")

    if outfile:
        fig.savefig(outfile, dpi=300)
    else:
        plt.show()

# Plot videos for spatial 2D models
def plot_sir_2d(t, S, I, R, xlim, title, outfile=None):
    fig, axes = plt.subplots(ncols=3, sharey=True)
    ax_S, ax_I, ax_R = axes

    vmin = 1e-3
    vmax = np.max([S,I,R])
    norm = LogNorm(vmin=vmin, vmax=vmax, clip=True)
    
    imshow_kwargs = {
        "cmap": "inferno",
        "origin": "lower",
        "norm": norm,
        "extent": [*xlim, *xlim],
        "interpolation":"nearest"
    }
    im_S = ax_S.imshow(S[0], label="S", **imshow_kwargs)
    im_I = ax_I.imshow(I[0], label="I", **imshow_kwargs)
    im_R = ax_R.imshow(R[0], label="R", **imshow_kwargs)

    ax_S.title.set_text("S")
    ax_I.title.set_text("I")
    ax_R.title.set_text("R")

    fig.suptitle(title)
    for ax in axes:
        ax.set_xlabel("x")
    ax_S.set_ylabel("y")

    time_label = fig.text(0.49, 0.85, "$t=0$")

    cax = fig.add_axes([0.125, 0.05, 0.775, 0.05])
    cbar = fig.colorbar(im_R, cax=cax, orientation="horizontal", label="Population density")

    def update(frame):
        im_S.set_data(S[frame])
        im_I.set_data(I[frame])
        im_R.set_data(R[frame])
        time_label.set_text(f"$t={t[frame]:1.1f}$")
        return im_S, im_I, im_R, time_label
    
    ani = FuncAnimation(fig, update, frames=np.arange(1,S.shape[0]), blit=True, interval=30)

    if outfile:
        writer = FFMpegWriter(fps=30)
        ani.save(outfile, writer=writer)
    else:
        plt.show()

if __name__ == "__main__":
    # Error if matplotlib is unavailable
    if not MATPLOTLIB_AVAILABLE:
        print("Error: matplotlib is required for plotting")
        sys.exit(1)

    import argparse

    # Parse arguments
    parser = argparse.ArgumentParser(description="Plotting for sir_ddft examples")
    parser.add_argument("-o", "--output", default=None, help="Try to write to this file instead of showing plot directly")
    parser.add_argument("datafile")

    args = parser.parse_args()

    # Load data and call plotting router
    sir_data = np.load(args.datafile)
    plot(sir_data, outfile=args.output)