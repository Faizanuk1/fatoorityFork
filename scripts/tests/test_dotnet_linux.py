import sys
from pathlib import Path
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from check_dotnet_linux import check_versions


class LinuxBaselineTests(unittest.TestCase):
    def test_numeric_versions_up_to_baseline(self):
        self.assertEqual(check_versions(
            "Name: GLIBC_2.9 Flags: none\nName: GLIBC_2.28 Flags: none\n"
            "Name: GCC_3.0 Flags: none\nName: GLIBC_2.2.5 Flags: none"
        ), (2, 28))

    def test_newer_glibc_is_rejected(self):
        with self.assertRaisesRegex(ValueError, "exceeds"):
            check_versions("Name: GLIBC_2.34 Flags: none")

    def test_unknown_and_missing_requirements_fail_closed(self):
        for output in ("Name: GLIBC_ABI_DT_RELR Flags: none", "no versions"):
            with self.subTest(output=output), self.assertRaises(ValueError):
                check_versions(output)
