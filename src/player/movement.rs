use bevy::prelude::*;

pub fn movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let dt = time.delta_seconds();

    let mut camera_transform = camera_query.single_mut();
    if keys.pressed(KeyCode::Up) {
        camera_transform.translation.y += 300.0 * dt;
    }
    if keys.pressed(KeyCode::Down) {
        camera_transform.translation.y -= 300.0 * dt;
    }
    if keys.pressed(KeyCode::Right) {
        camera_transform.translation.x += 300.0 * dt;
    }
    if keys.pressed(KeyCode::Left) {
        camera_transform.translation.x -= 300.0 * dt;
    }
}
