mod components;
mod plugins;
mod resources;
mod state_management;

use bevy::asset::AssetMetaCheck;
use bevy::audio::{AudioPlugin, SpatialScale};
use bevy::prelude::*;
#[cfg(debug_assertions)]
use bevy::window::PresentMode;
#[cfg(feature = "inspect")]
use bevy_inspector_egui::bevy_egui::EguiPlugin;
#[cfg(feature = "inspect")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use plugins::GamePlugin;

const NAME: &str = "Cataclyze";

const AUDIO_SCALE: f32 = 1. / 100.0;

const PLAYER_GROUP: Group = Group::from_bits_truncate(0b0001);
const PLAYER_PROJECTILE_GROUP: Group = Group::from_bits_truncate(0b0010);
const PLAYER_OWNED_GROUP: Group = Group::from_bits_truncate(0b0011);
const ENEMY_GROUP: Group = Group::from_bits_truncate(0b0100);
const ENEMY_PROJECTILE_GROUP: Group = Group::from_bits_truncate(0b1000);
#[allow(dead_code)]
const ENEMY_OWNED_GROUP: Group = Group::from_bits_truncate(0b1100);
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
				})
				.set(AudioPlugin {
					default_spatial_scale: SpatialScale::new_2d(AUDIO_SCALE),
					..default()
				}),
			RapierPhysicsPlugin::<NoUserData>::default(),
			GamePlugin,
			#[cfg(feature = "inspect")]
			EguiPlugin {
				enable_multipass_for_primary_context: true,
			},
			#[cfg(feature = "inspect")]
			WorldInspectorPlugin::new(),
			#[cfg(feature = "phys")]
			RapierDebugRenderPlugin::default(),
		))
		.run();
}
