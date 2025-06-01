use bevy::prelude::*;

use crate::components::{
	stats::{MoveSpeed, MoveSpeedMultiplier, MoveSpeedStat},
	utils::Lifetime,
};

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
	fn build(&self, app: &mut App) {
		// app.add_systems(Startup, register_lifetime_hook);
		app.add_systems(PostUpdate, process_lifetimes);
		app.add_systems(PreUpdate, process_move_speed);
	}
}
fn process_lifetimes(mut query: Query<(Entity, &mut Lifetime)>, time: Res<Time>, mut commands: Commands) {
	let delta = time.delta();
	for (entity, mut despawn) in query.iter_mut() {
		despawn.0.tick(delta);
		if despawn.0.finished() {
			commands.entity(entity).despawn();
		}
	}
}

fn process_move_speed(mut query: Query<(&mut MoveSpeed, &MoveSpeedStat, &MoveSpeedMultiplier)>) {
	for (mut adj, speed, multi) in &mut query {
		adj.0 = speed.0 * multi.0;
	}
}
