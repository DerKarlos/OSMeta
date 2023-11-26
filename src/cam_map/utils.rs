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
#[derive(Debug, Clone, Copy)]
pub   // type TileName = glam::Vec2; // BABYLON.Vector2
    struct TileName {
        pub x: i32,
        pub y: i32,
    }
