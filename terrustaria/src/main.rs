use bevy::{prelude::*, time::FixedTimestep};
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

mod map;
use map::{spawn_background, spawn_foreground_map};

mod constants;
use constants::TIME_STEP;

mod helpers;
use helpers::camera_debug_movement as camera_movement;

mod player;
use player::{player_movement, spawn_player};

mod npc;
use npc::{rotate_to_player, snap_to_player, spawn_enemies};

mod cursor;
use cursor::{update_cursor_pos, CursorPos};

mod highlight;
use highlight::{highlight_tile_labels, spawn_tile_labels, FontHandle};

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
        .init_resource::<FontHandle>()
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(TilemapPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(300.))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(spawn_background)
        // .add_startup_system(spawn_wall_map)
        .add_startup_system(spawn_foreground_map)
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_tile_labels)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemies)
        .add_startup_system(setup_camera)
        .add_system(camera_movement)
        .add_system_to_stage(CoreStage::First, update_cursor_pos.after(camera_movement))
        .add_system(highlight_tile_labels)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_movement)
                .with_system(snap_to_player)
                .with_system(rotate_to_player),
        )
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    // commands.spawn(Camera2dBundle::default()).insert(Transform::from_translation(CAMERA_POS));
    commands.spawn(Camera2dBundle::default());
}
