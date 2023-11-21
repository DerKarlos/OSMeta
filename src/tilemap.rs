use std::collections::{BTreeMap, VecDeque};

use bevy::prelude::*;

#[derive(Component, Default)]
pub struct TileMap<const TILE_SIZE: u32> {
    /// All currently loaded tiles.
    tiles: BTreeMap<i32, BTreeMap<i32, Entity>>,
    /// fifo queue of tiles to be loaded.
    to_load: VecDeque<IVec2>,
    /// The tile currently being loaded.
    loading: Option<(IVec2, Handle<Scene>)>,
    /// Dummy square to show while a scene is loading
    dummy: Handle<Mesh>,
}

#[derive(Component)]
pub struct Tile;

impl<const TILE_SIZE: u32> TileMap<TILE_SIZE> {
    pub const TILE_SIZE: f32 = TILE_SIZE as f32 / 10.0;

    pub fn load_nearest(&mut self, tilemap_id: Entity, commands: &mut Commands, pos: Vec3) {
        let pos = pos.xz() / Self::TILE_SIZE;
        let pos = pos.as_ivec2();
        for x_i in -1..=1 {
            for y_i in -1..=1 {
                let offset = IVec2::new(x_i, y_i);
                self.load(tilemap_id, commands, pos + offset);
            }
        }
    }

    /// Queue a tile coordinate for loading. This will load tiles
    /// in sequence to reduce lag (which would happen if we loaded lots
    /// of tiles at the same time).
    pub fn load(&mut self, tilemap_id: Entity, commands: &mut Commands, pos: IVec2) {
        self.tiles
            .entry(pos.x)
            .or_default()
            .entry(pos.y)
            .or_insert_with(|| {
                self.to_load.push_front(pos);

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
        mut tilemap: Query<(Entity, &mut Self)>,
    ) {
        for (id, mut tilemap) in &mut tilemap {
            // check if the currently loading tile is done
            if let Some((pos, scene)) = tilemap.loading.take() {
                use bevy::asset::LoadState::*;
                match server.get_load_state(&scene).unwrap() {
                    NotLoaded | Loading => {
                        tilemap.loading = Some((pos, scene));
                        return;
                    }
                    Loaded => {
                        // Done, remove dummy tile and insert the real one
                        let entity = tilemap.tiles.entry(pos.x).or_default().get_mut(&pos.y).unwrap();

                        let transform = Self::test_transform(pos);
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
                        error!("failed to load tile {pos}");
                    }
                }
            }

            assert!(tilemap.loading.is_none());
            // Check if there are more tiles to load
            let Some(pos) = tilemap.to_load.pop_back() else {
                return;
            };

            // https://gltiles.osm2world.org/glb/lod1/15/17388/11332.glb#Scene0"
            let name: String = format!("models/{}_{}.glb#Scene0", pos.x, pos.y);
            // Start loading next tile
            tilemap.loading = Some((pos, server.load(name))); // "models/17430_11371.glb#Scene0"
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
