use std::collections::BTreeMap;

use bevy::prelude::*;

#[derive(Component)]
pub struct TileMap {
    pub tiles: BTreeMap<i32, BTreeMap<i32, Entity>>,
    pub dummy: Handle<Mesh>,
}

#[derive(Component)]
/// Holds a dummy square until the actual mesh has loaded
pub struct Tile(pub Option<Entity>);

impl TileMap {
    pub fn load(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        x: i32,
        y: i32,
    ) {
        self.tiles
            .entry(x)
            .or_default()
            .entry(y)
            .or_insert_with(|| {
                // https://gltiles.osm2world.org/glb/lod1/15/17388/11332.glb#Scene0"

                // Just for testing:
                const TILE_SIZE: f32 = 814.5;
                const X0: i32 = 17430;
                const Y0: i32 = 11370;

                let name: String = format!("models/{}_{}.glb#Scene0", x, y); //format!("hello {}", "world!");
                let transform = Transform::from_xyz(
                    (x - X0) as f32 * TILE_SIZE,
                    0.,
                    (y - Y0) as f32 * TILE_SIZE,
                ); // OSM y => GPU z

                let dummy = commands
                    .spawn(PbrBundle {
                        mesh: self.dummy.clone(),
                        transform,
                        ..default()
                    })
                    .id();
                commands
                    .spawn((
                        SceneBundle {
                            scene: asset_server.load(name), // "models/17430_11371.glb#Scene0"
                            transform,
                            ..default()
                        },
                        Tile(Some(dummy)),
                    ))
                    .id()
            });
    }

    pub fn new(meshes: &mut Assets<Mesh>) -> Self {
        Self {
            tiles: Default::default(),
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
        }
    }
}
