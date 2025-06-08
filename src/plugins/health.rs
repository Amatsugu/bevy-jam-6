use bevy::prelude::*;

use crate::{
	components::stats::{Health, HealthRegen, MaxHealth},
	state_management::{GameOverSystems, GameplaySystems},
};

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(PostUpdate, clamp_health.in_set(GameplaySystems));
		app.add_systems(PostUpdate, clamp_health.in_set(GameOverSystems));
		app.add_systems(Update, health_regen.in_set(GameplaySystems));
	}
}

fn clamp_health(query: Query<(&mut Health, &MaxHealth)>) {
	for (mut health, max) in query {
		health.0 = health.0.clamp(0., max.0);
	}
}

fn health_regen(query: Query<(&mut Health, &HealthRegen, &MaxHealth)>, time: Res<Time>) {
	for (mut health, regen, max) in query {
		if health.0 < max.0 {
			health.0 += regen.0 * time.delta_secs();
			if health.0 > max.0 {
				health.0 = max.0;
			}
		}
	}
}
