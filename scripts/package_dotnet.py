#!/usr/bin/env python3
"""Build a NuGet package containing the native library for this host's RID."""
import argparse
import os
from pathlib import Path
import platform
import shutil
import subprocess
import tomllib
from zipfile import ZipFile

ROOT = Path(__file__).resolve().parents[1]


def native_target():
    system = platform.system()
    arch = {"x86_64": "x64", "AMD64": "x64", "aarch64": "arm64", "arm64": "arm64"}.get(platform.machine())
    names = {"Linux": ("linux", "libfatoora_ffi.so"), "Darwin": ("osx", "libfatoora_ffi.dylib"), "Windows": ("win", "fatoora_ffi.dll")}
    if system not in names or arch is None:
        raise SystemExit("Unsupported native target")
    os_name, filename = names[system]
    return f"{os_name}-{arch}", filename


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, default=ROOT / "dist/dotnet")
    parser.add_argument("--stage-dir", type=Path, help="Stage the host native library for a combined CI package")
    parser.add_argument("--native-root", type=Path, help="Package staged runtimes without rebuilding")
    args = parser.parse_args()
    rid, filename = native_target()
    version = tomllib.loads((ROOT / "fatoora-ffi/Cargo.toml").read_text())["package"]["version"]
    target = Path(os.environ.get("CARGO_TARGET_DIR", ROOT / "target")).resolve()
    if args.native_root:
        if not list(args.native_root.glob("runtimes/*/native/*")):
            raise SystemExit("No native assets found")
        native_args = [f"-p:NativeRoot={args.native_root.resolve()}"]
    else:
        subprocess.run(["cargo", "build", "--release", "-p", "fatoora-ffi", "--locked"], cwd=ROOT, check=True)
        library = target / "release" / filename
        native_args = [f"-p:NativeRid={rid}", f"-p:NativeLibrary={library}"]
        if args.stage_dir:
            destination = args.stage_dir / "runtimes" / rid / "native" / filename
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(library, destination)
    package = args.output_dir / f"Fatoority.{version}.nupkg"
    package.unlink(missing_ok=True)
    subprocess.run([os.environ.get("DOTNET", "dotnet"), "pack", str(ROOT / "bindings/dotnet/Fatoora.csproj"),
                    "-c", "Release", "--output", str(args.output_dir.resolve()),
                    *native_args,
                    f"-p:PackageVersion={version}"], cwd=ROOT, check=True)
    with ZipFile(package) as archive:
        assets = [name for name in archive.namelist() if name.startswith("runtimes/")]
        if not assets or any(len(name.split("/")) != 4 or name.split("/")[2] != "native" for name in assets):
            raise SystemExit(f"Invalid native package layout: {assets}")
        if args.native_root:
            expected = {p.relative_to(args.native_root).as_posix() for p in args.native_root.glob("runtimes/*/native/*") if p.is_file()}
        else:
            expected = {f"runtimes/{rid}/native/{filename}"}
        if set(assets) != expected:
            raise SystemExit("Package does not contain exactly the staged native assets")
        for name in assets:
            source = args.native_root / name if args.native_root else library
            if archive.read(name) != source.read_bytes():
                raise SystemExit(f"Stale native library in package: {name}")


if __name__ == "__main__":
    main()
