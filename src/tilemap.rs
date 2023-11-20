use std::collections::{BTreeMap, VecDeque};

use bevy::prelude::*;

#[derive(Component, Default)]
pub struct TileMap {
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

impl TileMap {
    /// Queue a tile coordinate for loading. This will load tiles
    /// in sequence to reduce lag (which would happen if we loaded lots
    /// of tiles at the same time).
    pub fn load(&mut self, commands: &mut Commands, x: i32, y: i32) {
        self.tiles
            .entry(x)
            .or_default()
            .entry(y)
            .or_insert_with(|| {
                self.to_load.push_front((x, y));
                let transform = test_transform(x, y);

                commands
                    .spawn(PbrBundle {
                        mesh: self.dummy.clone(),
                        transform,
                        ..default()
                    })
                    .id()
            });
    }
}

pub fn update(mut commands: Commands, server: Res<AssetServer>, mut tilemap: Query<&mut TileMap>) {
    for mut tilemap in &mut tilemap {
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

                    let transform = test_transform(x, y);
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

impl TileMap {
    pub fn new(meshes: &mut Assets<Mesh>) -> Self {
        Self {
            dummy: meshes.add(
                shape::Box {
                    min_x: 0.0,
                    max_x: 814.5,
                    min_y: 0.0,
                    max_y: 1.0,
                    min_z: 0.0,
                    max_z: 814.5,
                }
                .into(),
            ),
            ..default()
        }
    }
}

fn test_transform(x: i32, y: i32) -> Transform {
    // Just for testing:
    const TILE_SIZE: f32 = 814.5;
    const X0: i32 = 17430;
    const Y0: i32 = 11370;

    // OSM y => GPU z
    Transform::from_xyz((x - X0) as f32 * TILE_SIZE, 0., (y - Y0) as f32 * TILE_SIZE)
}
