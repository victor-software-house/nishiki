# Which host class gives stable wall-clock figures under Criterion

## Question

The [first probe](2026-11-04-bench-host-probe.md) found no host whose A/A medians agree within 2% on every language, with the old `examples/bench.rs` harness.
Part of that spread could be the harness.
The old harness timed 20 passes once.
Does the Criterion harness of task 5.1 change the answer, and which host class should records come from?
How far does one build move between the start and the end of a session?
The recorder's anchor guard follows that figure.

This note informs task 3.2 and the anchor guard of task 6.1.

## Answer

1. No host class meets design decision 11's criterion under Criterion either.
   On every host and mode, 13 to 46 of the 46 languages differ by more than 2% between the two halves of one session.
   So decision 11, point 6 applies.
   Wall-clock records are indicative.
   Speed decisions rest on Callgrind instruction counts, as the gate already does.
2. Namespace 8x16, unpinned, is the least noisy host and becomes the reference class for records.
   A language's figure moves by 8.5% across six runs (median over languages), and the last run differs from the first by 1.9% (median) and 10.2% (90th percentile).
3. Pinning to one CPU does not reliably help.
   It lowered the A/A gaps on instabox and Boat.
   On Namespace it widened the drift.
   The 8x16's 90th percentile went from 10.2% to 14.4%.
4. Part of the drift is the whole host changing speed, not one language's noise.
   On the pinned 8x16, all 46 languages ran faster in the last run than in the first.
   The host ran 5 to 9% faster after its first run.
   Each run can be divided by the host's speed in that run.
   That still leaves a spread of 5.2 to 27.5% (median over languages) on every host.
   No host class is within 2% even after that correction.
5. Boat `large` is the noisiest host measured.
   Its spread is 37%, and its host speed moved by up to 26% between runs.
   It stays fine for Callgrind, whose instruction counts do not depend on the host's speed.
   It is not fit for wall-clock work.
6. So the recorder's anchor guard follows the reference class's measured drift:
   a session on Namespace 8x16 whose anchor moves more than 11% (the 90th percentile above, rounded up) writes no record.
   The record carries that bound, and every figure carries its spread over the rounds.

## Method

- Commit [`a2bdad98`][commit], the bench workspace's `throughput` bench built once per host with `cargo bench --locked --bench throughput --no-run`.
- Six runs per mode on each host, alternating: one unpinned, then one pinned with `taskset -c <last CPU>`.
  Each run is the medium tier of every language (`lex/.*/medium`) at Criterion's settings: 1 s warm-up, 3 s measurement, 50 samples.
  [`aa-crit.sh`](2026-11-04-bench-host-probe-criterion/aa-crit.sh) is what ran on each host.
- A language's figure in a run is Criterion's median time per iteration.
- [`aa.ts`](2026-11-04-bench-host-probe-criterion/aa.ts) computes, per language and mode:
  - the A/A gap: the median of the even runs against the median of the odd runs, as a magnitude;
  - the spread: the largest run minus the smallest, over the median of the six;
  - the drift: the last run against the first, as a magnitude.
    It is the probe's analogue of the recorder's anchor, measured at a session's start and end.
- `bun aa.ts check` tests the gap on known pairs.
  A negative gap cannot rank below a positive one.
- Each host's folder beside this note holds `runs.tsv` (every run's median), `report.txt` (the per-language lines), `host.txt` and `progress.txt`.

## Results

Gaps, spreads and drifts are medians or extremes over the 46 languages.
"Without the host" is the median spread after each run is divided by that run's host speed.
A run's host speed is the geometric mean of its languages' throughput.

