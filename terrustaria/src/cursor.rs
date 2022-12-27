use bevy::{ecs::system::Resource, prelude::*};

// converts the cursor position into a world position
// takes into account any transforms applied by the camera
pub fn cursor_pos_in_world(
    windows: &Windows,
    cursor_pos: Vec2,
    cam_t: &Transform,
    cam: &Camera,
) -> Vec3 {
    let window = windows.primary();
    let window_size = Vec2::new(window.width(), window.height());

    let ndc_to_world = cam_t.compute_matrix() * cam.projection_matrix().inverse();
    let ndc = (cursor_pos / window_size) * 2.0 - Vec2::ONE;
    ndc_to_world.project_point3(ndc.extend(0.0))
}

#[derive(Resource)]
pub struct CursorPos(pub(crate) Vec3);

impl Default for CursorPos {
    fn default() -> Self {
        Self(Vec3::new(-1000.0, -1000.0, 0.0))
    }
}

// we need to keep the cursor position updated based on any `CursorMoved` events
pub fn update_cursor_pos(
    windows: Res<Windows>,
    camera_q: Query<(&Transform, &Camera)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_pos: ResMut<CursorPos>,
) {
    for cursor_moved in cursor_moved_events.iter() {
        for (cam_t, cam) in camera_q.iter() {
            *cursor_pos = CursorPos(cursor_pos_in_world(
                &windows,
                cursor_moved.position,
                cam_t,
                cam,
            ));
        }
    }
}
