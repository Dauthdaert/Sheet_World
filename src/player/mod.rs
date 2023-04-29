mod camera;
mod movement;

use bevy::prelude::*;
use bevy_parallax::ParallaxSystems;

use crate::states::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(camera::spawn);

        app.add_system(
            movement::movement
                .in_set(OnUpdate(GameState::InGame))
                .before(ParallaxSystems),
        );
    }
}
