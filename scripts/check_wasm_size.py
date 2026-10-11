#!/usr/bin/env python3
"""Report WASM section/transfer sizes and enforce the JS artifact size budget."""
import argparse
from collections import Counter
import gzip
import json
from pathlib import Path


def leb(data, offset):
    value = shift = 0
    while True:
        byte = data[offset]
        offset += 1
        value |= (byte & 127) << shift
        if byte < 128:
            return value, offset
        shift += 7
        if shift > 35:
            raise ValueError('invalid WASM section length')


def strip_binding_metadata(path):
    """Drop descriptors used only by wasm-bindgen tooling, after removing its exports."""
    data = Path(path).read_bytes()
    output = bytearray(data[:8])
    offset = 8
    while offset < len(data):
        kind = data[offset]
        length, start = leb(data, offset + 1)
        end = start + length
        if end > len(data):
            raise ValueError('truncated WASM section')
        discard = False
        if kind == 0:
            size, pos = leb(data, start)
            discard = data[pos:pos + size] == b'__wasm_bindgen_unstable'
        if not discard:
            output.extend(data[offset:end])
        offset = end
    Path(path).write_bytes(output)


def measure(path):
    data = Path(path).read_bytes()
    if data[:8] != b'\0asm\x01\0\0\0':
        raise ValueError('expected WASM version 1')
    names = {1: 'type', 2: 'import', 3: 'function', 4: 'table', 5: 'memory',
             6: 'global', 7: 'export', 8: 'start', 9: 'element', 10: 'code',
             11: 'data', 12: 'data_count', 13: 'tag'}
    sections = Counter()
    offset = 8
    while offset < len(data):
        kind = data[offset]
        length, start = leb(data, offset + 1)
        if start + length > len(data):
            raise ValueError('truncated WASM section')
        name = names.get(kind, f'unknown_{kind}')
        if kind == 0:
            size, pos = leb(data, start)
            name = 'custom:' + data[pos:pos + size].decode('utf8')
        sections[name] += length
        offset = start + length
    return {'file': str(path), 'bytes': len(data),
            'gzip_bytes': len(gzip.compress(data, compresslevel=9, mtime=0)),
            'section_payload_bytes': dict(sections)}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('artifact', nargs='?', default='bindings/js/fatoora_ffi.wasm')
    parser.add_argument('--max-bytes', type=int, default=4_000_000)
    parser.add_argument('--max-gzip-bytes', type=int, default=1_800_000)
    parser.add_argument('--json', action='store_true')
    args = parser.parse_args()
    result = measure(args.artifact)
    if args.json:
        print(json.dumps(result, indent=2))
    else:
        print(f"WASM: {result['bytes']:,} bytes; gzip: {result['gzip_bytes']:,} bytes")
        for name, size in sorted(result['section_payload_bytes'].items(), key=lambda item: -item[1]):
            print(f'  {name}: {size:,}')
    if result['bytes'] > args.max_bytes:
        raise SystemExit(f"WASM exceeds {args.max_bytes:,}-byte budget")
    if result['gzip_bytes'] > args.max_gzip_bytes:
        raise SystemExit(f"Gzipped WASM exceeds {args.max_gzip_bytes:,}-byte budget")


if __name__ == '__main__':
    main()
