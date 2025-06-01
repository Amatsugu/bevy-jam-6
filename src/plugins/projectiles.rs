use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::stats::Health;

pub struct ProjectilesPlugin;

impl Plugin for ProjectilesPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<ActiveCollisions>();
		app.add_systems(Update, (handle_collision_events, handle_projectiles).chain());
	}
}
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
struct ActiveCollisions(HashMap<Entity, Entity>);

fn handle_collision_events(
	mut collision_events: EventReader<CollisionEvent>,
	mut collisions: ResMut<ActiveCollisions>,
) {
	for event in collision_events.read() {
		match event {
			CollisionEvent::Started(entity_a, entity_b, _) => {
				collisions.0.insert(*entity_a, *entity_b);
			}
			CollisionEvent::Stopped(a, _, _) => {
				collisions.0.remove_entry(a);
			}
		};
	}
}

fn handle_projectiles(mut query: Query<(&mut Health, Entity)>, collisions: Res<ActiveCollisions>) {
	for (mut _health, entity) in &mut query {
		if !collisions.0.contains_key(&entity) {
			continue;
		}
	}
}
