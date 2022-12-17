use bevy::prelude::*;
use bevy_ecs_tilemap::{
    prelude::*,
};

#[derive(Component)]
struct Player;

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}


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
        .add_plugin(TilemapPlugin)
        .add_startup_system(startup)
        .run();
}

