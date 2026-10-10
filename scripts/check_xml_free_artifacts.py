#!/usr/bin/env python3
"""Inspect native imports in built binaries, archives, and unpacked wheels."""
import argparse
from pathlib import Path, PurePosixPath
import re
import struct
import subprocess
import tarfile
import tempfile
import zipfile


def pe_imports(data):
    """Read PE import descriptors without relying on a Visual Studio shell."""
    pe = struct.unpack_from("<I", data, 0x3C)[0]
    if data[pe:pe + 4] != b"PE\0\0":
        raise ValueError("invalid PE header")
    sections, optional_size = struct.unpack_from("<H12xH", data, pe + 6)
    optional = pe + 24
    magic = struct.unpack_from("<H", data, optional)[0]
    if magic not in (0x10B, 0x20B):
        raise ValueError("unsupported PE optional header")
    directories = optional + (96 if magic == 0x10B else 112)
    imports, _ = struct.unpack_from("<II", data, directories + 8)
    table = optional + optional_size

    def offset(rva):
        for index in range(sections):
            size, address, raw_size, raw = struct.unpack_from("<IIII", data, table + index * 40 + 8)
            if address <= rva < address + max(size, raw_size):
                result = raw + rva - address
                if result >= len(data):
                    raise ValueError("PE RVA outside file")
                return result
        raise ValueError("unmapped PE RVA")

    cursor = offset(imports) if imports else None
    names = []
    while cursor is not None:
        descriptor = struct.unpack_from("<IIIII", data, cursor)
        if not any(descriptor):
            break
        start = offset(descriptor[3])
        end = data.index(b"\0", start)
        names.append(data[start:end].decode("ascii"))
        cursor += 20
    count = struct.unpack_from("<I", data, directories - 4)[0]
    if count > 13:
        delay, _ = struct.unpack_from("<II", data, directories + 13 * 8)
        cursor = offset(delay) if delay else None
        while cursor is not None:
            descriptor = struct.unpack_from("<IIIIIIII", data, cursor)
            if not any(descriptor):
                break
            name = descriptor[1]
            if not descriptor[0] & 1:
                image_base = struct.unpack_from("<I" if magic == 0x10B else "<Q", data, optional + (28 if magic == 0x10B else 24))[0]
                name -= image_base
            start = offset(name)
            names.append(data[start:data.index(b"\0", start)].decode("ascii"))
            cursor += 32
    return names


def inspect(path):
    data = path.read_bytes()
    if data.startswith(b"\x7fELF"):
        details = subprocess.check_output(["readelf", "-d", str(path)], text=True)
        dependencies = re.findall(r"Shared library: \[(.*?)\]", details)
        # The loader resolves transitive dependencies, which the direct table omits.
        resolved = subprocess.run(["ldd", str(path)], text=True, capture_output=True)
        if resolved.returncode == 0:
            dependencies.extend(resolved.stdout.splitlines())
    elif data.startswith(b"MZ"):
        dependencies = pe_imports(data)
    elif data[:4] in (b"\xcf\xfa\xed\xfe", b"\xfe\xed\xfa\xcf", b"\xce\xfa\xed\xfe", b"\xfe\xed\xfa\xce", b"\xca\xfe\xba\xbe", b"\xbe\xba\xfe\xca"):
        dependencies = subprocess.check_output(["otool", "-L", str(path)], text=True).splitlines()[1:]
    else:
        return False
    for name in [path.name, *dependencies]:
        if re.search(r"libxml|xml2", name, re.IGNORECASE):
            raise ValueError(f"libxml2 dependency in {path.name}: {name.strip()}")
    print(f"XML-free native imports: {path.name}")
    return True


def archive_path(directory, name):
    relative = PurePosixPath(name.replace("\\", "/"))
    if relative.is_absolute() or ".." in relative.parts:
        raise ValueError("artifact archive contains an unsafe path")
    return Path(directory).joinpath(*relative.parts)


def check(paths):
    count = 0
    for path in paths:
        if path.is_dir():
            count += check(sorted(item for item in path.rglob("*") if item.is_file()))
        elif zipfile.is_zipfile(path):
            with tempfile.TemporaryDirectory() as directory, zipfile.ZipFile(path) as archive:
                files = []
                for member in archive.infolist():
                    if not member.is_dir():
                        target = archive_path(directory, member.filename)
                        target.parent.mkdir(parents=True, exist_ok=True)
                        target.write_bytes(archive.read(member))
                        files.append(target)
                count += sum(inspect(item) for item in files)
        elif path.name.endswith((".tar.gz", ".tgz")):
            with tempfile.TemporaryDirectory() as directory, tarfile.open(path) as archive:
                files = []
                for member in archive:
                    if member.isfile():
                        target = archive_path(directory, member.name)
                        target.parent.mkdir(parents=True, exist_ok=True)
                        target.write_bytes(archive.extractfile(member).read())
                        files.append(target)
                count += sum(inspect(item) for item in files)
        else:
            count += inspect(path)
    return count


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("paths", nargs="+", type=Path)
    arguments = parser.parse_args()
    if not check(arguments.paths):
        raise SystemExit("no native artifacts inspected")
