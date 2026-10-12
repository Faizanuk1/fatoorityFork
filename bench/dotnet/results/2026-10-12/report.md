# C# SDK benchmark — 2026-10-12

Local comparison of Fatoority a192bf15af1267c1f551390936d396f9c418550e and downloaded ZATCA .NET SDK 238-R3.4.8.

AMD Ryzen 7 7800X3D, Linux x86_64; .NET SDK 8.0.425; rustc 1.98.0. Both builds use Release. DOTNET_TieredCompilation=0. Single-threaded callers, sequential measurements on a shared workstation. No CPU pinning or isolated-host controls.

## Warm calls

Median milliseconds per operation over seven batches after 15 warmup calls. Adaptive batch size targets 200 ms, capped at 1,000 iterations; official validation uses one call per batch. Raw samples and iteration counts are in final-results.log.

| Invoice | Operation | Official ms | Fatoority ms | Official / Fatoority |
|---|---|---:|---:|---:|
| simplified | hash | 2.487 | 0.176 | 14.1x |
| simplified | sign | 13.630 | 0.875 | 15.6x |
| simplified | validate | 265.453 | 5.940 | 44.7x |
| standard | hash | 2.851 | 0.336 | 8.5x |
| standard | sign | 15.423 | 1.366 | 11.3x |
| standard | validate | 264.568 | 5.992 | 44.2x |

## Fresh process

Median wall-clock milliseconds over five fresh processes per implementation, alternating implementations. Includes process launch, fixture reads, selected operation setup, one call, output and process exit. Uses the simplified fixture. OS filesystem cache remains warm; these are not cold-disk measurements.

| Operation | Official ms | Fatoority ms |
|---|---:|---:|
| hash | 119.5 | 49.5 |
| sign | 248.3 | 50.6 |
| validate | 1260.2 | 183.6 |

## Workload and correctness

- Frozen sdk-parity simplified and standard invoices: unsigned XML 4,568 / 8,680 bytes; signed XML 12,086 / 16,122 bytes.
- Both hash implementations matched the frozen expected digest.
- Both signing implementations used the same certificate and private key. An untimed preflight checked hashes of both signed outputs and verified both with the official validator.
- Validation timed exactly the same sdk-signed.xml files, with the same certificate and predecessor hash. Both validators returned valid. Existing documented integrity-policy differences remain; passing these fixtures does not prove universal parity.
- Public API costs: official calls receive a newly parsed XmlDocument each iteration; Fatoority receives XML strings and parses internally. Official signing parses credential strings on each call; Fatoority reuses a Signer. File I/O and key-format conversion are excluded from warm calls.
- Validation includes consumption of the success flag. Fatoority must parse its returned JSON; official returns a typed result. The benchmark does not time printing reports.
- first_ms in warm logs is diagnostic only: shared fixture/signing setup has already occurred. Use the separate fresh-process results for startup comparisons.
- No HTTP, concurrency, memory, large-invoice, or Windows/macOS benchmark was performed. These ratios describe these API calls and fixtures on this machine.

## Validation rule sources

The official assembly contains older embedded CEN/KSA resources, but ConfigurationManager.ReadDataResource loads external Rules/Schematrons files. The archive supplies those files with SHA-256 values matching our pinned profile:

- CEN: 378a7a4d697aa96e05edb4649ab4536b33546b8e288453324a5668c3e15735b0
- KSA: 3312e1938829b8972dacf19a3d933deeb97cb71c7e982c56a434b76cc0e5f0c4

The benchmark uses the external stylesheets. An initial run without the external Rules directory failed initialization and is excluded from timings.

## Reproduce

See [the harness README](../../README.md) for setup and run commands. The
repository harness replaces the original local absolute paths with paths
resolved from the checkout; the workload and sampling parameters are preserved.
The raw results here are the original 2026-10-12 measurements, not new results
from that relocation.

The official SDK archive and toolchain binaries are not included. Temporary
extractions and tools were removed after measurement. New runs default to an
ignored output directory and do not replace these records.
