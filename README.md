[GeoRust](https://github.com/georust/geo) wrapper for [my homework](https://github.com/mtel0004/FIT3162).

## Build
`wasm-pack build --target web`

## How to use
```
// Call new_session() in JS
```

## Notes
Importing YAML save is not supported, due to [lack](https://github.com/acatton/serde-yaml-ng?tab=readme-ov-file#update-july-2025) of a maintained & popular YAML library. Export is supported (YAML is more human-readable).

## Rationale
The original plan was to use [iShape-js](https://github.com/iShape-Rust/iShape-js), a wasm build of [iOverlay](https://github.com/iShape-Rust/iOverlay) in a Javascript web app. However, iOverlay doesn't have all the features we need, e.g. calculate area, convex check. So I switched to GeoRust, which uses iOverlay as a dependency.

However, it doesn't seem that GeoRust can be compiled directly to wasm, instead you need to use it in your project and compile your project to wasm instead, even though I only need functions that are already available.

Further reading: https://kylebarron.dev/blog/geos-wasm (p.s. [GEOS](https://libgeos.org/) says it's a port of JTS, not the other way around.)

## Considerations
Avoid passing a lot of data between JS and WASM, because [it's slow](https://rustwasm.github.io/docs/book/game-of-life/implementing.html#interfacing-rust-and-javascript). May need to keep 2 copies of the polygon list.

## To do
- [ ] Use WASM's linear memory in JS?
- [ ] Import JS functions to Rust to avoid data copy?
- [ ] Test edge cases: empty list to iou()
- [ ] Dependabot
- [ ] Can JS handle NaN (div by 0 in float)?
- [ ] Put the export function in JS instead, except the serialization. I don't like all the explicit error propagating. Those errors likely won't be fatal in JS anyway.
