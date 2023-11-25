use std::collections::BTreeMap;

use bevy::{gltf::Gltf, prelude::*};

#[derive(Component, Default)]
pub struct TileMap<const TILE_SIZE: u32> {
    /// All currently loaded tiles.
    tiles: BTreeMap<i32, BTreeMap<i32, Entity>>,
    /// The tile currently being loaded.
    loading: Option<(IVec2, Handle<Gltf>)>,
    /// Dummy square to show while a scene is loading
    dummy: Handle<Mesh>,
}

#[derive(Component)]
pub struct Tile;

impl<const TILE_SIZE: u32> TileMap<TILE_SIZE> {
    pub const TILE_SIZE: f32 = TILE_SIZE as f32 / 10.0;

    pub fn load_next(
        &mut self,
        tilemap_id: Entity,
        commands: &mut Commands,
        server: &AssetServer,
        origin: Vec3,
    ) {
        let origin = origin.xz() / Self::TILE_SIZE;
        let origin = origin.as_ivec2();
        let mut best_score = f32::INFINITY;
        let mut best_pos = None;
        for x_i in -1..=1 {
            for y_i in -1..=1 {
                let offset = IVec2::new(x_i, y_i);
                let score = self.get_view_tile_score(origin, offset);
                if score < best_score {
                    best_pos = Some(origin + offset);
                    best_score = score;
                }
            }
        }
        if let Some(best_pos) = best_pos {
            self.load(tilemap_id, commands, server, best_pos);
        }
    }

    /// Takes an offset to the player position and returns a score for how important
    /// to load it is. Lower values are better.
    // FIXME(#18): use a smarter algorithm
    pub fn get_view_tile_score(&self, pos: IVec2, offset: IVec2) -> f32 {
        if let Some(line) = self.tiles.get(&(pos.x + offset.x)) {
            if line.get(&(pos.y + offset.y)).is_some() {
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
        &mut self,
        tilemap_id: Entity,
        commands: &mut Commands,
        server: &AssetServer,
        pos: IVec2,
    ) {
        if self.loading.is_some() {
            return;
        }
        // https://gltiles.osm2world.org/glb/lod1/15/17388/11332.glb#Scene0"
        let name: String = format!("tile://{}_{}.glb", pos.x, pos.y);
        // Start loading next tile
        self.loading = Some((pos, server.load(name))); // "models/17430_11371.glb#Scene0"
                                                       // Insert dummy tile while loading.
        self.tiles
            .entry(pos.x)
            .or_default()
            .entry(pos.y)
            .or_insert_with(|| {
                let transform = Self::test_transform(pos);

                let id = commands
                    .spawn(PbrBundle {
                        mesh: self.dummy.clone(),
                        transform,
                        ..default()
                    })
                    .id();
                commands.entity(tilemap_id).add_child(id);
                id
            });
    }

    pub fn update(
        mut commands: Commands,
        server: Res<AssetServer>,
        scenes: Res<Assets<Gltf>>,
        mut tilemap: Query<(Entity, &mut Self)>,
    ) {
        for (id, mut tilemap) in &mut tilemap {
            // check if the currently loading tile is done
            if let Some((pos, scene)) = tilemap.loading.take() {
                use bevy::asset::LoadState::*;
                match server.get_load_state(&scene).unwrap() {
                    NotLoaded | Loading => {
                        tilemap.loading = Some((pos, scene));
                    }
                    Loaded => {
                        // FIXME: implement caching of downloaded assets by implementing something like
                        // https://github.com/bevyengine/bevy/blob/main/examples/asset/processing/asset_processing.rs

                        // Done, remove dummy tile and insert the real one
                        let entity = tilemap
                            .tiles
                            .entry(pos.x)
                            .or_default()
                            .get_mut(&pos.y)
                            .unwrap();

                        let transform = Self::test_transform(pos);
                        let scene = scenes.get(scene).unwrap().scenes[0].clone();
                        let tile = commands
                            .spawn((
                                SceneBundle {
                                    scene, // "models/17430_11371.glb#Scene0"
                                    transform,
                                    ..default()
                                },
                                Tile,
                            ))
                            .id();
                        commands.entity(id).add_child(tile);
                        let dummy = std::mem::replace(entity, tile);
                        commands.entity(dummy).despawn();
                    }
                    Failed => {
                        error!("failed to load tile {pos} from network");
                    }
                }
            }
        }
    }

    pub fn new(meshes: &mut Assets<Mesh>) -> Self {
        let half = Self::TILE_SIZE / 2.0;
        Self {
            dummy: meshes.add(
                shape::Box {
                    min_x: -half,
                    max_x: half,
                    min_y: 0.0,
                    max_y: 1.0,
                    min_z: -half,
                    max_z: half,
                }
                .into(),
            ),
            ..default()
        }
    }

    fn test_transform(pos: IVec2) -> Transform {
        let pos = pos.as_vec2() * Self::TILE_SIZE;
        // OSM y => GPU z
        Transform::from_xyz(pos.x, 0., pos.y)
    }
}
