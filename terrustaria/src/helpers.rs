use bevy::prelude::*;
use bevy_rapier2d::dynamics::RigidBody;
use bevy_rapier2d::geometry::Collider;

// a simple camera system for moving and zooming the camera
// to easily see what is the output of what we create deep down in the mine
#[allow(dead_code)]
pub fn camera_debug_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    for (mut transform, mut ortho) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Z) {
            ortho.scale += 0.1;
        }

        if keyboard_input.pressed(KeyCode::X) {
            ortho.scale -= 0.1;
        }

        if ortho.scale < 0.5 {
            ortho.scale = 0.5;
        }

        let z = transform.translation.z;
        transform.translation += time.delta_seconds() * direction * 500.;
        // important: we need to restore the Z values when moving the camera around
        // Bevy has a specific camera setup and this can mess with how our layers are shown
        transform.translation.z = z;
    }
}


// can be called with (x,y) transforming to (x,y,Z_FRGRND) or empty transforming to (0,0,Z_FRGRND)
#[macro_export]
macro_rules! bring_to_foreground {
    ($x:expr, $y:expr) => {
        Transform::from_xyz($x, $y, Z_FOREGROUND)
    };
    () => {
        Transform::from_xyz(0., 0., Z_FOREGROUND)
    };
}

pub(crate) use bring_to_foreground;
use crate::constants::Z_FOREGROUND;


#[allow(dead_code)]
//spawns big box collider for testing
pub fn spawn_big_box_collider(mut commands: Commands) {
    #[derive(Component)]
    struct BigBoxCollider;
    commands
        .spawn((BigBoxCollider, Collider::cuboid(500., 100.)))
        .insert(Name::new("BoxCollider"))
        .insert(RigidBody::Fixed)
        .insert(TransformBundle::from(Transform::from_xyz(0., -200., Z_FOREGROUND)));
}