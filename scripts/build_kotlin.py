#!/usr/bin/env python3
"""Build desktop JAR or Android AAR native assets and verify their packaged bytes."""

import argparse
import os
from pathlib import Path
import platform
import re
import shutil
import subprocess
import tomllib
from zipfile import ZipFile

ROOT = Path(__file__).resolve().parents[1]
PROJECT = ROOT / "bindings/kotlin"
ANDROID = {
    "arm64-v8a": ("aarch64-linux-android", "aarch64-linux-android"),
    "x86_64": ("x86_64-linux-android", "x86_64-linux-android"),
}


def run(*command, env=None):
    subprocess.run(command, cwd=ROOT, env=env, check=True)


def gradle(*args):
    executable = PROJECT / ("gradlew.bat" if os.name == "nt" else "gradlew")
    run(str(executable), "-p", str(PROJECT), "--no-daemon", *args)


def desktop_target():
    arch = {
        "x86_64": "x86-64",
        "AMD64": "x86-64",
        "aarch64": "aarch64",
        "arm64": "aarch64",
    }.get(platform.machine())
    systems = {
        "Linux": ("linux", "libfatoora_ffi.so"),
        "Darwin": ("darwin", "libfatoora_ffi.dylib"),
        "Windows": ("win32", "fatoora_ffi.dll"),
    }
    if arch is None or platform.system() not in systems:
        raise SystemExit("Unsupported desktop architecture")
    system, filename = systems[platform.system()]
    return f"{system}-{arch}", filename


