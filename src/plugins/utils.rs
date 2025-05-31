use bevy::prelude::*;

use crate::components::utils::Lifetime;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
	fn build(&self, app: &mut App) {
		// app.add_systems(Startup, register_lifetime_hook);
		app.add_systems(PostUpdate, process_lifetimes);
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
