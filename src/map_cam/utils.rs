// o2w utils.rs
// pub fn don't need to be used:
#![allow(dead_code)]

pub type ScenePos = glam::Vec3; //type Vec3 = Vector3<f32>;   with x,y,z   https://docs.rs/cgmath/0.17.0/cgmath/struct.Vector3.html

pub static PI: f32 = std::f32::consts::PI;

pub static DEFAULT_LAT: f32 = 48.565837; //  x? 4402 35216..23 48.57478991x 1111?0.0 1615.6 795.652 - 820.450
pub static DEFAULT_LON: f32 = 13.453166; //  z? 2828 22624..31 13.447265625  73512.4 1616.1 305.071 -1310.565

/** Camera view direction to nord (camera alpha-90) degrees */
pub static DEFAULT_DIR: f32 = -32.;

/** Camera view slightly down (camera beta) degrees */
pub static DEFAULT_VIEW: f32 = -12.;

/** Camera distance to the view/rotating point in meters */
pub static DEFAULT_RADIUS: f32 = 375.;  // 547 -- F4 zoom 17/18/19 is about 1500/750/375 meter radius

/** Height of the view point (eyes height of a small person) in meters */
pub static DEFAULT_HEIGHT: f32 = 1.6;

/** Camera field of view angle (FoV) in degrees */
pub static DEFAULT_FOV: f32 = 23.; // F4=    OSMgo=23?


/** Factor to calculate meters from gps geo.decimals (latitude, Nort/South position) */
pub static LAT_FAKT: f32 = 111100.0; // 111285; // exactly enough  111120 = 1.852 * 1000.0 * 60  // 1 NM je Bogenminute: 1 Grad Lat = 60 NM = 111 km, 0.001 Grad = 111 m
// todo: pbf-tiles are about 1 meter overlapping witt 111100!
// Tested: 111285 is ok but not perfect


/**
 * OSM x/y tile "name" - parts
 *
 * We use the OSM naming for tiles:
 * https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames.
 * x and y relate to a lonitude and latitude position on Earth.
 * In OSM he two values are only used as part of the filename of a tile.
 * So we named this TypeScript variable/class-type as "Tile_Name",
 * even if it is an x/y coordinate in numbers.
 */
pub type TileName = glam::Vec2; // BABYLON.Vector2

/** OSM zoom level of the pbf-tiles (13: ca. 3200 meter square) */
pub const PBF_ZOOM: f32 = 13.;

/** OSM zoom unit of the view-tiles  */
pub const VIEW_ZOOM: f32 = 13.;

/** Calculated factor between pbf zoom and view zoom. */
pub const FACT_ZOOM: f32 = (2_u32).pow((VIEW_ZOOM - PBF_ZOOM) as u32) as f32;  // 3^2=8   (in zoom/zoom) powf can't be used for const

/**
 * x/z dimentions of a tile: nord, latitude / east, Longitude  (y is not used)
 * The Earth nord direction is shown into the backround of the BabylonJS 3D space.
 * Positiv Longitude will be a negative z value!
 */
pub type TileSize = glam::Vec2; // BABYLON.Vector3


