# Fatoority JavaScript / TypeScript

ES modules backed by Diplomat 0.16.1 and WebAssembly. Supports Node 22+ and modern browsers with WebAssembly, Web Crypto, FinalizationRegistry and top-level await.

Build from the repository root:

```sh
rustup target add wasm32-unknown-unknown
cargo install diplomat-tool --version 0.16.1 --locked
npm ci --prefix bindings/js
python3 scripts/generate_bindings.py
python3 scripts/build_js.py
npm run check --prefix bindings/js
node bindings/js/tests/runtime.test.mjs
python3 scripts/check_js_package.py
```

The package is built locally with `npm pack` in `bindings/js`; this PR does not configure npm publication.

```js
import { Config, SigningKey, CsidCompliance, ZatcaClient } from 'fatoority';

const key = SigningKey.generate();
const config = Config.new_(0); // 0: nonproduction, 1: simulation, 2: production
const client = new ZatcaClient(config);
const compliance = CsidCompliance.create(0, 'request-id', 'token', 'secret');
const production = await client.postCcsidForPcsid(compliance, {
  signal: AbortSignal.timeout(30_000),
});
```

API methods prepare requests and decode responses in Rust; the facade calls `fetch`. Inject a fetch implementation with `new ZatcaClient(config, {fetch})`. Requests refuse redirects. HTTP errors go to Rust decoding, and a successful invoice HTTP response still requires `response.ensureAccepted()` or inspection of `response.outcome()`.

For your own transport, import `ZatcaClient` from `fatoority/raw`, call a `prepare*` method, and read `method()`, `url()`, `headersJson()` (an array of header pairs), and `body().asSlice()`. Complete with `request.decode(status, bytes)`, or use `sendPrepared(request, {fetch, signal})`. The request owns its data and the operation/environment needed for decoding. It contains authorization headers and must not be logged.

The generated API preserves Rust names in camelCase; a Rust `new` constructor is `new_`. Decimal arguments and outputs are strings, and invoice counters are `bigint`. Optional strings use `null`, distinct from `''`. Consuming operations such as `build()`, signing and `intoXml()` reject reuse. Generated owners are released by FinalizationRegistry; do not manually free pointers or share handles between workers. Byte slices returned by the generated JS are copies. The facade snapshots outgoing bytes before awaiting the network.

Generated Rust errors are JavaScript `Error` objects whose `cause` is a `BindingError`. Inspect `error.cause.code()`, `.message()` and `.detailsJson()`. Network, cancellation and response-body read failures throw `TransportError`, retaining the original cause and, when available, `httpStatus`.

`Xml.validateZatca(config, xml, optionsJson)` supplies the host clock when `evaluated_at` is absent. An explicit timestamp makes validation reproducible. The raw API requires `evaluated_at` on WASM. Filesystem helpers are excluded from JS; read files in the host and pass their contents.

The browser loader resolves `fatoora_ffi.wasm` relative to the package modules. Deploy the generated modules and WASM together, or configure the bundler to copy the WASM at that URL. Direct gateway requests depend on the gateway's CORS policy. Keep signing keys and credentials in the host environment appropriate to your application.

The WASM build uses getrandom's custom backend with Web Crypto. `xsd-schema` transitively enables Chrono's wasm-bindgen features; the pinned Binaryen step removes unused descriptor exports and maps its retained UTF-8 throw helper to Diplomat's equivalent host import. The build fails if any unsupported import remains, including a JS clock or wasm-bindgen object runtime. Native transport and clock behavior use the normal Rust dependencies.
