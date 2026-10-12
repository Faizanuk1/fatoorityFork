"""Build the Linux x64 benchmark using a locally downloaded official SDK ZIP."""

import argparse
import json
import os
import platform
import shutil
import subprocess
import tempfile
import xml.etree.ElementTree as ET
from pathlib import Path
from zipfile import ZipFile

ROOT = Path(__file__).resolve().parents[2]
DEFAULT_WORK = ROOT / "dist/benchmarks/dotnet"


def extract_file(archive, name, destination, relative):
    path = (destination / relative).resolve()
    if not path.is_relative_to(destination.resolve()):
        raise ValueError(f"Archive path escapes destination: {name}")
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(archive.read(name))


def prepare_sdk(archive_path, work):
    """Validate a fresh extraction, then discard the previous SDK/build outputs."""
    with tempfile.TemporaryDirectory(prefix="sdk-", dir=work) as temporary:
        staged = Path(temporary)
        with ZipFile(archive_path) as archive:
            marker = "/Lib/Dot-Net8/DLL/Zatca.EInvoice.SDK/net8.0/"
            for name in archive.namelist():
                if name.endswith("/"):
                    continue
                if marker in name:
                    extract_file(archive, name, staged, name.split(marker)[1])
                elif "/Data/Rules/" in name:
                    extract_file(archive, name, staged, name.split("/Data/")[1])
        for relative in (
            "ZATCA.EInvoice.SDK.dll",
            "ZATCA.EInvoice.SDK.Contracts.dll",
            "runtimes/linux/lib/net8.0/IKVM.Runtime.dll",
            "runtimes/linux/lib/net8.0/IKVM.Java.dll",
            "Rules/Schematrons/CEN-EN16931-UBL.xsl",
            "Rules/Schematrons/20210819_ZATCA_E-invoice_Validation_Rules.xsl",
        ):
            if not (staged / relative).is_file():
                raise ValueError(f"Incomplete official SDK archive: missing {relative}")
        if not (staged / "ikvm/linux-x64").is_dir():
            raise ValueError("Incomplete official SDK archive: missing ikvm/linux-x64")
        # Cleaning bin and obj also removes assemblies copied by prior MSBuild runs.
        # Results and the CLI home are intentionally retained.
        for name in ("official", "bin", "obj"):
            path = work / name
            if path.is_symlink():
                path.unlink()
            elif path.exists():
                shutil.rmtree(path)
        staged.rename(work / "official")
    return work / "official"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("archive", type=Path)
    parser.add_argument("--work-dir", type=Path, default=DEFAULT_WORK)
    args = parser.parse_args()
    if platform.system() != "Linux" or platform.machine() != "x86_64":
        parser.error("This harness currently supports Linux x86_64 only.")
    work = args.work_dir.resolve()
    work.mkdir(parents=True, exist_ok=True)
    dotnet = os.environ.get("DOTNET", "dotnet")
    env = dict(os.environ, DOTNET_CLI_HOME=str(work / "home"))
    try:
        official = prepare_sdk(args.archive.expanduser(), work)
    except ValueError as error:
        parser.error(str(error))

    project = ET.Element("Project", Sdk="Microsoft.NET.Sdk")
    properties = ET.SubElement(project, "PropertyGroup")
    for key, value in {
        "OutputType": "Exe", "TargetFramework": "net8.0",
        "AllowUnsafeBlocks": "true", "Nullable": "enable",
        "EnableDefaultCompileItems": "false",
    }.items():
        ET.SubElement(properties, key).text = value
    items = ET.SubElement(project, "ItemGroup")
    shutil.copy(Path(__file__).with_name("Program.cs"), work / "Program.cs")
    ET.SubElement(items, "Compile", Include="Program.cs")
    ET.SubElement(items, "Compile", Include=str(ROOT / "bindings/dotnet/generated/*.cs"))
    assemblies = sorted(official.glob("*.dll"))
    assemblies += sorted((official / "runtimes/linux/lib/net8.0").glob("*.dll"))
    for assembly in assemblies:
        reference = ET.SubElement(items, "Reference", Include=assembly.stem)
        ET.SubElement(reference, "HintPath").text = str(assembly)
    ET.indent(project)
    ET.ElementTree(project).write(work / "Bench.csproj", encoding="unicode")

    subprocess.run(["cargo", "build", "--release", "-p", "fatoora-ffi", "--locked"], cwd=ROOT, check=True)
    metadata = json.loads(subprocess.check_output(
        ["cargo", "metadata", "--no-deps", "--format-version", "1"], cwd=ROOT, text=True
    ))
    subprocess.run([dotnet, "build", "-c", "Release", str(work / "Bench.csproj")], cwd=ROOT, env=env, check=True)
    output = work / "bin/Release/net8.0"
    shutil.copy(Path(metadata["target_directory"]) / "release/libfatoora_ffi.so", output)
    shutil.copytree(official / "ikvm", output / "ikvm")
    # The SDK loads these external stylesheets, not its older embedded copies.
    shutil.copytree(official / "Rules", output / "Rules")
    print(f"Ready: python3 bench/dotnet/run.py --work-dir {work}")


if __name__ == "__main__":
    main()
