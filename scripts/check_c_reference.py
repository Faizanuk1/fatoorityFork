#!/usr/bin/env python3
"""Compile C documentation declarations and complete examples against headers."""
import os
from pathlib import Path
import re
import shlex
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def main():
    headers = "\n".join(p.read_text() for directory in ("bindings/c", "bindings/c/generated")
                        for p in (ROOT / directory).glob("*.h"))
    count = 0
    with tempfile.TemporaryDirectory() as directory:
        for page in sorted((ROOT / "docs").rglob("*.md")):
            for index, block in enumerate(re.findall(r"```c\n(.*?)```", page.read_text(), re.S)):
                for symbol in re.findall(r"\bfatoora_\w+", block):
                    if not re.search(r"\b" + re.escape(symbol) + r"\b", headers):
                        raise SystemExit(f"{page.relative_to(ROOT)}: unknown C symbol {symbol}")
                # Standalone examples and declaration tabs include their headers.
                if "#include" not in block:
                    continue
                count += 1
                source = Path(directory) / (page.stem + f"_{index}.c")
                source.write_text(block)
                subprocess.run([
                    *shlex.split(os.environ.get("CC", "cc")), "-std=c11",
                    "-Wall", "-Wextra", "-Werror", "-fsyntax-only",
                    "-I", str(ROOT / "bindings/c"), str(source),
                    "-I", str(ROOT / "bindings/c/generated"),
                ], check=True)
    print(f"Compiled {count} C documentation blocks")


if __name__ == "__main__":
    main()
