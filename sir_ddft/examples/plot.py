#!/usr/bin/env python3

import sys
import numpy as np

try:
    import matplotlib
    import matplotlib.pyplot as plt
    from matplotlib.colors import LogNorm
    from matplotlib.animation import FuncAnimation
    from matplotlib.animation import FFMpegWriter
    MATPLOTLIB_AVAILABLE = True
    _MATPLOTLIB_NONINTERACTIVE_BACKENDS_EXTENSIONS = {
        "agg": "png", "pdf": "pdf", "ps": "ps", "svg": "svg", "pgf": "pdf", "cairo": "png"
    }
    MATPLOTLIB_INTERACTIVE = not matplotlib.get_backend() in _MATPLOTLIB_NONINTERACTIVE_BACKENDS_EXTENSIONS.keys()
    if not MATPLOTLIB_INTERACTIVE:
        _MATPLOTLIB_PREFERRED_FILEEXTENSION = _MATPLOTLIB_NONINTERACTIVE_BACKENDS_EXTENSIONS[matplotlib.get_backend()]
except ImportError:
    MATPLOTLIB_AVAILABLE = False

# Router for different plotting routines
def plot(data, outfile_prefix=None):
    # SIR data
    if "I" in data:
        t = data["t"]
        S = data["S"]
        I = data["I"]
        R = data["R"]
        title = data["title"]

        if len(S.shape) == 1:
            plot_sir(t, S, I, R, title, outfile_prefix)
        elif len(S.shape) == 2:
            xlim = data["xlim"]
            plot_sir_1d(t, S, I, R, xlim, title, outfile_prefix)
        elif len(S.shape) == 3:
            xlim = data["xlim"]
            plot_sir_2d(t, S, I, R, xlim, title, outfile_prefix)
        else:
            raise TypeError("invalid shape for plotting")
    # SZ data
    elif "Z" in data:
        t = data["t"]
        S = data["S"]
        Z = data["Z"]
        if len(S.shape) != 3:
            raise TypeError("invalid shape for plotting")
        xlim = data["xlim"]
        title = data["title"]
        plot_sir_2d_sz(t, S, Z, xlim, title, outfile_prefix)
    else:
        raise ValueError("cannot interpret data (neither SIR nor SZ data found)")

# Plot standard SIR model results
def plot_sir(t, S, I, R, title, outfile_prefix=None):
    fig, ax = plt.subplots()

    ax.plot(t,S, label="S")
    ax.plot(t,I, label="I")
    ax.plot(t,R, label="R")

    ax.grid(True)
    ax.legend()

    fig.suptitle("SIR model")
    ax.set_xlabel("t")
    ax.set_ylabel("Population")

    if outfile_prefix:
        fig.savefig(outfile_prefix + "." + _MATPLOTLIB_PREFERRED_FILEEXTENSION, dpi=300)
    else:
        plt.show()

# Plot space-time plots for spatial 1D models
def plot_sir_1d(t, S, I, R, xlim, title, outfile_prefix=None):
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

    if outfile_prefix:
        fig.savefig(outfile_prefix + "." + _MATPLOTLIB_PREFERRED_FILEEXTENSION, dpi=300)
    else:
        plt.show()

# Plot videos for spatial 2D models
def plot_sir_2d(t, S, I, R, xlim, title, outfile_prefix=None):
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

    if outfile_prefix:
        writer = FFMpegWriter(fps=30)
        ani.save(outfile_prefix + ".mp4", writer=writer)
    else:
        plt.show()

# Plot videos for spatial 2D models with zombies
def plot_sir_2d_sz(t, S, Z, xlim, title, outfile_prefix=None):
    fig, axes = plt.subplots(ncols=2, sharey=True)
    ax_S, ax_Z = axes

    vmin = 1e-3
    vmax = np.max([S,Z])
    norm = LogNorm(vmin=vmin, vmax=vmax, clip=True)
    
    imshow_kwargs = {
        "cmap": "inferno",
        "origin": "lower",
        "norm": norm,
        "extent": [*xlim, *xlim],
        "interpolation":"nearest"
    }
    im_S = ax_S.imshow(S[0], label="S", **imshow_kwargs)
    im_Z = ax_Z.imshow(Z[0], label="Z", **imshow_kwargs)

    ax_S.title.set_text("S")
    ax_Z.title.set_text("Z")

    fig.suptitle(title)
    for ax in axes:
        ax.set_xlabel("x")
    ax_S.set_ylabel("y")

    time_label = fig.text(0.49, 0.85, "$t=0$")

    cax = fig.add_axes([0.125, 0.05, 0.775, 0.05])
    cbar = fig.colorbar(im_Z, cax=cax, orientation="horizontal", label="Population density")

    def update(frame):
        im_S.set_data(S[frame])
        im_Z.set_data(Z[frame])
        time_label.set_text(f"$t={t[frame]:1.1f}$")
        return im_S, im_Z, time_label
    
    ani = FuncAnimation(fig, update, frames=np.arange(1,S.shape[0]), blit=True, interval=30)

    if outfile_prefix:
        writer = FFMpegWriter(fps=30)
        ani.save(outfile_prefix + ".mp4", writer=writer)
    else:
        plt.show()

# Plot videos for spatial 2D models with zombies
def plot_sir_2d_sz(t, S, Z, xlim, title, outfile=None):
    fig, axes = plt.subplots(ncols=2, sharey=True)
    ax_S, ax_Z = axes

    vmin = 1e-3
    vmax = np.max([S,Z])
    norm = LogNorm(vmin=vmin, vmax=vmax, clip=True)
    
    imshow_kwargs = {
        "cmap": "inferno",
        "origin": "lower",
        "norm": norm,
        "extent": [*xlim, *xlim],
        "interpolation":"nearest"
    }
    im_S = ax_S.imshow(S[0], label="S", **imshow_kwargs)
    im_Z = ax_Z.imshow(Z[0], label="Z", **imshow_kwargs)

    ax_S.title.set_text("S")
    ax_Z.title.set_text("Z")

    fig.suptitle(title)
    for ax in axes:
        ax.set_xlabel("x")
    ax_S.set_ylabel("y")

    time_label = fig.text(0.49, 0.85, "$t=0$")

    cax = fig.add_axes([0.125, 0.05, 0.775, 0.05])
    cbar = fig.colorbar(im_Z, cax=cax, orientation="horizontal", label="Population density")

    def update(frame):
        im_S.set_data(S[frame])
        im_Z.set_data(Z[frame])
        time_label.set_text(f"$t={t[frame]:1.1f}$")
        return im_S, im_Z, time_label
    
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
    parser.add_argument("-o", "--output", default=None, help="Try to write to this file instead of showing plot directly (extension is determined automatically)")
    parser.add_argument("datafile")

    args = parser.parse_args()

    # Load data and call plotting router
    data = np.load(args.datafile)
    plot(data, outfile=args.output)