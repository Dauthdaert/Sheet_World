mod background;
mod player;
mod states;
mod ui;
mod world;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::common_conditions::input_toggle_active,
    prelude::*,
};
use bevy_asset_loader::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use states::GameState;

pub fn app() -> App {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    mode: bevy::window::WindowMode::Windowed,
                    title: "Sheet World".to_string(),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    );

    #[cfg(debug_assertions)]
    {
        app.add_plugin(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F3)),
        );

        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin::default());
    }

    app.add_state::<GameState>();

    app.add_loading_state(
        LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::InGame),
    );

    app.add_plugin(player::PlayerPlugin);
    app.add_plugin(world::WorldPlugin);
    app.add_plugin(ui::UIPlugin);

    // TODO: Put this back in once I have proper background art.
    //app.add_plugin(background::BackgroundPlugin);

    app
}
