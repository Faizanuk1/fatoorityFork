// Host imports for the Diplomat artifact. Copied by generate_bindings.py.
import { readString8 } from './diplomat-runtime.mjs';

let wasm;
let crypto = globalThis.crypto;
const url = new URL('../fatoora_ffi.wasm', import.meta.url);
const isNode = typeof process !== 'undefined' && Boolean(process.versions?.node);
if (isNode && !crypto) crypto = (await import('node:crypto')).webcrypto;
const env = {
  fatoora_random_fill(ptr, len) {
    try {
      // Web Crypto limits each getRandomValues call to 65536 bytes.
      for (let offset = 0; offset < len; offset += 65536) {
        crypto.getRandomValues(new Uint8Array(wasm.memory.buffer, ptr + offset, Math.min(65536, len - offset)));
      }
      return 0;
    } catch { return 1; }
  },
  diplomat_throw_error_js(ptr, len) { throw new Error(readString8(wasm, ptr, len)); },
};
for (const level of ['debug', 'error', 'info', 'log', 'warn']) {
  env[`diplomat_console_${level}_js`] = (ptr, len) => console[level](readString8(wasm, ptr, len));
}
const bytes = isNode ? await (await import('node:fs/promises')).readFile(url)
  : await (await fetch(url)).arrayBuffer();
const loaded = await WebAssembly.instantiate(bytes, { env });
wasm = loaded.instance.exports;
wasm.diplomat_init();
export default wasm;
