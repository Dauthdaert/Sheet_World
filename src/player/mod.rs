mod camera;
mod movement;

pub use camera::PlayerCamera;

use bevy::prelude::*;
use bevy_parallax::ParallaxSystems;

use crate::states::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(camera::spawn.in_schedule(OnEnter(GameState::InGame)));

        app.add_system(
            movement::movement
                .in_set(OnUpdate(GameState::InGame))
                .before(ParallaxSystems),
        );
    }
}
