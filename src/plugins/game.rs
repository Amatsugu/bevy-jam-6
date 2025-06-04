use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;
#[cfg(debug_assertions)]
use iyes_perf_ui::{
	PerfUiPlugin,
	prelude::{PerfUiEntryFPS, PerfUiEntryFrameTimeWorst, PerfUiEntryRenderGpuTime},
};
use rand::SeedableRng;
use rand_chacha::ChaChaRng;

use crate::{
	components::tags::MainCamera,
	plugins::{effects::EffectsPlugin, spawner::EnemySpawnerPlugin, types::TypesPlugin},
	resources::utils::RNG,
};

use super::{
	death::DeathPlugin, enemies::EnemiesPlugin, hooks::HooksPlugin, player::PlayerPlugin,
	projectiles::ProjectilesPlugin, utils::UtilsPlugin,
};

#[derive(Default)]

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			PlayerPlugin,
			EnemiesPlugin,
			EnemySpawnerPlugin,
			HooksPlugin,
			UtilsPlugin,
			TypesPlugin,
			ProjectilesPlugin,
			DeathPlugin,
			EffectsPlugin,
		));
		app.add_systems(Startup, (setup, disable_gravity));

		app.insert_resource(RNG(ChaChaRng::seed_from_u64(0)));

		#[cfg(debug_assertions)]
		{
			app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
				// .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
				.add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
				.add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin)
				.add_plugins(PerfUiPlugin);
		}
	}
}

fn setup(mut commands: Commands) {
	commands.spawn((Camera2d, MainCamera));
	#[cfg(debug_assertions)]
	commands.spawn((
		PerfUiEntryFPS::default(),
		PerfUiEntryRenderGpuTime::default(),
		PerfUiEntryFrameTimeWorst::default(),
	));
}

fn disable_gravity(mut cfg: Single<&mut RapierConfiguration>) {
	cfg.gravity = Vec2::ZERO;
}
