use bevy::prelude::*;
use bevy_parallax::ParallaxMoveEvent;

pub fn movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut parallax_move_writer: EventWriter<ParallaxMoveEvent>,
) {
    let dt = time.delta_seconds();

    let mut camera_transform = camera_query.single_mut();
    if keys.pressed(KeyCode::Up) {
        camera_transform.translation.y += 300.0 * dt;
        parallax_move_writer.send(ParallaxMoveEvent {
            camera_move_speed: Vec2::new(0.0, 1.0),
        });
    }
    if keys.pressed(KeyCode::Down) {
        camera_transform.translation.y -= 300.0 * dt;
        parallax_move_writer.send(ParallaxMoveEvent {
            camera_move_speed: Vec2::new(0.0, -1.0),
        });
    }
    if keys.pressed(KeyCode::Right) {
        camera_transform.translation.x += 300.0 * dt;
        parallax_move_writer.send(ParallaxMoveEvent {
            camera_move_speed: Vec2::new(1.0, 0.0),
        });
    }
    if keys.pressed(KeyCode::Left) {
        camera_transform.translation.x -= 300.0 * dt;
        parallax_move_writer.send(ParallaxMoveEvent {
            camera_move_speed: Vec2::new(-1.0, 0.0),
        });
    }
}
