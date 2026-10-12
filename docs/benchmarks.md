# Benchmarks

Benchmarks compare Fatoority with the official ZATCA SDK on local fixtures.
CLI process timings and warm C# API timings measure different workloads.

## CLI hashing

CLI invoice hashing benchmark results:
- `bench/cli/results/hash_bench.md`
<!-- bench-table-start -->
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `ZATCA SDK` | 537.8 ± 8.6 | 522.0 | 549.1 | 187.35 ± 12.59 |
| `fatoora-cli` | 2.9 ± 0.2 | 2.4 | 3.6 | 1.00 |
<!-- bench-table-end -->

The actual script for the benchmark is located at `bench/cli/hash_bench.sh`

Set `FATOORA_CLI` to the official SDK executable path and optionally
`FATOORITY_CLI` to this project's `fatoora` executable. The script otherwise
uses the local release or debug build for this project.

## C# / .NET

Measured on Linux x86_64 with an AMD Ryzen 7 7800X3D, .NET SDK 8.0.425,
and Release builds on 2026-10-12. Each value is the median milliseconds per
warm call across seven batches after 15 warmup calls.

| Operation | Invoice | Official .NET SDK | Fatoority | Speedup |
| --- | --- | ---: | ---: | ---: |
| Hash | Simplified | 2.487 ms | 0.176 ms | 14.1× |
| Hash | Standard | 2.851 ms | 0.336 ms | 8.5× |
| Sign | Simplified | 13.630 ms | 0.875 ms | 15.6× |
| Sign | Standard | 15.423 ms | 1.366 ms | 11.3× |
| Validate | Simplified | 265.453 ms | 5.940 ms | 44.7× |
| Validate | Standard | 264.568 ms | 5.992 ms | 44.2× |

Both implementations matched expected hashes, and both signing outputs passed
official validation. Both validators accepted the same frozen signed documents.
The official SDK's external CEN/KSA rules match our pinned rule sources.

These timings include public API costs: XML parsing, credential parsing in the
official signing method, and JSON result parsing in our validator. Our signer
reuses parsed credentials. They describe these fixtures on this machine;
large invoices, concurrency, HTTP, memory use, and other platforms were not
measured.

Fresh-process medians, including startup and one simplified-invoice operation:

| Operation | Official .NET SDK | Fatoority |
| --- | ---: | ---: |
| Hash | 120 ms | 50 ms |
| Sign | 248 ms | 51 ms |
| Validate | 1,260 ms | 184 ms |

See the [C# benchmark harness](https://github.com/mqqz/fatoority/tree/main/bench/dotnet)
for reproduction commands and the
[recorded results](https://github.com/mqqz/fatoority/blob/main/bench/dotnet/results/2026-10-12/report.md)
for raw samples, binary hashes, and measurement details.
