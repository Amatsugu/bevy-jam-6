use bevy::prelude::*;

use crate::{
	components::stats::{Health, MaxHealth},
	state_management::{GameOverSystems, GameplaySystems},
};

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(PostUpdate, clamp_health.in_set(GameplaySystems));
		app.add_systems(PostUpdate, clamp_health.in_set(GameOverSystems));
	}
}

fn clamp_health(query: Query<(&mut Health, &MaxHealth)>) {
	for (mut health, max) in query {
		health.0 = health.0.clamp(0., max.0);
	}
}
