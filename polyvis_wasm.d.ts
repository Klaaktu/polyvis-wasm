/* tslint:disable */
/* eslint-disable */
export function new_session(): Instance;
export function deserialize_session(text: string): Instance;
export function is_convex(points: Coord2D[]): boolean;
export enum TextFormat {
  JSON = 0,
  YAML = 1,
}
export class Coord2D {
  private constructor();
  free(): void;
  0: number;
  1: number;
}
export class Instance {
  free(): void;
  constructor();
  add_polygon(points: Coord2D[], color: number): bigint;
  iou(ids: BigUint64Array): number;
  intersection(ids: BigUint64Array): Coord2D[];
  serialize(format: TextFormat): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly new_session: () => number;
  readonly deserialize_session: (a: number, b: number) => [number, number, number];
  readonly __wbg_coord2d_free: (a: number, b: number) => void;
  readonly __wbg_get_coord2d_0: (a: number) => number;
  readonly __wbg_set_coord2d_0: (a: number, b: number) => void;
  readonly __wbg_get_coord2d_1: (a: number) => number;
  readonly __wbg_set_coord2d_1: (a: number, b: number) => void;
  readonly is_convex: (a: number, b: number) => number;
  readonly __wbg_instance_free: (a: number, b: number) => void;
  readonly instance_new: () => number;
  readonly instance_add_polygon: (a: number, b: number, c: number, d: number) => [bigint, number, number];
  readonly instance_iou: (a: number, b: number, c: number) => [number, number, number];
  readonly instance_intersection: (a: number, b: number, c: number) => [number, number, number, number];
  readonly instance_serialize: (a: number, b: number) => [number, number, number, number];
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __externref_drop_slice: (a: number, b: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
