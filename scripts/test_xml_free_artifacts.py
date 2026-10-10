"""Focused regressions for native artifact import inspection."""
from pathlib import Path
import struct
import tempfile
import unittest

from check_xml_free_artifacts import inspect, pe_imports


def pe(dependency):
    data = bytearray(0x300)
    data[:2] = b"MZ"
    struct.pack_into("<I", data, 0x3C, 0x80)
    data[0x80:0x84] = b"PE\0\0"
    struct.pack_into("<H", data, 0x86, 1)
    struct.pack_into("<H", data, 0x94, 240)
    struct.pack_into("<H", data, 0x98, 0x20B)
    struct.pack_into("<II", data, 0x110, 0x1000, 40)
    struct.pack_into("<IIII", data, 0x188 + 8, 0x100, 0x1000, 0x100, 0x200)
    struct.pack_into("<IIIII", data, 0x200, 0x1080, 0, 0, 0x1060, 0x1080)
    name = dependency.encode("ascii") + b"\0"
    data[0x260:0x260 + len(name)] = name
    return data


class ArtifactTests(unittest.TestCase):
    def test_pe_reads_imports_and_rejects_libxml_names(self):
        self.assertEqual(pe_imports(pe("KERNEL32.dll")), ["KERNEL32.dll"])
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "fatoora_ffi.dll"
            path.write_bytes(pe("KERNEL32.dll"))
            self.assertTrue(inspect(path))
            for name in ("libxml2.dll", "XML2.DLL"):
                path.write_bytes(pe(name))
                with self.assertRaisesRegex(ValueError, "libxml2 dependency"):
                    inspect(path)

    def test_pe_reads_delay_imports(self):
        data = pe("libxml2.dll")
        data[0x280:0x28C] = b"libxml2.dll\0\0"
        data[0x260:0x26C] = bytes(12)
        struct.pack_into("<I", data, 0x20C, 0x1080)
        struct.pack_into("<I", data, 0x104, 16)
        struct.pack_into("<II", data, 0x108 + 13 * 8, 0x1028, 64)
        struct.pack_into("<IIIIIIII", data, 0x228, 1, 0x1080, 0, 0, 0, 0, 0, 0)
        self.assertEqual(pe_imports(data), ["libxml2.dll", "libxml2.dll"])

    def test_pe_rejects_unmapped_import_directory(self):
        data = pe("KERNEL32.dll")
        struct.pack_into("<I", data, 0x110, 0x9000)
        with self.assertRaisesRegex(ValueError, "unmapped PE RVA"):
            pe_imports(data)


if __name__ == "__main__":
    unittest.main()
