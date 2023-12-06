use std::{collections::BTreeMap, f32::consts::PI, fmt::Display};

use bevy::{
    gltf::Gltf,
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use big_space::FloatingOriginSettings;

use crate::{geopos::GeoPos, GalacticGrid};

#[derive(Resource, Default)]
pub struct TileMap {
    /// All currently loaded tiles.
    tiles: [BTreeMap<u32, BTreeMap<u32, Entity>>; MAX_TILE_ZOOM as usize],
    /// The tile currently being loaded.
    loading: Option<(TileIndex, Handle<Gltf>)>,
}

pub const TILE_ZOOM: u8 = 15;
pub const MAX_TILE_ZOOM: u8 = 16;

impl TileMap {
    pub fn hide_faraway_tiles(
        In((origin, radius)): In<(TileIndex, Vec2)>,
        mut tiles: Query<(&TileIndex, &mut Visibility)>,
    ) {
        for (tile, mut vis) in tiles.iter_mut() {
            // FIXME: use tile zoom level to increase view distance for lower zoom tiles.
            let offset = tile.distance_squared(origin);
            let oob = offset > radius.as_uvec2().length_squared();
            if oob {
                *vis = Visibility::Hidden;
            } else {
                *vis = Visibility::Inherited;
            }
        }
    }

    pub fn load_next(
        In((origin, radius)): In<(TileIndex, Vec2)>,
        tilemap: Res<TileMap>,
    ) -> Option<TileIndex> {
        let radius = radius.abs().ceil().copysign(radius).as_ivec2();
        let mut best_score = f32::INFINITY;
        let mut best_pos = None;
        for x_i in -radius.x..=radius.x {
            for y_i in -radius.y..=radius.y {
                let offset = IVec2::new(x_i, y_i);
                if offset.length_squared() > radius.length_squared() {
                    continue;
                }

                let pos = origin.offset(offset);
                let score = tilemap.get_view_tile_score(pos, offset);
                if score < best_score {
                    best_pos = Some(pos);
                    best_score = score;
                }
            }
        }
        best_pos
    }

    /// Takes an offset to the player position and returns a score for how important
    /// to load it is. Lower values are better.
    // FIXME(#18): use a smarter algorithm
    pub fn get_view_tile_score(&self, pos: TileIndex, offset: IVec2) -> f32 {
        if let Some(line) = self.tiles[usize::from(pos.zoom)].get(&pos.idx.x) {
            if line.get(&pos.idx.y).is_some() {
                return f32::INFINITY;
            }
        }
        offset.as_vec2().length_squared()
    }

    /// Queue a tile coordinate for loading. This will load tiles
    /// in sequence to reduce lag (which would happen if we loaded lots
    /// of tiles at the same time).
    /// Silently does nothing if the tile was already loaded or is in the process of loading.
    /// Silently does nothing if another tile is already being loaded.
    pub fn load(
        In(pos): In<Option<TileIndex>>,
        mut commands: Commands,
        server: Res<AssetServer>,
        mut tilemap: ResMut<TileMap>,
        mut meshes: ResMut<Assets<Mesh>>,
        space: Res<FloatingOriginSettings>,
    ) {
        let Some(pos) = pos else { return };
        if tilemap.loading.is_some() {
            return;
        }
        // https://gltiles.osm2world.org/glb/lod1/15/17388/11332.glb#Scene0"
        let name: String = format!("tile://{}_{}_{}.glb", pos.zoom, pos.idx.x, pos.idx.y);
        // Start loading next tile
        tilemap.loading = Some((pos, server.load(name))); // "models/17430_11371.glb#Scene0"
                                                          // Insert dummy tile while loading.
        tilemap.tiles[usize::from(pos.zoom)]
            .entry(pos.idx.x)
            .or_default()
            .entry(pos.idx.y)
            .or_insert_with(|| {
                let (grid, _coord, mesh) = flat_tile(pos, &space);
                let mesh = meshes.add(mesh);

                commands
                    .spawn((PbrBundle { mesh, ..default() }, pos, grid))
                    .id()
            });
    }

    pub fn update(
        mut commands: Commands,
        server: Res<AssetServer>,
        scenes: ResMut<Assets<Gltf>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut tilemap: ResMut<Self>,
        space: Res<FloatingOriginSettings>,
    ) {
        // check if the currently loading tile is done
        let Some((pos, scene)) = tilemap.loading.take() else {
            return;
        };
        let state = server.get_load_state(&scene).unwrap();
        use bevy::asset::LoadState::*;
        if let NotLoaded | Loading = state {
            tilemap.loading = Some((pos, scene));
            return;
        }
        // FIXME: implement caching of downloaded assets by implementing something like
        // https://github.com/bevyengine/bevy/blob/main/examples/asset/processing/asset_processing.rs

        // Done, remove dummy tile and insert the real one
        let Some(entity) = tilemap.tiles[usize::from(pos.zoom)]
            .entry(pos.idx.x)
            .or_default()
            .get(&pos.idx.y)
        else {
            return;
        };

        let Some(mut entity) = commands.get_entity(*entity) else {
            return;
        };
        entity.remove::<PbrBundle>();

        match state {
            NotLoaded | Loading => unreachable!(),
            Loaded => {
                let (grid, transform) = Self::test_transform(pos, &space);
                let scene = scenes.get(scene).unwrap().scenes[0].clone();
                entity.insert((
                    SceneBundle {
                        scene, // "models/17430_11371.glb#Scene0"
                        transform,
                        ..default()
                    },
                    grid,
                ));
            }
            Failed => {
                let (_grid, coord, mesh) = flat_tile(pos, &space);
                let mesh = meshes.add(mesh);
                let url = format!(
                    "https://a.tile.openstreetmap.org/{}/{}/{}.png",
                    coord.zoom, coord.pos.x, coord.pos.y
                );
                debug!(
                    ?url,
                    "failed to load tile {pos} from network, switching to flat tile"
                );
                let image: Handle<Image> = server.load(url);
                let material = materials.add(StandardMaterial {
                    base_color_texture: Some(image),
                    perceptual_roughness: 1.0,
                    ..default()
                });
                entity.insert(PbrBundle {
                    mesh,
                    material,
                    ..default()
                });
            }
        }
    }

    fn test_transform(pos: TileIndex, space: &FloatingOriginSettings) -> (GalacticGrid, Transform) {
        let coord = pos.as_coord().center();
        let pos = coord.to_geo_pos().to_cartesian();
        let up = pos.normalize().as_vec3();
        let next = TileCoord {
            pos: Vec2 {
                x: coord.pos.x,
                y: coord.pos.y - 1.0,
            },
            zoom: coord.zoom,
        }
        .to_geo_pos()
        .to_cartesian();
        let (grid, pos) = space.translation_to_grid(pos);
        let (grid_next, next) = space.translation_to_grid(next);
        let diff = grid_next - grid;
        let diff = Vec3 {
            x: diff.x as f32 * space.grid_edge_length(),
            y: diff.y as f32 * space.grid_edge_length(),
            z: diff.z as f32 * space.grid_edge_length(),
        };
        let next = next + diff;
        (
            grid,
            Transform::from_translation(pos).looking_to(next - pos, up),
        )
    }
}

// Compute a square mesh at the position for the given tile.
fn flat_tile(pos: TileIndex, space: &FloatingOriginSettings) -> (GalacticGrid, TileCoord, Mesh) {
    let coord = pos.as_coord();

    // Four corners of the tile in cartesian coordinates relative to the
    // planet's center.
    let a = coord.to_geo_pos().to_cartesian();
    let b = pos.right().as_coord().to_geo_pos().to_cartesian();
    let c = pos.down().right().as_coord().to_geo_pos().to_cartesian();
    let d = pos.down().as_coord().to_geo_pos().to_cartesian();

    // Normals on a sphere are just the position on the sphere normalized.
    let normals = vec![
        a.normalize().as_vec3(),
        b.normalize().as_vec3(),
        c.normalize().as_vec3(),
        d.normalize().as_vec3(),
    ];

    // `a` is our anchor point, all others are relative
    let b = b - a;
    let c = c - a;
    let d = d - a;

    let (grid, a) = space.translation_to_grid(a);
    let b = a + b.as_vec3();
    let c = a + c.as_vec3();
    let d = a + d.as_vec3();

    let positions = vec![a.to_array(), b.to_array(), c.to_array(), d.to_array()];
    let uvs = vec![Vec2::ZERO, Vec2::X, Vec2::splat(1.0), Vec2::Y];

    let indices = Indices::U32(vec![0, 3, 2, 2, 1, 0]);

    let mesh = Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_indices(Some(indices));
    (grid, coord, mesh)
}

/// A coordinate in the OWM tile coordinate system.
/// We use floats instead of integers so we can specify positions of objects
/// within a tile. E.g. (0.5, 0.5) is the position in the middle of tile (0, 0).
#[derive(Debug, Copy, Clone)]
pub struct TileCoord {
    pub pos: Vec2,
    pub zoom: u8,
}

impl TileCoord {
    pub fn to_geo_pos(self) -> GeoPos {
        let pow_zoom = 2_u32.pow(self.zoom.into()) as f32;

        let lon = self.pos.x / pow_zoom * 360.0 - 180.0;
        let lat_rad = (PI * (1. - 2. * self.pos.y / pow_zoom)).sinh().atan();
        let lat = lat_rad.to_degrees();
        GeoPos { lat, lon }
    }

    /// Offset this position by half a tile size. If you started out with a left upper
    /// corner position, you'll now be in the middle of the tile.
    fn center(&self) -> Self {
        Self {
            pos: self.pos + 0.5,
            zoom: self.zoom,
        }
    }

    pub fn as_tile_index(&self) -> TileIndex {
        TileIndex {
            idx: self.pos.as_uvec2(),
            zoom: self.zoom,
        }
    }
}

/// An x/y index of an OWM tile.
#[derive(Debug, Copy, Clone, Component)]
pub struct TileIndex {
    idx: UVec2,
    zoom: u8,
}

impl TileIndex {
    pub fn as_coord(self) -> TileCoord {
        TileCoord {
            pos: self.idx.as_vec2(),
            zoom: self.zoom,
        }
    }
    pub fn right(self) -> Self {
        Self {
            idx: self.idx + UVec2::X,
            ..self
        }
    }
    pub fn down(self) -> Self {
        Self {
            idx: self.idx + UVec2::Y,
            ..self
        }
    }

    fn distance_squared(&self, origin: TileIndex) -> u32 {
        assert_eq!(self.zoom, origin.zoom);
        let max_tiles = 2_u32.pow(self.zoom.into());
        let mut x = self.idx.x.abs_diff(origin.idx.x);
        x = x.min(max_tiles - x);
        let mut y = self.idx.y.abs_diff(origin.idx.y);
        y = y.min(max_tiles - y);
        x * x + y * y
    }

    fn offset(self, offset: IVec2) -> TileIndex {
        let max_tiles = 2_i32.pow(self.zoom.into());
        let mut idx = self.idx.as_ivec2() + offset;
        if idx.x < 0 {
            idx.x += max_tiles;
        }
        idx.x %= max_tiles;
        if idx.y < 0 {
            idx.y += max_tiles;
        }
        idx.y %= max_tiles;
        TileIndex {
            idx: idx.as_uvec2(),
            zoom: self.zoom,
        }
    }
}

impl Display for TileIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.idx.fmt(f)
    }
}
