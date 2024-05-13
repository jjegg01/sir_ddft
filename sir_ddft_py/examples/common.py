import numpy as np
import plot

def run_sim(solver, dt, frames, title, outfile_prefix, xlim=None):
    # Initialize results dictionary
    results = {}
    initial_state = solver.get_result()
    for key in initial_state.keys():
        if key == "time":
            key = "t"
        results[key] = []

    def store_result(result):
        for key in result.keys():
            if key == "time":
                results["t"].append(result[key])
            else:
                results[key].append(result[key])

    # Integration
    store_result(initial_state)
    for i in range(frames):
        print(f"{i}/{frames}")
        solver.add_time(dt)
        solver.integrate()
        store_result(solver.get_result())

    # Transform output to NumPy types
    for key in results.keys():
        if key == "t":
            results[key] = np.asarray(results[key])
        else:
            results[key] = np.clip(results[key], 1e-15, None)
    results["xlim"] = xlim
    results["title"] = title
    
    if plot.MATPLOTLIB_AVAILABLE:
        if plot.MATPLOTLIB_INTERACTIVE:
            plot.plot(results)
        else:
            plot.plot(results, outfile_prefix=outfile_prefix)
    else:
        print(f"Warning: Failed to import matplotlib for graphical output. Output will be stored in file {outfile} instead.")
        print(f"Hint: You can run the plotting code manually via 'python3 examples/plot.py {outfile}' (needs matplotlib)")
        np.savez(outfile_prefix + ".npz", results)