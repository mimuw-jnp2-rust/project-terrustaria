use bevy::{prelude::*, time::FixedTimestep};
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

mod map;
use map::{spawn_background, spawn_foreground_map, spawn_wall_map, spawn_colliders};

mod constants;
use constants::world::*;

mod helpers;
use helpers::{
    camera_debug_movement as camera_movement,
};

mod player;
use player::{player_movement, spawn_player, player_jump, player_jump_reset};

mod npc;
use npc::{rotate_to_player, snap_to_player, spawn_enemies};

mod cursor;
use cursor::{update_cursor_pos, CursorPos};

mod tile;

mod destroy_tiles;
use destroy_tiles::destroy_tile_after_click;

fn main() {
    App::new()
        .add_plugins(
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
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(TilemapPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PHYSICS_SCALE))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(spawn_background)
        .add_startup_system(spawn_wall_map)
        .add_startup_system(spawn_foreground_map)
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_colliders)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemies)
        //.add_startup_system(spawn_big_box_collider)
        .add_startup_system(setup_camera)
        .add_system(camera_movement)
        .add_system_to_stage(CoreStage::First, update_cursor_pos.after(camera_movement))
        .add_system(destroy_tile_after_click)
        //player systems
        .add_system(player_jump)
        .add_system(player_jump_reset)
        .add_system(player_movement)
        // .add_system(display_events)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(snap_to_player)
                .with_system(rotate_to_player),
        )
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
