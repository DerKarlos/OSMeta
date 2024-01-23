use bevy::{
    asset::LoadState,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    gltf::Gltf,
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    utils::HashSet,
};

use crate::geocoord::{GeoCoord, EARTH_RADIUS};
use crate::ViewDistance;

use crate::big_space::Space;
use crate::player::Control;

use crate::{GalacticGrid, GalacticTransform, GalacticTransformOwned};

mod coord;
mod index;
pub use coord::*;
pub use index::*;

#[derive(Resource, Default)]
pub struct TileMap {
    /// All currently loaded tiles.
    tiles: HashSet<TileIndex>,
}

#[derive(Component)]
/// A marker component for tiles that are currently being loaded.
pub struct Loading;

pub const TILE_ZOOM: u8 = 15;

fn phytagoras(a: f32, b: f32) -> f32 {
    (a * a + b * b).sqrt()
}

impl TileMap {
    pub fn hide_faraway_tiles(
        In((origin, radius)): In<(TileIndex, f32)>,
        mut tiles: Query<(&TileIndex, &mut Visibility)>,
        fly_cam: Query<GalacticTransform, With<Control>>, // todo: make camera elevation a global resource?
    ) {
        let elevation = fly_cam.single().position_double().length() as f32 - EARTH_RADIUS;
        for (tile, mut vis) in tiles.iter_mut() {
            // FIXME: use tile zoom level to increase view-distance for lower zoom tiles.
            let tile_size = tile.as_coord().to_geo_coord().tile_size(TILE_ZOOM);
            let distance = (tile.distance_squared(origin) as f32).sqrt() * tile_size;
            //info!("o_e_r {:?} {:?} {:?}", distance, elevation, radius);
            if phytagoras(distance, elevation) > radius {
                *vis = Visibility::Hidden;
            } else {
                *vis = Visibility::Inherited;
            }
        }
    }

    pub fn load_next(
        In((origin, radius)): In<(TileIndex, f32)>,
        tilemap: Res<TileMap>,
        loading: Query<&Loading>,
        fly_cam: Query<GalacticTransform, With<Control>>,
    ) -> Option<TileIndex> {
        if !loading.is_empty() {
            return None;
        }
        let tile_size = origin.as_coord().to_geo_coord().tile_size(TILE_ZOOM);
        let mut best_score = f32::INFINITY;
        let mut best_pos = None;
        let elevation = fly_cam.single().position_double().length() as f32 - EARTH_RADIUS;
        let dist_max = (radius / tile_size).ceil() as i32;
        for x_i in -dist_max..=dist_max {
            for y_i in -dist_max..=dist_max {
                let offset = IVec2::new(x_i, y_i);
                let distance = (offset.length_squared() as f32).sqrt() * tile_size;
                //info!("o e r {:?} {:?} {:?}",distance,elevation,radius);
                if phytagoras(distance, elevation) > radius {
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
        if self.tiles.contains(&pos) {
            return f32::INFINITY;
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
    ) {
        let Some(pos) = pos else { return };
        // https://gltiles.osm2world.org/glb/lod1/15/17388/11332.glb#Scene0"
        let name: String = format!("tile://{}_{}_{}.glb", pos.zoom(), pos.x, pos.y);
        // Start loading next tile
        let gltf: Handle<Gltf> = server.load(name);
        if !tilemap.tiles.insert(pos) {
            return;
        }

        // Insert dummy tile while loading.
        let (grid, _coord, mesh) = flat_tile(pos);
        let mesh = meshes.add(mesh);

        commands.spawn((PbrBundle { mesh, ..default() }, pos, grid, Loading, gltf));
    }

    pub fn update(
        mut commands: Commands,
        server: Res<AssetServer>,
        scenes: ResMut<Assets<Gltf>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        next: Query<(Entity, &TileIndex, &Handle<Gltf>), With<Loading>>,
    ) {
        let Ok((entity, pos, scene)) = next.get_single() else {
            return;
        };
        let state = server.get_load_state(scene).unwrap();
        if let LoadState::NotLoaded | LoadState::Loading = state {
            return;
        }

        let Some(mut entity) = commands.get_entity(entity) else {
            return;
        };
        entity.remove::<Loading>();
        entity.remove::<Handle<Gltf>>();

        match state {
            LoadState::NotLoaded | LoadState::Loading => unreachable!(),
            LoadState::Loaded => {
                entity.remove::<PbrBundle>();
                let GalacticTransformOwned { transform, cell } = pos.to_cartesian();
                let scene = scenes.get(scene).unwrap().scenes[0].clone();
                entity.insert(cell);
                entity.insert(SceneBundle {
                    scene, // "models/17430_11371.glb#Scene0"
                    transform,
                    ..default()
                });
            }
            LoadState::Failed => {
                let url = format!(
                    "https://a.tile.openstreetmap.org/{}/{}/{}.png",
                    pos.zoom(),
                    pos.x,
                    pos.y
                );
                debug!(
                    ?url,
                    "failed to load tile {pos} from network, switching to flat tile"
                );
                let image: Handle<Image> = server.load(url);
                entity.insert(materials.add(StandardMaterial {
                    base_color_texture: Some(image),
                    perceptual_roughness: 1.0,
                    ..default()
                }));
            }
        }
    }
}

// Compute a square mesh at the position for the given tile.
fn flat_tile(pos: TileIndex) -> (GalacticGrid, coord::TileCoord, Mesh) {
    let coord = pos.as_coord();

    // Four corners of the tile in cartesian coordinates relative to the
    // planet's center.
    let a = coord.to_geo_coord().to_cartesian();
    let b = pos.right().as_coord().to_geo_coord().to_cartesian();
    let c = pos.down().right().as_coord().to_geo_coord().to_cartesian();
    let d = pos.down().as_coord().to_geo_coord().to_cartesian();

    // Normals on a sphere are just the position on the sphere normalized.
    let normals = vec![
        a.normalize().as_vec3(),
        b.normalize().as_vec3(),
        c.normalize().as_vec3(),
        d.normalize().as_vec3(),
    ];

    // `a` is our anchor point, all others are relative
    let b = *b - *a;
    let c = *c - *a;
    let d = *d - *a;

    let (grid, a) = Space::translation_to_grid(a);
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

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    // After recomputing the view-distance from the FPS
                    recompute_view_distance,
                    (
                        // Hide tiles that are now beyond the view-distance
                        get_main_camera_position.pipe(TileMap::hide_faraway_tiles),
                        // And load tiles that are now within the view-distance
                        get_main_camera_position
                            .pipe(TileMap::load_next)
                            .pipe(TileMap::load),
                    ),
                )
                    .chain(),
                TileMap::update,
            ),
        );
    }
}

fn recompute_view_distance(
    diagnostics: Res<DiagnosticsStore>,
    mut view_distance: ResMut<ViewDistance>,
) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps) = fps.smoothed() {
            if fps < 40.0 {
                view_distance.0 *= 0.99;
            } else if fps > 59.5 {
                view_distance.0 *= 1.01;
            }
            view_distance.0 = view_distance.0.clamp(2000.0, 10000.0);
        }
    }
}

fn get_main_camera_position(
    player: crate::player::Player,
    view_distance: Res<ViewDistance>,
) -> (TileIndex, f32) {
    let player = player.pos();

    let pos = player.pos();
    let origin = GeoCoord::from_cartesian(pos);
    let tile_size = origin.tile_size(TILE_ZOOM);
    let radius = view_distance.0 + tile_size + 0.5;
    let origin = origin.to_tile_coordinates(TILE_ZOOM);

    (origin.as_tile_index(), radius)
}
