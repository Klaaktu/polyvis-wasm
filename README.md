[![Nightly release](https://github.com/Klaaktu/polyvis-wasm/actions/workflows/nightly.yml/badge.svg)](https://github.com/Klaaktu/polyvis-wasm/actions/workflows/nightly.yml)

[GeoRust](https://github.com/georust/geo) wrapper for [my homework](https://github.com/mtel0004/FIT3162).

## Build
`wasm-pack build --target web`

## How to use
```Javascript
import * as wasm from "polyvis_wasm.js";
await wasm.default();
const instance = wasm.new_session();
```
There are some convenience functions and instance methods. See [polyvis_wasm.d.ts](https://github.com/Klaaktu/polyvis-wasm/blob/pkg/polyvis_wasm.d.ts).

See [mtel0004/FIT3162/src/utils.ts](https://github.com/mtel0004/FIT3162/blob/main/src/utils.ts) for adapting types between JS and the WASM module (JS `number` type is 64-bit float).

## Notes
Importing YAML save is not supported, due to [lack](https://github.com/acatton/serde-yaml-ng?tab=readme-ov-file#update-july-2025) of a maintained & popular YAML library. Export is supported (YAML is more human-readable).

## Considerations
- Avoid passing a lot of data between JS and WASM, because [it's slow](https://rustwasm.github.io/docs/book/game-of-life/implementing.html#interfacing-rust-and-javascript). May need to keep 2 copies of the polygon list.
- Deno has HTTP import but it can't be used with WASM yet. Currently the web standard for loading WASM is fetch & instantiate `new URL('something.wasm', import.meta.url)`, so the dependency is runtime instead of a regular import, which Deno doesn't support on [principle](https://github.com/denoland/deno/issues/5987#issuecomment-637253490). There is a draft standard ["ES Module Integration"](https://github.com/WebAssembly/esm-integration/tree/main/proposals/esm-integration) which Deno already supports and there are polyfills ([example](https://github.com/Menci/vite-plugin-wasm)) for the browser, but the tooling is awkward: [`wasmbuild` locks dependency versions of your project](https://github.com/denoland/wasmbuild/blob/main/lib/versions.ts), [`vite-plugin-wasm` seems to be generating its own glue code](https://github.com/Menci/vite-plugin-wasm/blob/main/src/wasm-parser.ts), `wasm-pack`'s `deno` target is using the slowest `WebAssembly.instantiate()` method instead of Deno's ESM Integration.
- Watch for the [fixed-size arrays feature](https://github.com/wasm-bindgen/wasm-bindgen/issues/122) so we can ditch the wrapper structs.

## The spiral of rationale
1. Use [iShape-js](https://github.com/iShape-Rust/iShape-js), a wasm build of [iOverlay](https://github.com/iShape-Rust/iOverlay) in a Javascript web app.
2. Switch to GeoRust and write a wrapper. Because iOverlay doesn't have all the features we need (e.g. calculate area, convex check) and GeoRust doesn't compile to WASM. GeoRust is the most popular Rust geometry library and uses iOverlay as a dependency.
3. Write a full backend. Because passing (copying) a lot of data is slow.

Further reading: https://kylebarron.dev/blog/geos-wasm (p.s. [GEOS](https://libgeos.org/) says it's a port of JTS, not the other way around.)

## To do
- [ ] Use `std::sync::LazyLock` instead of instance? (Not recommended in general)
- [ ] Use WASM's linear memory in JS?
- [ ] Import JS functions to Rust to avoid data copy and using structs to pass data.
- [ ] Multi-threading?
