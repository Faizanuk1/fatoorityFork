#!/usr/bin/env python3
"""Stage the public C API and generated headers in a native release package."""
import argparse
from pathlib import Path
import shutil

from check_native_packages import C_API_FILES

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("destination", type=Path)
    args = parser.parse_args()
    sources = ROOT / "bindings/c"
    files = dict(C_API_FILES)
    files.update({f"generated/{header.name}": f"include/generated/{header.name}"
                  for header in (sources / "generated").glob("*.h")})
    for source, relative in files.items():
        destination = args.destination / relative
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(sources / source, destination)


if __name__ == "__main__":
    main()
