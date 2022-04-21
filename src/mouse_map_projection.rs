use bevy::prelude::*;

/// A plugin that manages the MouseWorldPosition resource
/// to show the mouse's position relative to the any camera
/// with the MainCamera component
pub struct MouseWorldProjectionPlugin;

impl Plugin for MouseWorldProjectionPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(MouseWorldPosition(None))
            .add_system(mouse_position_system);
    }
}

/// a resource showing projected mouse position
/// relative to the camera with MainCamera component
pub struct MouseWorldPosition(pub Option<Vec2>);

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

fn mouse_position_system(
    //mut commands: Commands,
    //mut previous_pos: Option<ResMut<PreviousPos>>,
    //mouse: Res<Input<MouseButton>>,
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut mouse_position: ResMut<MouseWorldPosition>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    if let Ok((camera, camera_transform)) = q_camera.get_single() {
        let wnd = wnds.get(camera.window).unwrap();
        if let Some(screen_pos) = wnd.cursor_position() {
            // get the size of the window
            let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

            // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

            // matrix for undoing the projection and camera transform
            let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

            // use it to convert ndc to world-space coordinates
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

            // reduce it to a 2D value
            let world_pos: Vec2 = world_pos.truncate();

            *mouse_position = MouseWorldPosition(Some(world_pos));
        } else {
            *mouse_position = MouseWorldPosition(None);
        }
    }
}
