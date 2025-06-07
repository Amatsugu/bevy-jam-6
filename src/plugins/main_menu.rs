use bevy::prelude::*;

use crate::{
	NAME,
	components::tags::MainMenu,
	plugins::utils::play_audio_onshot,
	resources::{audio::AudioClips, utils::Fonts},
	state_management::{GameWaitingSystems, GameplayState, GameplaySystems, ResetSystems},
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, spawn_menu.in_set(ResetSystems));
		app.add_systems(Update, menu.in_set(GameWaitingSystems));
		app.add_systems(PreUpdate, clean_menu.in_set(GameplaySystems));
	}
}

fn spawn_menu(mut commands: Commands, fonts: Res<Fonts>) {
	commands.spawn((
		MainMenu,
		Transform::from_xyz(0.0, 50., 0.0),
		Text2d::new(NAME.to_string()),
		TextFont {
			font: fonts.noto_thin.clone(),
			font_size: 100.,
			..default()
		},
		TextLayout::new_with_justify(JustifyText::Center),
	));

	commands.spawn((
		MainMenu,
		Transform::from_xyz(0.0, -30., 0.0),
		Text2d::new("Press [SPACE] to Start"),
		TextFont {
			font: fonts.noto_thin.clone(),
			font_size: 20.,
			..default()
		},
		TextLayout::new_with_justify(JustifyText::Center),
	));
}

fn clean_menu(query: Query<Entity, With<MainMenu>>, mut commands: Commands) {
	for entity in query {
		commands.entity(entity).despawn();
	}
}

fn menu(
	key: Res<ButtonInput<KeyCode>>,
	mut next: ResMut<NextState<GameplayState>>,
	mut commands: Commands,
	audio: Res<AudioClips>,
) {
	if key.just_pressed(KeyCode::Space) {
		info!("Moving to Waiting");
		next.set(GameplayState::Startup);
		play_audio_onshot(&mut commands, audio.start.clone());
	}
}
