# C# SDK comparison

Compares the Fatoority C# bindings with the official ZATCA .NET SDK 238-R3.4.8
using the same frozen invoice fixtures. It measures hashing, signing, and local
validation through the public APIs, plus fresh-process latency.

The [2026-10-12 results](results/2026-10-12/report.md) include raw samples,
binary hashes, methodology, and limitations. Historical results are kept separate
from new runs.

## Run

Requirements: Linux x86_64, Rust, Python 3.11+, .NET SDK 8, and a locally
downloaded `zatca-einvoicing-sdk-DotNet-238-R3.4.8.zip`. The official SDK is not
redistributed here. `DOTNET` may select a .NET executable outside PATH.

From the repository root:

```sh
python3 bench/dotnet/setup.py ~/Downloads/zatca-einvoicing-sdk-DotNet-238-R3.4.8.zip
python3 bench/dotnet/run.py
```

Setup builds the Rust release library and a Release C# harness. It extracts the
official assemblies, IKVM files, and external validation rules into the ignored
`dist/benchmarks/dotnet/` directory. Existing Rust build outputs are reused.
No Docker or ZATCA network calls are required.

Both scripts accept `--work-dir`; pass the same directory to each. The runner
accepts `--output-dir`, defaulting to `results/` under the work directory. Each
run replaces files in its output directory, so choose a new directory when
preserving multiple runs.

To remove extracted SDKs and benchmark build outputs after saving any wanted
results:

```sh
rm -rf dist/benchmarks/dotnet
```

## Interpretation

Warm timings include XML parsing and success checks, but exclude file I/O and
fixture preparation. Fatoority reuses a parsed signer; the official signing API
accepts credential strings on every call. Validation consumes a JSON success
flag for Fatoority and a typed result for the official SDK. These are API-level
comparisons, not isolated cryptographic benchmarks.

Hashes must match frozen expectations. An untimed signing check confirms that
both signed outputs preserve those hashes and pass the official validator.
Both validators must accept the identical frozen signed fixtures. Failures stop
the run instead of becoming performance results.

The official SDK loads external `Rules/Schematrons` files. Its older embedded
stylesheets are not the active business-rule source. The external rule files in
the measured archive match Fatoority's pinned Java SDK profile.

Fresh-process measurements include launch, fixture reads, setup, one operation,
and exit. Filesystem caches remain warm. The harness disables tiered JIT and
measures implementations sequentially, without CPU pinning. Results do not
establish large-invoice, concurrent, HTTP, memory, or other-platform performance.
