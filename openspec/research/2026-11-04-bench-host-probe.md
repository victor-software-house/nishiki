# Which host class gives stable wall-clock benchmarks

## Question

The `bench-methodology` change picks its reference host by an A/A probe (design, decision 11):
the same build, measured twice, must agree within 2% on every language.
Which of the cheap hosts passes, and does pinning the harness to one CPU help?
Do the hosts expose hardware instruction counters?

This note informs task 3.1.
It does not make the decision.

## Answer

1. No candidate passes.
   On every host, unpinned or pinned, 14 to 33 of the 46 languages differ by more than 2% between the two halves of one session.
2. Pinning to one CPU does not make the figures stable.
   It lowered the worst gap on instabox from 14.5% to 4.4%, but more languages moved by more than 2% on a pinned Namespace 8x16 than on an unpinned one.
3. A language's throughput moves by 16% to 25% (median over languages) across ten runs of one build on every host.
4. instabox exposes no `perf` package for its kernel, and its `perf_event_paranoid` is 2.
   On Namespace the `perf` install failed silently.
   Whether Namespace exposes counters is not known.
5. So design decision 11, point 6 applies.
   Recorded wall-clock figures carry their measured spread.
   Speed claims rest on instruction counts.
   Part of the spread is the harness itself.
   It times 20 passes of each fixture once, with no warm-up beyond one pass.
   The probe should run again with the Criterion harness of task 5.1 before the first record.

## Method

- Commit [`4e1681b9`][commit], `examples/bench.rs` built once per host with `cargo build --release --locked --features textmate --example bench`.
- Ten runs per mode on each host, alternating: one unpinned, then one pinned with `taskset -c <last CPU>`.
- A language's throughput in a run is its fixtures' total KiB over their total time.
- The A/A gap is the median of the odd runs against the median of the even runs.
  The spread is the largest run minus the smallest, over the median of all ten.
- [`aa.sh`](2026-11-04-bench-host-probe/aa.sh) computes both from the raw outputs beside this note.

## Results

| Host | Mode | Worst A/A gap | Languages over 2% | Median spread | Worst spread |
|:--|:--|--:|--:|--:|--:|
| instabox, 4 vCPU EPYC, 1 thread a core | unpinned | 14.5% (c) | 16 | 15.6% | 44.6% (c) |
| instabox | pinned | 4.4% (lua) | 14 | 20.2% | 55.6% (less) |
| Namespace 4x8, EPYC, 2 threads a core | unpinned | 15.2% (bat) | 21 | 17.9% | 71.7% (bat) |
| Namespace 4x8 | pinned | 13.2% (less) | 22 | 18.2% | 48.1% (tsx) |
| Namespace 8x16, EPYC, 2 threads a core | unpinned | 13.1% (javascript) | 25 | 17.8% | 46.8% (erb) |
| Namespace 8x16 | pinned | 11.1% (less) | 33 | 24.7% | 63.3% (jsonl) |

Each host's CPU, memory, kernel and `perf` output are in its `host.txt` and `perf.txt`.

## Not verified

1. Whether Namespace exposes `instructions:u`: `apk add perf` printed nothing, and `perf` did not run.
2. Larger or dedicated hosts, such as a Namespace 16x32 or a bare-metal runner. Their cost per run was not weighed here.
3. How much of the spread the harness causes and how much the host does.
   The two separate only when the Criterion harness runs on the same hosts.

## Compute

1. Namespace 4x8, instance `hn7t2snu47sqc`: 158 s from create to destroy, about 12 unit minutes, about $0.02.
2. Namespace 8x16, instance `6ieet5ilkk7bu`: 150 s, about 24 unit minutes, about $0.04.
3. instabox `nishiki-bench-probe`, free plan: 460 s from create to delete.

Each instance was destroyed by its run script.

[commit]: https://github.com/victor-software-house/nishiki-archive/commit/4e1681b93903bf601e9fd3ed5000b8ac0ad61e22
