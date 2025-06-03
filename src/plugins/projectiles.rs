use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{
	stats::{Damage, Health, Life},
	tags::{Piercing, Projectile},
};

pub struct ProjectilesPlugin;

impl Plugin for ProjectilesPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, handle_projectiles);
	}
}

fn handle_projectiles(
	mut projectiles: Query<(Entity, &Damage, Option<&mut Piercing>), With<Projectile>>,
	mut targets: Query<(&mut Health, &mut Life)>,
	mut collision_events: EventReader<CollisionEvent>,
	mut commands: Commands,
) {
	for event in collision_events.read() {
		if let CollisionEvent::Started(entity_a, entity_b, _) = event {
			if let Ok((e, damage, mut piercing)) = projectiles.get_mut(*entity_a) {
				if let Ok((mut health, mut life)) = targets.get_mut(*entity_b) {
					apply_damage(&mut health, &mut life, damage);
				}
				process_piercing(&mut piercing, e, &mut commands);
			} else if let Ok((e, damage, mut piercing)) = projectiles.get_mut(*entity_b) {
				if let Ok((mut health, mut life)) = targets.get_mut(*entity_a) {
					apply_damage(&mut health, &mut life, damage);
				}
				process_piercing(&mut piercing, e, &mut commands);
			}
		}
	}
}
pub fn process_piercing(piercing: &mut Option<Mut<'_, Piercing>>, entity: Entity, commands: &mut Commands) {
	if let Some(p) = piercing {
		p.0 -= 1;
		if p.0 == 0 {
			commands.entity(entity).try_despawn();
		}
	} else {
		commands.entity(entity).try_despawn();
	}
}

pub fn apply_damage(health: &mut Health, life: &mut Life, damage: &Damage) {
	health.0 -= damage.0;
	if health.0 <= 0. {
		life.0 = false;
	}
}
