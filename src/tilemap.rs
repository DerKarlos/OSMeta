use std::collections::{BTreeMap, VecDeque};

use bevy::prelude::*;

#[derive(Component, Default)]
pub struct TileMap {
    tiles: BTreeMap<i32, BTreeMap<i32, Entity>>,
    to_load: VecDeque<(i32, i32)>,
    loading: Option<(i32, i32, Handle<Scene>)>,
    dummy: Handle<Mesh>,
}

#[derive(Component)]
pub struct Tile;

impl TileMap {
    pub fn load(&mut self, commands: &mut Commands, x: i32, y: i32) {
        self.to_load.push_front((x, y));
        self.tiles
            .entry(x)
            .or_default()
            .entry(y)
            .or_insert_with(|| {
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

    pub fn update(&mut self, commands: &mut Commands, server: &Res<AssetServer>) {
        if let Some((x, y, scene)) = self.loading.take() {
            use bevy::asset::LoadState::*;
            match server.get_load_state(&scene).unwrap() {
                NotLoaded | Loading => {
                    self.loading = Some((x, y, scene));
                    return;
                }
                Loaded => {
                    let entity = self.tiles.entry(x).or_default().get_mut(&y).unwrap();

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

        assert!(self.loading.is_none());
        let Some((x, y)) = self.to_load.pop_back() else {
            return;
        };

        // https://gltiles.osm2world.org/glb/lod1/15/17388/11332.glb#Scene0"
        let name: String = format!("models/{}_{}.glb#Scene0", x, y);
        self.loading = Some((x, y, server.load(name))); // "models/17430_11371.glb#Scene0"
    }

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

    let transform =
        Transform::from_xyz((x - X0) as f32 * TILE_SIZE, 0., (y - Y0) as f32 * TILE_SIZE);
    // OSM y => GPU z
    transform
}
