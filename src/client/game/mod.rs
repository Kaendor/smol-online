use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use self::inputs::PlayerInputPlugin;
use self::network::NetworkPlugin;

mod inputs;
mod network;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, setup_map))
            .add_plugins((NetworkPlugin, PlayerInputPlugin, TilemapPlugin));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[inline]
pub fn world_to_grid_pos(world: Vec2) -> IVec2 {
    world.floor().as_ivec2()
}

const QUADRANT_SIDE_LENGTH: u32 = 80;

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("iso_color.png");
    let map_size = TilemapSize {
        x: QUADRANT_SIDE_LENGTH * 2,
        y: QUADRANT_SIDE_LENGTH * 2,
    };
    let quadrant_size = TilemapSize {
        x: QUADRANT_SIDE_LENGTH,
        y: QUADRANT_SIDE_LENGTH,
    };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);

    fill_tilemap_rect(
        TileTextureIndex(0),
        TilePos { x: 0, y: 0 },
        quadrant_size,
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    fill_tilemap_rect(
        TileTextureIndex(1),
        TilePos {
            x: QUADRANT_SIDE_LENGTH,
            y: 0,
        },
        quadrant_size,
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    fill_tilemap_rect(
        TileTextureIndex(2),
        TilePos {
            x: 0,
            y: QUADRANT_SIDE_LENGTH,
        },
        quadrant_size,
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    fill_tilemap_rect(
        TileTextureIndex(3),
        TilePos {
            x: QUADRANT_SIDE_LENGTH,
            y: QUADRANT_SIDE_LENGTH,
        },
        quadrant_size,
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    let tile_size = TilemapTileSize { x: 64.0, y: 32.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Isometric(IsoCoordSystem::Staggered);

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        map_type,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}

pub fn fill_tilemap_rect(
    texture_index: TileTextureIndex,
    origin: TilePos,
    size: TilemapSize,
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
) {
    commands.entity(tilemap_id.0).with_children(|parent| {
        for x in 0..size.x {
            for y in 0..size.y {
                let tile_pos = TilePos {
                    x: origin.x + x,
                    y: origin.y + y,
                };

                let tile_entity = parent
                    .spawn(TileBundle {
                        position: tile_pos,
                        tilemap_id,
                        texture_index,
                        ..Default::default()
                    })
                    .id();
                tile_storage.set(&tile_pos, tile_entity);
            }
        }
    });
}

/// Calculates a [`Transform`] for a tilemap that places it so that its center is at
/// `(0.0, 0.0, 0.0)` in world space.
pub fn get_tilemap_center_transform(
    size: &TilemapSize,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType,
    z: f32,
) -> Transform {
    let low = TilePos::new(0, 0).center_in_world(grid_size, map_type);
    let high = TilePos::new(size.x - 1, size.y - 1).center_in_world(grid_size, map_type);

    let diff = high - low;

    Transform::from_xyz(-diff.x / 2., -diff.y / 2., z)
}
