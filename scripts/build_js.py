#!/usr/bin/env python3
"""Build the WASM artifact and stage npm license notices. Generate bindings separately."""
from pathlib import Path
import os
import shutil
import subprocess
ROOT = Path(__file__).resolve().parents[1]
subprocess.run(['cargo', 'build', '--locked', '--release', '-p', 'fatoora-ffi', '--target', 'wasm32-unknown-unknown'], cwd=ROOT, check=True)
target = Path(os.environ.get('CARGO_TARGET_DIR', ROOT / 'target'))
if not target.is_absolute():
    target = ROOT / target
shutil.copyfile(target / 'wasm32-unknown-unknown/release/fatoora_ffi.wasm', ROOT / 'bindings/js/fatoora_ffi.wasm')
subprocess.run(['node', str(ROOT / 'bindings/js/optimize-wasm.mjs'), str(ROOT / 'bindings/js/fatoora_ffi.wasm')], check=True)
for name in ('LICENSE-MIT', 'LICENSE-APACHE', 'THIRD_PARTY_NOTICES.md'):
    shutil.copyfile(ROOT / name, ROOT / 'bindings/js' / name)

notices = ROOT / 'bindings/js/licenses/zatca'
notices.mkdir(parents=True, exist_ok=True)
for name in ('LICENSE-LGPL-3.0.txt', 'LICENSE-GPL-3.0.txt', 'NOTICE.md'):
    shutil.copyfile(ROOT / 'fatoora-core/tests/fixtures/business-rules' / name, notices / name)
