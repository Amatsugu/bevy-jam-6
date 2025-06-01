mod components;
mod plugins;

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use plugins::GamePlugin;

const NAME: &str = "Bevy Jam 6";

fn main() {
	App::new()
		.add_plugins((
			DefaultPlugins
				.set(AssetPlugin {
					meta_check: AssetMetaCheck::Never,
					..default()
				})
				.set(WindowPlugin {
					primary_window: Some(Window {
						title: NAME.into(),
						name: Some(NAME.into()),
						#[cfg(debug_assertions)]
						resolution: (1920., 1080.).into(),
						#[cfg(debug_assertions)]
						present_mode: PresentMode::AutoNoVsync,
						..default()
					}),
					..default()
				}),
			#[cfg(debug_assertions)]
			EguiPlugin {
				enable_multipass_for_primary_context: true,
			},
			#[cfg(debug_assertions)]
			WorldInspectorPlugin::new(),
			RapierPhysicsPlugin::<NoUserData>::default(),
			GamePlugin,
		))
		.run();
}
