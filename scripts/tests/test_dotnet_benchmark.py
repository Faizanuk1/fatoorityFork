"""SDK replacement must never inherit files from a previous benchmark setup."""

import importlib.util
from pathlib import Path
import tempfile
import unittest
from zipfile import ZipFile

SPEC = importlib.util.spec_from_file_location(
    "dotnet_benchmark_setup",
    Path(__file__).resolve().parents[2] / "bench/dotnet/setup.py",
)
setup = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(setup)


class BenchmarkSetupTests(unittest.TestCase):
    def make_archive(self, path, extra=False, omit=None):
        prefix = "sdk/Lib/Dot-Net8/DLL/Zatca.EInvoice.SDK/net8.0/"
        files = {
            prefix + name: b"new"
            for name in (
                "ZATCA.EInvoice.SDK.dll", "ZATCA.EInvoice.SDK.Contracts.dll",
                "runtimes/linux/lib/net8.0/IKVM.Runtime.dll",
                "runtimes/linux/lib/net8.0/IKVM.Java.dll",
                "ikvm/linux-x64/bin/libjvm.so",
            )
        }
        for name in ("CEN-EN16931-UBL.xsl", "20210819_ZATCA_E-invoice_Validation_Rules.xsl"):
            files["sdk/Data/Rules/Schematrons/" + name] = b"rules"
        if extra:
            files[prefix + "obsolete.dll"] = b"old"
            files[prefix + "ikvm/linux-x64/obsolete.so"] = b"old"
            files["sdk/Data/Rules/obsolete.xsl"] = b"old"
        with ZipFile(path, "w") as archive:
            for name, data in files.items():
                if omit is None or not name.endswith(omit):
                    archive.writestr(name, data)

    def test_replacement_removes_old_sdk_runtime_rules_and_build_outputs(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            work = root / "work"
            work.mkdir()
            archive = root / "sdk.zip"
            self.make_archive(archive, extra=True)
            setup.prepare_sdk(archive, work)
            for relative in ("bin/Release/net8.0/obsolete.dll", "obj/stale.cache", "results/keep.json"):
                path = work / relative
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text("old")
            self.make_archive(archive)
            official = setup.prepare_sdk(archive, work)
            self.assertEqual((official / "ZATCA.EInvoice.SDK.dll").read_bytes(), b"new")
            self.assertFalse(list(official.rglob("obsolete*")))
            self.assertFalse((work / "bin").exists())
            self.assertFalse((work / "obj").exists())
            self.assertEqual((work / "results/keep.json").read_text(), "old")

    def test_incomplete_archive_cannot_borrow_previous_sdk_or_rules(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            work = root / "work"
            work.mkdir()
            archive = root / "sdk.zip"
            self.make_archive(archive, extra=True)
            setup.prepare_sdk(archive, work)
            for missing in ("ZATCA.EInvoice.SDK.dll", "CEN-EN16931-UBL.xsl"):
                with self.subTest(missing=missing):
                    self.make_archive(archive, omit=missing)
                    with self.assertRaisesRegex(ValueError, "Incomplete official SDK"):
                        setup.prepare_sdk(archive, work)
                    self.assertTrue((work / "official/obsolete.dll").is_file())
                    self.assertFalse(list(work.glob("sdk-*")))
            with ZipFile(archive, "w") as empty:
                empty.writestr("unrelated.txt", "not an SDK")
            with self.assertRaisesRegex(ValueError, "missing ZATCA.EInvoice.SDK.dll"):
                setup.prepare_sdk(archive, work)


if __name__ == "__main__":
    unittest.main()
