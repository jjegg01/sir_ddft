README
======

These scripts were used to generate the figures in [this article](https://doi.org/10.48550/arXiv.2307.00437).

To reproduce the results, simply run `make` after installing `sir_ddft` into your current Python environment.

**NOTE:** These scripts can run for a relatively long time, in particular for the simulations used to plot
the phasediagram. You can tune the performance to your local hardware by specifying the number of simulations
to run in parallel ("jobs") and the number of threads to use per simulation, e.g.:
```
python3 sz_ddft_phasediagram.py --jobs 24 --threads-per-job 2 phasediagram_jobs.csv data/phasediagram/
```
By default, 4 jobs with 2 threads each will be launched.

For comparison, the command above consumed about 16 CPU hours at 50% wall-time efficiency
on the [PALMA II cluster at the Universiy of Münster](https://www.uni-muenster.de/news/view.php?cmdid=9680) using an AMD Zen 4 node.