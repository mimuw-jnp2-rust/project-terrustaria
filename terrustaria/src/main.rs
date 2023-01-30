use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

mod map;
use map::{handle_cover, spawn_cover_map};
use map::{spawn_background, spawn_colliders, spawn_foreground_map, spawn_wall_map};

mod constants;
use constants::world::*;

mod helpers;
use helpers::camera_debug_movement as camera_movement;

mod player;
use player::{player_jump, player_jump_reset, player_movement, spawn_player};

mod cursor;
use cursor::{update_cursor_pos, CursorPos};

mod tile;

mod destroy_tiles;
use destroy_tiles::destroy_tile_after_click;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    width: 1270.0,
                    height: 720.0,
                    title: String::from("Terrustaria"),
                    ..Default::default()
                },
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .init_resource::<CursorPos>()
    .add_plugin(TilemapPlugin)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
        PHYSICS_SCALE,
    ))
    .add_startup_system(spawn_background)
    .add_startup_system(spawn_wall_map)
    .add_startup_system(spawn_foreground_map)
    .add_startup_system(spawn_cover_map)
    .add_startup_system_to_stage(StartupStage::PostStartup, spawn_colliders)
    .add_startup_system(spawn_player)
    .add_system(camera_movement)
    // player systems
    .add_system(player_jump)
    .add_system(player_jump_reset)
    .add_system(player_movement)
    .add_system(handle_cover)
    .add_system(update_cursor_pos)
    .add_system(destroy_tile_after_click)
    .add_system(bevy::window::close_on_esc);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierDebugRenderPlugin::default());

    app.run();
}
