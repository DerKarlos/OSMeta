/* tslint:disable */
/* eslint-disable */
/**
* Represents a point using the Cartesian system of coordinates.
*/
export class CartesianPoint {
  free(): void;
/**
* @param {number} x
* @param {number} y
* @param {number} z
*/
  constructor(x: number, y: number, z: number);
/**
* Returns the equivalent [`CartesianPoint`] of the given [`GeographicPoint`]
* @param {GeographicPoint} point
* @returns {CartesianPoint}
*/
  static from_geographic(point: GeographicPoint): CartesianPoint;
/**
* @returns {number}
*/
  x(): number;
/**
* @param {number} x
*/
  set_x(x: number): void;
/**
* @returns {number}
*/
  y(): number;
/**
* @param {number} y
*/
  set_y(y: number): void;
/**
* @returns {number}
*/
  z(): number;
/**
* @param {number} z
*/
  set_z(z: number): void;
/**
* Returns the distance between self and the given point.
* @param {CartesianPoint} other
* @returns {number}
*/
  distance(other: CartesianPoint): number;
/**
* Performs the cartesian product between self and the given point.
* @param {CartesianPoint} other
* @returns {CartesianPoint}
*/
  cross(other: CartesianPoint): CartesianPoint;
/**
* Rotates self in theta radians about the edge passing by the origin and the given axis point.
* @param {CartesianPoint} axis
* @param {number} theta
*/
  rotate(axis: CartesianPoint, theta: number): void;
}
/**
* Represents a point using the geographic system of coordinates.
*/
export class GeographicPoint {
  free(): void;
/**
* @param {number} longitude
* @param {number} latitude
* @param {number} altitude
* @returns {GeographicPoint}
*/
  static new(longitude: number, latitude: number, altitude: number): GeographicPoint;
/**
* Returns the equivalent [`GeographicPoint`] of the given [`CartesianPoint`]
* @param {CartesianPoint} point
* @returns {GeographicPoint}
*/
  static from_cartesian(point: CartesianPoint): GeographicPoint;
/**
* Calls set_longitude on self and returns it.
* @param {number} value
* @returns {GeographicPoint}
*/
  with_longitude(value: number): GeographicPoint;
/**
* Calls set_latitude on self and returns it.
* @param {number} value
* @returns {GeographicPoint}
*/
  with_latitude(value: number): GeographicPoint;
/**
* Calls set_altitude on self and returns it.
* @param {number} value
* @returns {GeographicPoint}
*/
  with_altitude(value: number): GeographicPoint;
/**
* Sets the given longitude (in radiants) to the point.
*
* ## Definition
* Since the longitude of a point on a sphere is the angle east (positive) or
* west (negative) in reference of the maridian zero, the longitude value must
* be in the range __[-π, +π)__. Any other value will be recomputed in order
* to set its equivalent inside the range.
*
* ### Longitude adjustment
* Both boundaries of the longitude range are consecutive, which means that
* overflowing one is the same as continuing from the other in the same
* direction.
*
* ## Example
* ```
* use globe_rs::GeographicPoint;
* use std::f64::consts::PI;
* use float_cmp::approx_eq;
*
* let mut point = GeographicPoint::default();
* point.set_longitude(PI + 1_f64);
*
* assert!(approx_eq!(f64, point.longitude(), -PI + 1_f64, ulps = 2));
* ```
* @param {number} value
*/
  set_longitude(value: number): void;
/**
* Sets the given latitude (in radiants) to the point.
*
* ## Definition
* Since the latitude of a point on a sphere is the angle between the
* equatorial plane and the straight line that passes through that point and
* through the center of the sphere, the latitude value must be in the range
* __\[-π/2, +π/2\]__. Any other value will be recomputed in order to set its
* equivalent inside the range. Notice that this action may recompute the
* longitude as well.
*
* ### Latitude adjustment
* Overflowing any of both boundaries of the latitude range behaves like
* moving away from that point and getting closer to the oposite one.
*
* ### Longitude adjustment
* Geometrically speaking, meridians are half of a circle going from the north
* pole to the south one. The position of each meridian in the perimeter of
* the sphere (horizontal axis) is set by the longitude itself. However, this
* value may change when the latitude overflows its normalized range. This
* happen since exceeding any of its established limits means moving from one
* to the other half of the circle on which the meridian is drawn. And
* therefore, the longitude gets increased by exactly `π` radiants.
*
* Of course, this mutation on the longitude only applies when the overflow of
* the latitude is not enough to complete a full lap. If it is, the longitude
* does not change at all.
*
* ## Example
* ```
* use globe_rs::GeographicPoint;
* use std::f64::consts::PI;
* use float_cmp::approx_eq;
*
* let mut point = GeographicPoint::default();
* point.set_latitude(-5. * PI / 4.);
*
* assert!(approx_eq!(f64, point.latitude(), PI / 4., ulps = 2));
* assert!(approx_eq!(f64, point.longitude(), -PI, ulps = 2));
* ```
* @param {number} value
*/
  set_latitude(value: number): void;
/**
* Sets the given altitude to the point.
* @param {number} value
*/
  set_altitude(value: number): void;
/**
* Returns the longitude (in radiants) of the point.
* @returns {number}
*/
  longitude(): number;
/**
* Returns the latitude (in radiants) of the point.
* @returns {number}
*/
  latitude(): number;
/**
* Returns the altitude (in radiants) of the point.
* @returns {number}
*/
  altitude(): number;
/**
* Returns the result of dividing `π` to the longitude of the point, resulting
* in a value in the range __[-1.0, 1.0)__
* @returns {number}
*/
  long_ratio(): number;
/**
* Returns the result of dividing `π/2` to the latitude of the point, resulting
* in a value in the range __\[-1.0, 1.0\]__
* @returns {number}
*/
  lat_ratio(): number;
/**
* Computes the [great-circle distance](https://en.wikipedia.org/wiki/Great-circle_distance) from self to the given point (in radiants).
* @param {GeographicPoint} other
* @returns {number}
*/
  distance(other: GeographicPoint): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly main: (a: number, b: number) => number;
  readonly __wbg_cartesianpoint_free: (a: number) => void;
  readonly cartesianpoint_new: (a: number, b: number, c: number) => number;
  readonly cartesianpoint_from_geographic: (a: number) => number;
  readonly cartesianpoint_x: (a: number) => number;
  readonly cartesianpoint_set_x: (a: number, b: number) => void;
  readonly cartesianpoint_y: (a: number) => number;
  readonly cartesianpoint_set_y: (a: number, b: number) => void;
  readonly cartesianpoint_z: (a: number) => number;
  readonly cartesianpoint_set_z: (a: number, b: number) => void;
  readonly cartesianpoint_distance: (a: number, b: number) => number;
  readonly cartesianpoint_cross: (a: number, b: number) => number;
  readonly cartesianpoint_rotate: (a: number, b: number, c: number) => void;
  readonly __wbg_geographicpoint_free: (a: number) => void;
  readonly geographicpoint_new: (a: number, b: number, c: number) => number;
  readonly geographicpoint_from_cartesian: (a: number) => number;
  readonly geographicpoint_with_longitude: (a: number, b: number) => number;
  readonly geographicpoint_with_latitude: (a: number, b: number) => number;
  readonly geographicpoint_with_altitude: (a: number, b: number) => number;
  readonly geographicpoint_set_longitude: (a: number, b: number) => void;
  readonly geographicpoint_set_latitude: (a: number, b: number) => void;
  readonly geographicpoint_set_altitude: (a: number, b: number) => void;
  readonly geographicpoint_longitude: (a: number) => number;
  readonly geographicpoint_latitude: (a: number) => number;
  readonly geographicpoint_altitude: (a: number) => number;
  readonly geographicpoint_long_ratio: (a: number) => number;
  readonly geographicpoint_lat_ratio: (a: number) => number;
  readonly geographicpoint_distance: (a: number, b: number) => number;
  readonly wgpu_compute_pass_set_pipeline: (a: number, b: number) => void;
  readonly wgpu_compute_pass_set_bind_group: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_compute_pass_set_push_constant: (a: number, b: number, c: number, d: number) => void;
  readonly wgpu_compute_pass_insert_debug_marker: (a: number, b: number, c: number) => void;
  readonly wgpu_compute_pass_push_debug_group: (a: number, b: number, c: number) => void;
  readonly wgpu_compute_pass_pop_debug_group: (a: number) => void;
  readonly wgpu_compute_pass_write_timestamp: (a: number, b: number, c: number) => void;
  readonly wgpu_compute_pass_begin_pipeline_statistics_query: (a: number, b: number, c: number) => void;
  readonly wgpu_compute_pass_end_pipeline_statistics_query: (a: number) => void;
  readonly wgpu_compute_pass_dispatch_workgroups: (a: number, b: number, c: number, d: number) => void;
  readonly wgpu_compute_pass_dispatch_workgroups_indirect: (a: number, b: number, c: number) => void;
  readonly wgpu_render_bundle_set_pipeline: (a: number, b: number) => void;
  readonly wgpu_render_bundle_set_bind_group: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_set_vertex_buffer: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_set_push_constants: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_draw: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_draw_indexed: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly wgpu_render_bundle_draw_indirect: (a: number, b: number, c: number) => void;
  readonly wgpu_render_bundle_draw_indexed_indirect: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_set_pipeline: (a: number, b: number) => void;
  readonly wgpu_render_pass_set_bind_group: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_set_vertex_buffer: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_set_push_constants: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_draw: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_draw_indexed: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly wgpu_render_pass_draw_indirect: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_draw_indexed_indirect: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_multi_draw_indirect: (a: number, b: number, c: number, d: number) => void;
  readonly wgpu_render_pass_multi_draw_indexed_indirect: (a: number, b: number, c: number, d: number) => void;
  readonly wgpu_render_pass_multi_draw_indirect_count: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly wgpu_render_pass_multi_draw_indexed_indirect_count: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly wgpu_render_pass_set_blend_constant: (a: number, b: number) => void;
  readonly wgpu_render_pass_set_scissor_rect: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_set_viewport: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly wgpu_render_pass_set_stencil_reference: (a: number, b: number) => void;
  readonly wgpu_render_pass_insert_debug_marker: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_push_debug_group: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_pop_debug_group: (a: number) => void;
  readonly wgpu_render_pass_write_timestamp: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_begin_pipeline_statistics_query: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_end_pipeline_statistics_query: (a: number) => void;
  readonly wgpu_render_pass_execute_bundles: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_set_index_buffer: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_set_index_buffer: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_pop_debug_group: (a: number) => void;
  readonly wgpu_render_bundle_insert_debug_marker: (a: number, b: number) => void;
  readonly wgpu_render_bundle_push_debug_group: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h200cc064afa7c6a8: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he11a1f5f47cf7670: (a: number, b: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h15600046f21fb94b: (a: number, b: number, c: number) => void;
  readonly wasm_bindgen__convert__closures__invoke0_mut__hd93c6a9c7690dee5: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures__invoke1_mut__ha49b025fdffdd60e: (a: number, b: number, c: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
