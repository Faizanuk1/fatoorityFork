// xsd-schema enables Chrono's default features transitively. wasm-bindgen's
// exported descriptors keep unused JS glue alive even though our WASM API never
// calls the clock. Remove only that tooling surface, then perform dead-code
// elimination. Reject unsupported host imports after normalizing the throw ABI.
import binaryen from 'binaryen';
import { readFileSync, writeFileSync } from 'node:fs';
const path = process.argv[2];
const module = binaryen.readBinary(readFileSync(path));
module.setFeatures(binaryen.Features.All);
const exports = [];
for (let i = 0; i < module.getNumExports(); i++) exports.push(binaryen.getExportInfo(module.getExportByIndex(i)).name);
for (const name of exports) {
  if (name.startsWith('__wbindgen_') || name.startsWith('__externref_')) module.removeExport(name);
}
module.runPasses(['remove-unused-module-elements']);
// wasm-bindgen's retained exception helper has the same UTF-8 ptr/length
// ABI as Diplomat's throw helper. Normalize that one import; no JS object,
// Date, externref, or wasm-bindgen runtime is supplied.
const functions = [];
for (let i = 0; i < module.getNumFunctions(); i++) functions.push(binaryen.getFunctionInfo(module.getFunctionByIndex(i)));
for (const fn of functions) {
  if (fn.module === '__wbindgen_placeholder__' && /^__wbg___wbindgen_throw_[a-f0-9]+$/.test(fn.base)) {
    if (fn.params !== binaryen.createType([binaryen.i32, binaryen.i32]) || fn.results !== binaryen.none) {
      throw new Error('Unexpected wasm-bindgen throw ABI');
    }
    module.removeFunction(fn.name);
    module.addFunctionImport(fn.name, 'env', 'diplomat_throw_error_js', fn.params, fn.results);
  }
}
if (!module.validate()) throw new Error('Invalid WASM after removing unused tooling exports');
const bytes = module.emitBinary();
module.dispose();
const imports = WebAssembly.Module.imports(new WebAssembly.Module(bytes));
const allowed = new Set(['fatoora_random_fill', 'diplomat_throw_error_js', ...['debug', 'error', 'info', 'log', 'warn'].map(level => `diplomat_console_${level}_js`)]);
for (const item of imports) {
  if (item.module !== 'env' || !allowed.has(item.name) || item.kind !== 'function') {
    throw new Error(`Unsupported WASM host import: ${item.module}.${item.name}`);
  }
}
writeFileSync(path, bytes);