def verify(package, expected, prefix=""):
    with ZipFile(package) as archive:
        native = {
            name
            for name in archive.namelist()
            if name.endswith((".so", ".dll", ".dylib"))
        }
        if native != {prefix + name for name in expected}:
            raise SystemExit(f"Unexpected native assets in {package}: {native}")
        for name, source in expected.items():
            if archive.read(prefix + name) != source.read_bytes():
                raise SystemExit(f"Stale packaged native library: {name}")
        # Android stores Java resources in classes.jar.
        if package.suffix == ".aar":
            from io import BytesIO

            archive = ZipFile(BytesIO(archive.read("classes.jar")))
            if "licenses/webpki-roots/LICENSE" not in archive.namelist():
                raise SystemExit("Missing Android TLS root license")
        for name in (
            "README.md",
            "THIRD_PARTY_NOTICES.md",
            "LICENSE-MIT",
            "LICENSE-APACHE",
            "licenses/zatca/LICENSE-LGPL-3.0.txt",
            "licenses/zatca/LICENSE-GPL-3.0.txt",
            "licenses/zatca/NOTICE.md",
        ):
            if name not in archive.namelist():
                raise SystemExit(f"Missing license file: {name}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--android", action="store_true")
    parser.add_argument(
        "--ndk", type=Path, help="Android NDK r28+; defaults to ANDROID_NDK_HOME"
    )
    parser.add_argument(
        "--native-root", type=Path, help="Combine previously staged desktop assets"
    )
    parser.add_argument(
        "--stage-dir", type=Path, help="Copy host desktop asset for CI assembly"
    )
    args = parser.parse_args()
    target = Path(os.environ.get("CARGO_TARGET_DIR", ROOT / "target")).resolve()
    if args.android:
        if args.native_root or args.stage_dir:
            parser.error("Desktop staging options cannot be used with --android")
        ndk = args.ndk or (
            Path(os.environ["ANDROID_NDK_HOME"])
            if "ANDROID_NDK_HOME" in os.environ
            else None
        )
        if ndk is None:
            parser.error("--android requires --ndk or ANDROID_NDK_HOME")
        host = {
            "Linux": "linux-x86_64",
            "Darwin": "darwin-x86_64",
            "Windows": "windows-x86_64",
        }[platform.system()]
        toolchain = ndk.resolve() / "toolchains/llvm/prebuilt" / host / "bin"
        staging = PROJECT / "android/build/native"
        shutil.rmtree(staging, ignore_errors=True)
        for abi, (rust, clang) in ANDROID.items():
            env = dict(os.environ)
            env["ANDROID_NDK_ROOT"] = str(ndk.resolve())
            env[f"CXX_{rust.replace('-', '_')}"] = str(
                toolchain / (clang + "26-clang++" + (".cmd" if os.name == "nt" else ""))
            )
            cc = str(
                toolchain / (clang + "26-clang" + (".cmd" if os.name == "nt" else ""))
            )
            env[f"CARGO_TARGET_{rust.upper().replace('-', '_')}_LINKER"] = cc
            env[f"CC_{rust.replace('-', '_')}"] = cc
            env[f"AR_{rust.replace('-', '_')}"] = str(
                toolchain / ("llvm-ar.exe" if os.name == "nt" else "llvm-ar")
            )
            # 16 KiB-compatible ELF segments; supported by NDK r28 and Android 15+.
            env[f"CARGO_TARGET_{rust.upper().replace('-', '_')}_RUSTFLAGS"] = (
                "-C link-arg=-Wl,-z,max-page-size=16384"
            )
            run(
                "cargo",
                "build",
                "--locked",
                "-p",
                "fatoora-ffi",
                "--profile",
                "jvm-release",
                "--target",
                rust,
                env=env,
            )
            library = target / rust / "jvm-release/libfatoora_ffi.so"
            headers = subprocess.check_output(
                [
                    str(
                        toolchain
                        / ("llvm-readelf.exe" if os.name == "nt" else "llvm-readelf")
                    ),
                    "-lW",
                    str(library),
                ],
                text=True,
            )
            alignment = [
                int(line.split()[-1], 16)
                for line in headers.splitlines()
                if re.match(r"\s*LOAD\s", line)
            ]
            if not alignment or min(alignment) < 16384:
                raise SystemExit(f"Android library lacks 16 KiB ELF alignment: {abi}")
            dest = staging / abi / "libfatoora_ffi.so"
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(target / rust / "jvm-release/libfatoora_ffi.so", dest)
        gradle(
            "-Pandroid=true",
            ":android:assembleRelease",
            ":android:assembleDebugAndroidTest",
            ":android:publishAndroidPublicationToStagingRepository",
        )
        gradle(
            "-Pandroid=true",
            ":android-consumer:assembleDebug",
            ":android-consumer:assembleDebugAndroidTest",
        )
        package = PROJECT / "android/build/outputs/aar/android-release.aar"
        verify(
            package,
            {p.relative_to(staging).as_posix(): p for p in staging.glob("*/*.so")},
            "jni/",
        )
    else:
        staging = PROJECT / "build/nativeResources"
        if args.native_root and args.native_root.resolve() == staging.resolve():
            parser.error("--native-root must be outside the package staging directory")
        shutil.rmtree(staging, ignore_errors=True)
        if args.native_root:
            assets = list(args.native_root.glob("*/*"))
            if not assets:
                parser.error("No staged native assets found")
            for source in assets:
                if not source.is_file():
                    parser.error(f"Invalid staged asset: {source}")
                dest = staging / source.relative_to(args.native_root)
                dest.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(source, dest)
        else:
            resource, filename = desktop_target()
            run(
                "cargo",
                "build",
                "--locked",
                "-p",
                "fatoora-ffi",
                "--profile",
                "jvm-release",
            )
            dest = staging / resource / filename
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(target / "jvm-release" / filename, dest)
            if args.stage_dir:
                out = args.stage_dir / resource / filename
                out.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(dest, out)
        gradle("jar", "sourcesJar", "publishJvmPublicationToStagingRepository")
        version = tomllib.loads((ROOT / "fatoora-ffi/Cargo.toml").read_text())[
            "package"
        ]["version"]
        package = PROJECT / f"build/libs/fatoority-{version}.jar"
        verify(
            package, {p.relative_to(staging).as_posix(): p for p in staging.glob("*/*")}
        )
    print(f"Verified package: {package} ({package.stat().st_size:,} bytes)")


if __name__ == "__main__":
    main()
