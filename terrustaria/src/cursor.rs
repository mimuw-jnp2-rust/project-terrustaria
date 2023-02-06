use crate::player::MainCamera;
use bevy::render::camera::RenderTarget;
use bevy::{ecs::system::Resource, prelude::*};

#[derive(Resource)]
pub struct CursorPos(pub(crate) Vec3);

impl Default for CursorPos {
    fn default() -> Self {
        Self(Vec3::new(-1000.0, -1000.0, 0.0))
    }
}

// update the cursor position resource system
pub fn update_cursor_pos(
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut cursor_pos: ResMut<CursorPos>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width(), wnd.height());
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        let world_pos: Vec3 = world_pos.truncate().extend(0.);

        *cursor_pos = CursorPos(world_pos);
    }
}
