use std::collections::HashMap;

use bevy::{ecs::world::CommandQueue, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::components::{
	stats::{Damage, Health},
	tags::{Enemy, Owner, Projectile},
};

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
				collisions.0.insert(*entity_b, *entity_a);
			}
			CollisionEvent::Stopped(a, b, _) => {
				collisions.0.remove_entry(a);
				collisions.0.remove_entry(b);
			}
		};
	}
}

fn handle_projectiles(
	query: Query<(&Projectile, &Damage, Entity)>,
	collisions: Res<ActiveCollisions>,
	mut commands: Commands,
) {
	let mut queue = CommandQueue::default();
	for (proj, damage, entity) in query {
		let proj_type = proj.0;
		if let Some(hit) = collisions.0.get(&entity) {
			let t = hit.clone();
			let dmg = damage.0;

			queue.push(move |world: &mut World| {
				match proj_type {
					Owner::Player => {
						if world.get::<Enemy>(t).is_some() {
							if let Some(mut health) = world.get_mut::<Health>(t) {
								health.0 -= dmg;
							}
						}
					}
					Owner::Enemy => {
						if let Some(mut health) = world.get_mut::<Health>(t) {
							health.0 -= dmg;
						}
					}
				}
				let mut c = world.commands();
				let mut ec = c.entity(entity);
				ec.despawn();
			});
		}
		commands.append(&mut queue);
	}
}
