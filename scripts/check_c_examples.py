#!/usr/bin/env python3
"""Compile and run the C guide examples against local fixtures."""
import argparse
import base64
import os
from pathlib import Path
import shlex
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--library-dir", type=Path, help="Use an already-built native library")
    args = parser.parse_args()
    if args.library_dir:
        library = args.library_dir.resolve()
    else:
        subprocess.run(["cargo", "build", "-p", "fatoora-ffi", "--locked"], cwd=ROOT, check=True)
        library = Path(os.environ.get("CARGO_TARGET_DIR", ROOT / "target")).resolve() / "debug"
    fixtures = ROOT / "fatoora-core/tests/fixtures"
    sdk = fixtures / "sdk-parity"
    cases = {
        "build_invoice": [],
        "csr": [fixtures / "csr-configs/csr-config-example-EN.properties"],
        "invoice_signing": [sdk / "cases/simplified-invoice/input.xml",
                            sdk / "credentials/certificate.der", sdk / "credentials/private-key.der"],
        "qr": [sdk / "cases/simplified-invoice/sdk-signed.xml"],
        "validation": [sdk / "cases/standard-invoice/input.xml"],
        # No arguments exercises help; never submit a real invoice in a docs check.
        "api": [],
    }
    with tempfile.TemporaryDirectory() as directory:
        for name, arguments in cases.items():
            binary = str(Path(directory) / name)
            subprocess.run([
                *shlex.split(os.environ.get("CC", "cc")), "-std=c11", "-Wall", "-Wextra", "-Werror",
                "-Ibindings/c",
                f"bindings/c/examples/{name}.c", "bindings/c/fatoora.c",
                "-L", str(library), "-lfatoora_ffi", f"-Wl,-rpath,{library}", "-o", binary,
            ], cwd=ROOT, check=True)
            result = subprocess.run([binary, *map(str, arguments)], capture_output=True, text=True)
            if result.returncode:
                raise SystemExit(f"{name} example failed: {result.stderr}")
            if name == "build_invoice":
                assert ">0.3333</cbc:PriceAmount>" in result.stdout
                assert ">1.15</cbc:TaxInclusiveAmount>" in result.stdout
            elif name == "csr":
                assert "BEGIN PRIVATE KEY" in result.stdout
                assert "BEGIN CERTIFICATE REQUEST" in result.stdout
            elif name == "invoice_signing":
                assert "SignatureValue" in result.stdout
            elif name == "qr":
                assert base64.b64decode(result.stdout.strip(), validate=True)
            elif name == "validation":
                assert "XSD valid: true" in result.stdout
                invalid = Path(directory) / "invalid.xml"
                invalid.write_text("<wrong/>")
                rejected = subprocess.run([binary, str(invalid)], capture_output=True, text=True)
                assert rejected.returncode != 0
            else:
                assert "Usage:" in result.stdout
            print(f"C API {name} guide example passed", flush=True)


if __name__ == "__main__":
    main()
