"""Measure warm calls and fresh processes; leave historical results untouched."""

import argparse
import hashlib
import json
import os
import platform
import statistics
import subprocess
import time
from pathlib import Path

from setup import DEFAULT_WORK, ROOT


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--work-dir", type=Path, default=DEFAULT_WORK)
    parser.add_argument("--output-dir", type=Path)
    args = parser.parse_args()
    work = args.work_dir.resolve()
    output = (args.output_dir or work / "results").resolve()
    output.mkdir(parents=True, exist_ok=True)
    dotnet = os.environ.get("DOTNET", "dotnet")
    env = dict(os.environ, DOTNET_CLI_HOME=str(work / "home"),
               DOTNET_TieredCompilation="0",
               FATOORA_BENCH_FIXTURES=str(ROOT / "fatoora-core/tests/fixtures/sdk-parity"))
    binary = work / "bin/Release/net8.0/Bench.dll"
    if not binary.is_file():
        parser.error("Build first with bench/dotnet/setup.py.")
    command = [dotnet, str(binary)]
    with (output / "final-results.log").open("w") as log:
        for kind in ("simplified", "standard"):
            for operation in ("hash", "sign", "validate"):
                for implementation in ("official", "ours"):
                    subprocess.run(command + [kind, implementation, operation],
                                   env=env, stdout=log, stderr=log, check=True)
                    print(kind, operation, implementation, flush=True)
    rows = []
    for operation in ("hash", "sign", "validate"):
        samples = {implementation: [] for implementation in ("official", "ours")}
        for _ in range(5):
            for implementation, values in samples.items():
                start = time.perf_counter()
                subprocess.run(command + ["cold", implementation, operation],
                               env=env, capture_output=True, text=True, check=True)
                values.append((time.perf_counter() - start) * 1000)
        for implementation, values in samples.items():
            rows.append(dict(op=operation, impl=implementation,
                             process_wall_ms=values, median_ms=statistics.median(values)))
        print("cold", operation, flush=True)
    (output / "cold-results.json").write_text(json.dumps(rows, indent=2) + "\n")
    metadata = {
        "head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
        "platform": platform.platform(),
        "dotnet_sdk": subprocess.check_output([dotnet, "--version"], cwd=ROOT, env=env, text=True).strip(),
        "rustc": subprocess.check_output(["rustc", "--version"], text=True).strip(),
    }
    for name, path in {
        "official_dll": work / "official/ZATCA.EInvoice.SDK.dll",
        "native": binary.parent / "libfatoora_ffi.so",
    }.items():
        metadata[name + "_sha256"] = hashlib.sha256(path.read_bytes()).hexdigest()
    (output / "metadata.json").write_text(json.dumps(metadata, indent=2) + "\n")
    print(f"Results: {output}")


if __name__ == "__main__":
    main()
