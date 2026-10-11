#!/usr/bin/env python3
"""Build and run the generated C and C++ contract tests on Linux/macOS."""
import os
from pathlib import Path
import shlex
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def main():
    subprocess.run(["cargo", "build", "-p", "fatoora-ffi", "--locked"], cwd=ROOT, check=True)
    target = Path(os.environ.get("CARGO_TARGET_DIR", ROOT / "target")).resolve() / "debug"
    with tempfile.TemporaryDirectory() as directory:
        for language, compiler, standard, suffix in (
            ("c", os.environ.get("CC", "cc"), "c11", "c"),
            ("cpp", os.environ.get("CXX", "c++"), "c++17", "cpp"),
        ):
            binary = str(Path(directory) / language)
            subprocess.run([
                *shlex.split(compiler), f"-std={standard}", "-Wall", "-Wextra", "-Werror",
                "-I", "bindings/c/generated" if language == "c" else "bindings/cpp",
                f"fatoora-ffi/tests/diplomat_contract.{suffix}",
                "-L", str(target), "-lfatoora_ffi", f"-Wl,-rpath,{target}", "-o", binary,
            ], cwd=ROOT, check=True)
            subprocess.run([binary, str(ROOT / "fatoora-core/tests/fixtures/sdk-parity/cases/standard-invoice/sdk-signed.xml")] if language == "cpp" else [binary], check=True)
            print(f"{language} contract passed", flush=True)
        for test in ("c_facade_contract", "c_facade_allocation"):
            binary = str(Path(directory) / test)
            source = f"fatoora-ffi/tests/{test}.c"
            subprocess.run([
                *shlex.split(os.environ.get("CC", "cc")), "-std=c11", "-Wall", "-Wextra", "-Werror",
                "-I", "bindings/c", source,
                *([] if test == "c_facade_allocation" else ["bindings/c/fatoora.c"]),
                "-L", str(target), "-lfatoora_ffi", f"-Wl,-rpath,{target}", "-o", binary,
            ], cwd=ROOT, check=True)
            subprocess.run([
                binary, str(ROOT / "fatoora-core/tests/fixtures/sdk-parity/cases/standard-invoice/sdk-signed.xml"),
            ] if test == "c_facade_contract" else [binary], check=True)
        subprocess.run([sys.executable, str(ROOT / "scripts/check_c_examples.py"),
                        "--library-dir", str(target)], cwd=ROOT, check=True)
        subprocess.run([
            *shlex.split(os.environ.get("CXX", "c++")), "-std=c++17", "-Wall", "-Wextra", "-Werror",
            "-pedantic-errors", "-x", "c++", "-fsyntax-only", "bindings/c/fatoora.h",
        ], cwd=ROOT, check=True)
        binary = str(Path(directory) / "zatca")
        subprocess.run([
            *shlex.split(os.environ.get("CC", "cc")), "-std=c11", "-Wall", "-Wextra", "-Werror",
            "-I", "bindings/c/generated", "fatoora-ffi/tests/zatca_contract.c",
            "-L", str(target), "-lfatoora_ffi", f"-Wl,-rpath,{target}", "-o", binary,
        ], cwd=ROOT, check=True)
        subprocess.run([binary, str(ROOT / "fatoora-core/tests/fixtures/sdk-parity/cases/standard-invoice/input.xml")], check=True)


if __name__ == "__main__":
    main()
