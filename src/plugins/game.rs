use bevy::{
	core_pipeline::{
		bloom::Bloom,
		tonemapping::{DebandDither, Tonemapping},
	},
	prelude::*,
	window::PrimaryWindow,
};
use bevy_rapier2d::{
	plugin::RapierConfiguration,
	prelude::{Collider, Restitution},
};
#[cfg(feature = "inspect")]
use iyes_perf_ui::{
	PerfUiPlugin,
	prelude::{PerfUiEntryFPS, PerfUiEntryFrameTimeWorst, PerfUiEntryRenderGpuTime},
};
use rand::SeedableRng;
use rand_chacha::ChaChaRng;

use crate::{
	components::{tags::MainCamera, utils::Cleanable},
	plugins::{
		drops::DropsPlugin, effects::EffectsPlugin, game_over::GameOverPlugin, health::HealthPlugin,
		main_menu::MainMenuPlugin, spawner::EnemySpawnerPlugin, types::TypesPlugin, ui::UIPlugin,
		weapons::WeaponsPlugin,
	},
	resources::{
		audio::AudioClips,
		utils::{Fonts, RandomGen},
	},
	state_management::{
		GameCleanupSystems, GameOverSystems, GameStartSystems, GameWaitingSystems, GameplayState, GameplaySystems,
		ResetSystems,
	},
};

use super::{
	death::DeathPlugin, enemies::EnemiesPlugin, hooks::HooksPlugin, player::PlayerPlugin,
	projectiles::ProjectilesPlugin, utils::UtilsPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.insert_state(GameplayState::Reset);

		app.add_plugins((
			PlayerPlugin,
			EnemiesPlugin,
			EnemySpawnerPlugin,
			HooksPlugin,
			UtilsPlugin,
			TypesPlugin,
			ProjectilesPlugin,
			HealthPlugin,
			DeathPlugin,
			EffectsPlugin,
			WeaponsPlugin,
			MainMenuPlugin,
			GameOverPlugin,
			UIPlugin,
			DropsPlugin,
		));
		app.add_systems(PreStartup, (setup, spwan_bounds, load_auido));
		app.add_systems(PostStartup, disable_gravity);
		app.add_systems(Last, cleanup.in_set(GameCleanupSystems));
		app.add_systems(Last, reset_transition.in_set(ResetSystems));
		app.add_systems(Last, start_transition.in_set(GameStartSystems));

		app.insert_resource(RandomGen(ChaChaRng::seed_from_u64(0)));

		#[cfg(feature = "inspect")]
		{
			app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
				// .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
				.add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
				.add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin)
				.add_plugins(PerfUiPlugin);
		}

		setup_sets(app);
	}
}

fn setup_sets(app: &mut App) {
	app.configure_sets(PreUpdate, ResetSystems.run_if(in_state(GameplayState::Reset)));
	app.configure_sets(Update, ResetSystems.run_if(in_state(GameplayState::Reset)));
	app.configure_sets(PostUpdate, ResetSystems.run_if(in_state(GameplayState::Reset)));
	app.configure_sets(Last, ResetSystems.run_if(in_state(GameplayState::Reset)));

	app.configure_sets(PreUpdate, GameWaitingSystems.run_if(in_state(GameplayState::Waiting)));
	app.configure_sets(Update, GameWaitingSystems.run_if(in_state(GameplayState::Waiting)));
	app.configure_sets(PostUpdate, GameWaitingSystems.run_if(in_state(GameplayState::Waiting)));

	app.configure_sets(PostUpdate, GameStartSystems.run_if(in_state(GameplayState::Startup)));
	app.configure_sets(Update, GameStartSystems.run_if(in_state(GameplayState::Startup)));
	app.configure_sets(PreUpdate, GameStartSystems.run_if(in_state(GameplayState::Startup)));
	app.configure_sets(Last, GameStartSystems.run_if(in_state(GameplayState::Startup)));

	app.configure_sets(PreUpdate, GameplaySystems.run_if(in_state(GameplayState::Playing)));
	app.configure_sets(Update, GameplaySystems.run_if(in_state(GameplayState::Playing)));
	app.configure_sets(PostUpdate, GameplaySystems.run_if(in_state(GameplayState::Playing)));

	app.configure_sets(PreUpdate, GameOverSystems.run_if(in_state(GameplayState::GameOver)));
	app.configure_sets(Update, GameOverSystems.run_if(in_state(GameplayState::GameOver)));
	app.configure_sets(PostUpdate, GameOverSystems.run_if(in_state(GameplayState::GameOver)));

	app.configure_sets(PreUpdate, GameCleanupSystems.run_if(in_state(GameplayState::Cleanup)));
	app.configure_sets(Update, GameCleanupSystems.run_if(in_state(GameplayState::Cleanup)));
	app.configure_sets(PostUpdate, GameCleanupSystems.run_if(in_state(GameplayState::Cleanup)));
	app.configure_sets(Last, GameCleanupSystems.run_if(in_state(GameplayState::Cleanup)));
}