| Host | Mode | Worst A/A gap | Over 2% | Median spread | Without the host | Drift median | Drift p90 |
|:--|:--|--:|--:|--:|--:|--:|--:|
| instabox, 4 vCPU EPYC, 1 thread a core | unpinned | 17.2% (proto) | 34 | 18.9% | 13.2% | 5.7% | 16.4% |
| instabox | pinned | 8.2% (typescript) | 27 | 15.5% | 10.5% | 9.5% | 20.1% |
| Namespace 4x8, EPYC, 2 threads a core | unpinned | 18.2% (zig) | 30 | 15.5% | 11.3% | 7.0% | 15.3% |
| Namespace 4x8 | pinned | 30.0% (xml) | 25 | 19.9% | 16.1% | 3.5% | 15.9% |
| Namespace 8x16, EPYC, 2 threads a core | unpinned | 7.9% (java) | 15 | 8.5% | 8.1% | 1.9% | 10.2% |
| Namespace 8x16 | pinned | 7.8% (powershell) | 13 | 9.5% | 5.2% | 9.8% | 14.4% |
| Boat `large`, 8 vCPU Ryzen 9 9950X, 1 thread a core | unpinned | 35.3% (javascript) | 46 | 36.5% | 25.1% | 24.8% | 35.5% |
| Boat `large` | pinned | 29.8% (tsx) | 45 | 37.4% | 27.5% | 16.6% | 27.3% |

The host's speed in each run, against its first run:

| Host | Mode | Runs 2 to 6 |
|:--|:--|:--|
| instabox | unpinned | −0.9%, +9.5%, +2.1%, −7.1%, +5.5% |
| instabox | pinned | +11.3%, +14.4%, +4.7%, +8.0%, +9.8% |
| Namespace 4x8 | unpinned | +2.9%, +1.8%, −2.0%, −6.0%, −7.0% |
| Namespace 4x8 | pinned | +2.4%, −10.8%, −0.4%, −2.7%, −3.8% |
| Namespace 8x16 | unpinned | +2.6%, +0.5%, −1.5%, +3.8%, +3.2% |
| Namespace 8x16 | pinned | +5.5%, +8.5%, +9.1%, +7.6%, +9.0% |
| Boat `large` | unpinned | −21.6%, −9.9%, −14.5%, −9.6%, −25.6% |
| Boat `large` | pinned | −8.4%, +6.1%, −19.7%, +7.5%, −8.7% |

The anchor language, Python, drifted by 1.7% on the unpinned 8x16 and by 4.0 to 26.5% everywhere else.
The [first probe](2026-11-04-bench-host-probe.md), with the old harness, found a worst gap of 4.4 to 15.2% and 14 to 33 languages over 2%:
Criterion narrowed the 8x16's spread from 17.8% to 8.5%, but did not bring any host within 2%.

A Namespace instance is recognised by its `/nsc` directory, checked on an 8x16 together with its shape: 8 CPUs, AMD EPYC, 2 threads a core, 16,052 MiB.

## Not verified

1. Whether a larger host, such as a Namespace 16x32 or Boat `xlarge`, does better.
2. What changes a host's speed between runs, and whether a run could remove it.
   In three sessions the first run stands apart.
   The pinned instabox and 8x16 ran slower in it than in every later run.
   The unpinned Boat ran faster in it.
   In the others the speed rises and falls with no trend.
3. The anchor's own spread.
   The recorder measures the anchor for 2 s, and these runs measured for 3 s.
   The anchor's drift may be a little wider than the probe's.

## Compute

Each host ran from create to delete under its run script, whose exit trap removed it.

1. instabox, free plan: 2,940 s.
2. Namespace 4x8, instance `upl9773vkvnmg`: 2,845 s, about 190 unit minutes, about $0.28.
3. Namespace 8x16, instance `puenoderr08g0`: 2,814 s, about 375 unit minutes, about $0.56.
4. Boat `large`, sandbox: 2,850 s at twice the default rate.
   An earlier sandbox ran 1,836 s for nothing.
   Its job was started with `nohup` inside `boat exec`.
   `boat exec` killed it at the exec's 300 s timeout.
   The rerun used `boat exec --detach`.
5. Namespace 8x16 scratch instance `gsoi6hp5j741i`, to find the marker: 5 s.

[commit]: https://github.com/victor-software-house/nishiki-archive/commit/a2bdad9882389c892074ef75139d890982b792c0
