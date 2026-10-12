#!/usr/bin/env python3
"""Reject Linux NuGet assets that require glibc newer than the release baseline."""
import argparse
from pathlib import Path
import re
import subprocess

BASELINE = (2, 28)


def check_versions(output):
    names = set(re.findall(r"Name: (GLIBC_\S+)", output))
    if not names:
        raise ValueError("No glibc version requirements found")
    versions = []
    for name in sorted(names):
        match = re.fullmatch(r"GLIBC_(\d+(?:\.\d+)+)", name)
        if match is None:
            raise ValueError(f"Unsupported glibc requirement: {name}")
        version = tuple(map(int, match[1].split(".")))
        if version > BASELINE:
            raise ValueError(f"{name} exceeds the GLIBC_2.28 baseline")
        versions.append(version)
    return max(versions)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("library", type=Path)
    args = parser.parse_args()
    output = subprocess.check_output(
        ["readelf", "--version-info", str(args.library)], text=True
    )
    try:
        version = check_versions(output)
    except ValueError as error:
        raise SystemExit(str(error)) from error
    print(f"Linux native glibc requirement: {'.'.join(map(str, version))} <= 2.28")


if __name__ == "__main__":
    main()
