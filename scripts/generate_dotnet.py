#!/usr/bin/env python3
"""Generate C# using mqqz/diplomat's fatoora-fix generator (see dotnet README)."""
import argparse
import os
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    output = ROOT / "bindings/dotnet/generated"
    with tempfile.TemporaryDirectory() as temporary:
        subprocess.run([
            os.environ.get("DIPLOMAT_TOOL_DOTNET", str(ROOT / ".tools/diplomat-fork/bin" /
                ("diplomat-tool.exe" if os.name == "nt" else "diplomat-tool"))),
            "--entry", "fatoora-ffi/src/lib.rs",
            "--config", "lib_name=fatoora_ffi",
            "--config", "dotnet.namespace=Fatoora",
            "--config", "dotnet.diplomat_str_as_string=true",
            "--config", "dotnet.exception_message_method=Message",
            "dotnet", temporary,
        ], cwd=ROOT, check=True)
        sources = {p.name: p.read_bytes() for p in Path(temporary).glob("*.cs")}
        if not sources:
            raise SystemExit("Generator produced no C# sources")
        existing = {p.name: p.read_bytes() for p in output.glob("*.cs")}
        if args.check:
            if sources != existing:
                raise SystemExit("Stale C# bindings; run scripts/generate_dotnet.py")
        else:
            output.mkdir(parents=True, exist_ok=True)
            for obsolete in existing.keys() - sources.keys():
                (output / obsolete).unlink()
            for name, content in sources.items():
                (output / name).write_bytes(content)


if __name__ == "__main__":
    main()
