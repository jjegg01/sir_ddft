.PHONY: all clean data

all: data plot

data: data/passive_zombies_passive_humans.npz data/passive_zombies_fleeing_humans.npz data/chasing_zombies_passive_humans.npz data/chasing_zombies_fleeing_humans.npz
	python3 sz_ddft_phasediagram.py phasediagram_jobs.csv data/phasediagram/

plot: sz_time.pdf sz_totals.pdf sz_phasediagram.pdf sz_labels.pdf

sz_time.pdf: plot_sz_time.py
	python3 plot_sz_time.py \
	data/passive_zombies_passive_humans.npz \
	data/passive_zombies_fleeing_humans.npz \
	data/chasing_zombies_passive_humans.npz \
	data/chasing_zombies_fleeing_humans.npz \
	$@

sz_totals.pdf: plot_sz_totals.py
	python3 plot_sz_totals.py \
	data/passive_zombies_passive_humans.npz \
	data/passive_zombies_fleeing_humans.npz \
	data/chasing_zombies_passive_humans.npz \
	data/chasing_zombies_fleeing_humans.npz \
	$@

sz_phasediagram.pdf: plot_sz_phasediagram.py
	python3 plot_sz_phasediagram.py phasediagram_jobs.csv data/phasediagram/ $@

sz_labels.pdf: plot_labels.py
	python3 plot_labels.py sz_labels.pdf

data/passive_zombies_passive_humans.npz: sz_ddft_single.py
	python3 sz_ddft_single.py --fear-amplitude 0 --hunger-amplitude 0 $@

data/passive_zombies_fleeing_humans.npz: sz_ddft_single.py
	python3 sz_ddft_single.py --fear-amplitude -300 --hunger-amplitude 0 $@

data/chasing_zombies_passive_humans.npz: sz_ddft_single.py
	python3 sz_ddft_single.py --fear-amplitude 0 --hunger-amplitude -100 $@

data/chasing_zombies_fleeing_humans.npz: sz_ddft_single.py
	python3 sz_ddft_single.py --fear-amplitude -300 --hunger-amplitude -100 $@
