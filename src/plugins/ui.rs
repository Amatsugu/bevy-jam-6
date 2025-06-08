use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
	components::{
		stats::{Health, MaxHealth},
		ui::{HealthBar, HealthBarText, HealthTextDisplayMode},
		utils::Cleanable,
		weapons::ProjectileType,
	},
	plugins::player::Player,
	resources::utils::{Fonts, KillCount},
	state_management::GameStartSystems,
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Update,
			(
				update_health_bars,
				update_health_text,
				update_kill_count_ui,
				selected_projectile_display,
			),
		);
		app.add_systems(Update, spawn_ui.in_set(GameStartSystems));
	}
}

#[derive(Component)]
struct KillCountUI;

fn spawn_ui(mut commands: Commands, window: Single<&Window, With<PrimaryWindow>>, fonts: Res<Fonts>) {
	let size = window.size();
	commands.spawn((
		Transform::from_xyz(0.0, (size.y / 2.0) - 20., 0.0),
		Text2d::new("Kills: 0"),
		TextFont {
			font: fonts.noto_thin.clone(),
			font_size: 30.,
			..default()
		},
		TextLayout::new_with_justify(JustifyText::Center),
		KillCountUI,
		Cleanable,
	));
	let pos = Vec3::new(0., (size.y / -2.) + 80., 10.);
	commands.spawn((
		Transform::from_translation(pos),
		ProjectileDisplay,
		Text2d::new("Projectile: 0"),
		TextFont {
			font: fonts.noto.clone(),
			font_size: 20.,
			..default()
		},
		TextLayout::new_with_justify(JustifyText::Center),
		Cleanable,
		children![(
			Transform::from_xyz(0.0, 20., 0.0),
			Text2d::new("[1] [2] [3] [4]"),
			TextFont {
				font: fonts.noto.clone(),
				font_size: 15.,
				..default()
			},
			TextLayout::new_with_justify(JustifyText::Center),
		)],
	));
}

fn update_kill_count_ui(mut text: Single<&mut Text2d, With<KillCountUI>>, count: Res<KillCount>) {
	text.0 = format!("KIlls: {}", count.0);
}

fn update_health_bars(health_bars: Query<(&mut Transform, &HealthBar)>, healths: Query<(&Health, &MaxHealth)>) {
	for (mut transform, bar) in health_bars {
		if let Ok((health, max_health)) = healths.get(bar.0) {
			let scale = health.0 / max_health.0;
			transform.scale = Vec3::new(scale.max(0.), 1.0, 1.0);
		}
	}
}

fn update_health_text(texts: Query<(&mut Text2d, &HealthBarText)>, healths: Query<(&Health, &MaxHealth)>) {
	for (mut text, health_text) in texts {
		if let Ok((health, max)) = healths.get(health_text.health_entity) {
			match health_text.display {
				HealthTextDisplayMode::Raw => {
					if health_text.show_max {
						text.0 = format!("{}/{}", health.0.round(), max.0.round());
					} else {
						text.0 = format!("{}", health.0.round());
					}
				}
				HealthTextDisplayMode::Percentage => {
					let p = (health.0 / max.0) * 100.;
					if health_text.show_max {
						text.0 = format!("{}%/100%", p.round());
					} else {
						text.0 = format!("{}%", p.round());
					}
				}
			}
		}
	}
}

#[derive(Component)]
struct ProjectileDisplay;

fn selected_projectile_display(
	player: Single<&ProjectileType, With<Player>>,
	mut display: Single<&mut Text2d, With<ProjectileDisplay>>,
) {
	let proj = player.into_inner();
	let name = match proj {
		ProjectileType::Basic { .. } => "[1] Basic",
		ProjectileType::Piercing { .. } => "[2] Piercing",
		ProjectileType::Bouncing { .. } => "[3] Bouncing",
		ProjectileType::Grenade { .. } => "[4] Grenade",
	};
	display.0 = format!("Projectile: {}", name);
}
