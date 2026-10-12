#!/usr/bin/env python3
"""Install the pinned C# generator from the reviewed fork revision."""
import argparse
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
REVISION = "8f2d28f1b39653320db1446ebd8fd264cc8a166d"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=ROOT / ".tools/diplomat-fork")
    args = parser.parse_args()
    with tempfile.TemporaryDirectory() as directory:
        source = Path(directory)
        def run(*command):
            subprocess.run(command, cwd=source, check=True)
        run("git", "init", "--quiet")
        run("git", "fetch", "--depth=1", "https://github.com/mqqz/diplomat.git", REVISION)
        run("git", "checkout", "--detach", "FETCH_HEAD")
        run("cargo", "test", "-p", "diplomat-tool", "--locked", "dotnet")
        run("cargo", "install", "--path", str(source / "tool"), "--locked",
            "--root", str(args.root.resolve()), "--force")


if __name__ == "__main__":
    main()