fn reset_transition(mut next: ResMut<NextState<GameplayState>>) {
	next.set(GameplayState::Waiting);
	info!("Moving to Waiting");
}

fn cleanup(query: Query<Entity, With<Cleanable>>, mut commands: Commands, mut next: ResMut<NextState<GameplayState>>) {
	for entity in query {
		commands.entity(entity).despawn();
	}
	next.set(GameplayState::Reset);
	info!("Moving to Reset");
}

fn start_transition(mut next: ResMut<NextState<GameplayState>>) {
	next.set(GameplayState::Playing);
	info!("Moving to Playing");
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn((
		Camera2d,
		MainCamera,
		Camera { hdr: true, ..default() },
		Tonemapping::AcesFitted,
		Bloom::default(),
		DebandDither::Enabled,
	));
	#[cfg(feature = "inspect")]
	commands.spawn((
		PerfUiEntryFPS::default(),
		PerfUiEntryRenderGpuTime::default(),
		PerfUiEntryFrameTimeWorst::default(),
	));

	commands.insert_resource(Fonts {
		noto: asset_server.load("fonts/NotoSans-VariableFont_wdth,wght.ttf"),
		noto_regular: asset_server.load("fonts/NotoSans-Regular.ttf"),
		noto_thin: asset_server.load("fonts/NotoSans-Thin.ttf"),
	});
}

fn disable_gravity(mut cfg: Single<&mut RapierConfiguration>) {
	cfg.gravity = Vec2::ZERO;
}

fn spwan_bounds(mut commands: Commands, window: Single<&Window, With<PrimaryWindow>>) {
	let size = window.size();
	//Left
	commands.spawn((
		Transform::from_xyz(-size.x / 2., 0.0, 0.0),
		Collider::cuboid(1., size.y / 2.),
		Restitution::coefficient(0.5),
	));
	//Right
	commands.spawn((
		Transform::from_xyz(size.x / 2., 0.0, 0.0),
		Collider::cuboid(1., size.y / 2.),
		Restitution::coefficient(0.5),
	));
	//Top
	commands.spawn((
		Transform::from_xyz(0.0, size.y / 2., 0.0),
		Collider::cuboid(size.x / 2., 1.),
		Restitution::coefficient(0.5),
	));
	//Bottom
	commands.spawn((
		Transform::from_xyz(0.0, size.y / -2., 0.0),
		Collider::cuboid(size.x / 2., 1.),
		Restitution::coefficient(0.5),
	));
}

fn load_auido(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.insert_resource(AudioClips {
		start: asset_server.load("sounds/start.wav"),
		explosion: asset_server.load("sounds/explosion.wav"),
		spiral: asset_server.load("sounds/spiral.wav"),
		dash: asset_server.load("sounds/dash.wav"),
		hit: asset_server.load("sounds/hit.wav"),
		hurt: asset_server.load("sounds/hurt.wav"),
		shoot_auto: asset_server.load("sounds/shoot_auto.wav"),
		shoot_spread: asset_server.load("sounds/shoot_spread.wav"),
		shoot_burst: asset_server.load("sounds/shoot_burst.wav"),
		gameover: asset_server.load("sounds/gameover.wav"),
		pickup: asset_server.load("sounds/powerUp.wav"),
		heal: asset_server.load("sounds/heal.wav"),
		weapon_switch: asset_server.load("sounds/weapon_switch.wav"),
	});
}
