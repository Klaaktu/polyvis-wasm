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

See [mtel0004/FIT3162/src/utils.ts](https://github.com/mtel0004/FIT3162/blob/main/src/utils.ts) for adapting types between JS and the WASM module.

## Notes
Importing YAML save is not supported, due to [lack](https://github.com/acatton/serde-yaml-ng?tab=readme-ov-file#update-july-2025) of a maintained & popular YAML library. Export is supported (YAML is more human-readable).

## Considerations
Avoid passing a lot of data between JS and WASM, because [it's slow](https://rustwasm.github.io/docs/book/game-of-life/implementing.html#interfacing-rust-and-javascript). May need to keep 2 copies of the polygon list.

## The spiral of rationale
1. Use [iShape-js](https://github.com/iShape-Rust/iShape-js), a wasm build of [iOverlay](https://github.com/iShape-Rust/iOverlay) in a Javascript web app.
2. Switch to GeoRust and write a wrapper. Because iOverlay doesn't have all the features we need (e.g. calculate area, convex check) and GeoRust doesn't compile to WASM. GeoRust is the most popular Rust geometry library and uses iOverlay as a dependency.
3. Write a full backend. Because passing (copying) a lot of data is slow.

Further reading: https://kylebarron.dev/blog/geos-wasm (p.s. [GEOS](https://libgeos.org/) says it's a port of JTS, not the other way around.)

## To do
- [ ] Use WASM's linear memory in JS?
- [ ] Import JS functions to Rust to avoid data copy?
- [ ] Test edge cases: empty list to iou()
- [x] Dependabot
- [x] Can JS handle NaN (div by 0 in float)? EDIT: Div by 0 now returns Error.
- [x] Put the export function in JS instead, except the serialization. I don't like all the explicit error propagating. Those errors likely won't be fatal in JS anyway.
- [ ] Multi-threading?
- [ ] Graph data structure for intersections between pairs.
