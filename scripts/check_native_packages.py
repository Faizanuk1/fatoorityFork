"""Check pinned rule notices and the C API in native release archives."""

import argparse
import stat
import tarfile
import zipfile
from pathlib import Path, PurePosixPath

RULE_NOTICES = ("LICENSE-LGPL-3.0.txt", "LICENSE-GPL-3.0.txt", "NOTICE.md")
C_API_FILES = {
    "fatoora.h": "include/fatoora.h",
    "fatoora.c": "src/fatoora.c",
    **{f"examples/{name}.c": f"examples/c/{name}.c" for name in
       ("build_invoice", "csr", "invoice_signing", "qr", "validation", "api")},
}


def check_archive(path, expected):
    found = set()

    def check_member(name, regular, read):
        name = str(PurePosixPath(name.replace("\\", "/")))
        if name not in expected:
            return
        if name in found:
            raise ValueError(f"{path.name}: duplicate {name}")
        if not regular:
            raise ValueError(f"{path.name}: {name} must be a regular file")
        if read() != expected[name]:
            raise ValueError(f"{path.name}: {name} differs from the pinned source")
        found.add(name)

    if path.name.endswith(".tar.gz"):
        with tarfile.open(path, "r:gz") as archive:
            for member in archive.getmembers():
                check_member(
                    member.name,
                    member.isfile(),
                    lambda member=member: archive.extractfile(member).read(),
                )
    elif path.suffix == ".zip":
        with zipfile.ZipFile(path) as archive:
            for member in archive.infolist():
                check_member(
                    member.filename,
                    not member.is_dir()
                    and not stat.S_ISLNK(member.external_attr >> 16),
                    lambda member=member: archive.read(member),
                )
    else:
        raise ValueError(f"{path.name}: expected a .tar.gz or .zip archive")

    missing = expected.keys() - found
    if missing:
        raise ValueError(f"{path.name}: missing {', '.join(sorted(missing))}")


def check_packages(directory, root):
    source = root / "fatoora-core/tests/fixtures/business-rules"
    expected = {
        f"licenses/zatca/{name}": (source / name).read_bytes() for name in RULE_NOTICES
    }
    packages = sorted(directory.iterdir())
    if not packages:
        raise ValueError(f"{directory}: no native release archives")
    for path in packages:
        package_expected = dict(expected)
        if path.name.startswith("fatoora-ffi-"):
            package_expected.update({
                destination: (root / "bindings/c" / name).read_bytes()
                for name, destination in C_API_FILES.items()
            })
            package_expected.update({
                f"include/generated/{header.name}": header.read_bytes()
                for header in (root / "bindings/c/generated").glob("*.h")
            })
        check_archive(path, package_expected)
    return len(packages)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "directory", type=Path, help="Directory of native release archives"
    )
    args = parser.parse_args()
    try:
        count = check_packages(args.directory, Path(__file__).resolve().parents[1])
    except (OSError, ValueError, tarfile.TarError, zipfile.BadZipFile) as error:
        parser.exit(1, f"Native package check failed: {error}\n")
    print(f"Verified native package contents in {count} release archive(s)")


if __name__ == "__main__":
    main()
