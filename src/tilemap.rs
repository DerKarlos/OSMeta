use std::collections::{BTreeMap, VecDeque};

use bevy::prelude::*;

#[derive(Component, Default)]
pub struct TileMap<const TILE_SIZE: u32> {
    /// All currently loaded tiles.
    tiles: BTreeMap<i32, BTreeMap<i32, Entity>>,
    /// fifo queue of tiles to be loaded.
    to_load: VecDeque<(i32, i32)>,
    /// The tile currently being loaded.
    loading: Option<(i32, i32, Handle<Scene>)>,
    /// Dummy square to show while a scene is loading
    dummy: Handle<Mesh>,
}

#[derive(Component)]
pub struct Tile;

impl<const TILE_SIZE: u32> TileMap<TILE_SIZE> {
    pub const TILE_SIZE: f32 = TILE_SIZE as f32 / 10.0;
    /// Queue a tile coordinate for loading. This will load tiles
    /// in sequence to reduce lag (which would happen if we loaded lots
    /// of tiles at the same time).
    pub fn load(&mut self, tilemap_id: Entity, commands: &mut Commands, x: i32, y: i32) {
        self.tiles
            .entry(x)
            .or_default()
            .entry(y)
            .or_insert_with(|| {
                self.to_load.push_front((x, y));

                let transform = Self::test_transform(x, y);

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
            if let Some((x, y, scene)) = tilemap.loading.take() {
                use bevy::asset::LoadState::*;
                match server.get_load_state(&scene).unwrap() {
                    NotLoaded | Loading => {
                        tilemap.loading = Some((x, y, scene));
                        return;
                    }
                    Loaded => {
                        // Done, remove dummy tile and insert the real one
                        let entity = tilemap.tiles.entry(x).or_default().get_mut(&y).unwrap();

                        let transform = Self::test_transform(x, y);
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
                    Failed => todo!(),
                }
            }

            assert!(tilemap.loading.is_none());
            // Check if there are more tiles to load
            let Some((x, y)) = tilemap.to_load.pop_back() else {
                return;
            };

            // https://gltiles.osm2world.org/glb/lod1/15/17388/11332.glb#Scene0"
            let name: String = format!("models/{}_{}.glb#Scene0", x, y);
            // Start loading next tile
            tilemap.loading = Some((x, y, server.load(name))); // "models/17430_11371.glb#Scene0"
        }
    }

    pub fn new(meshes: &mut Assets<Mesh>) -> Self {
        Self {
            dummy: meshes.add(
                shape::Box {
                    min_x: 0.0,
                    max_x: Self::TILE_SIZE,
                    min_y: 0.0,
                    max_y: 1.0,
                    min_z: 0.0,
                    max_z: Self::TILE_SIZE,
                }
                .into(),
            ),
            ..default()
        }
    }

    fn test_transform(x: i32, y: i32) -> Transform {
        // OSM y => GPU z
        Transform::from_xyz(x as f32 * Self::TILE_SIZE, 0., y as f32 * Self::TILE_SIZE)
    }
}
