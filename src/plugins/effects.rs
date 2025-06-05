use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionEvent;

use crate::{
	components::{
		effects::{Explosion, ExplosionProgress},
		stats::{Damage, Health, Life},
	},
	plugins::projectiles::apply_damage,
	resources::effects::ExplosionMeshData,
};

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, create_meshes);
		app.add_systems(Update, (animate_explosions, handle_explosion_hits));
		app.add_systems(PostUpdate, init_explosions);
	}
}

fn create_meshes(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	commands.insert_resource(ExplosionMeshData {
		material: materials.add(Color::linear_rgba(304. / 255., 243. / 255., 161. / 255., 0.2)),
		mesh: meshes.add(Circle::new(1.0)),
	});
}

fn init_explosions(
	query: Query<Entity, (With<Explosion>, Without<Mesh2d>)>,
	mut commands: Commands,
	mesh_data: Res<ExplosionMeshData>,
) {
	for entity in query {
		commands.entity(entity).insert((
			Mesh2d(mesh_data.mesh.clone()),
			MeshMaterial2d(mesh_data.material.clone()),
		));
	}
}

fn animate_explosions(
	query: Query<(&mut Transform, &mut ExplosionProgress, &Explosion, Entity)>,
	time: Res<Time>,
	mut commands: Commands,
) {
	for (mut transform, mut prog, exp, entity) in query {
		if prog.0 > exp.range {
			commands.entity(entity).despawn();
			continue;
		}
		prog.0 += exp.epansion_rate * time.delta_secs();
		transform.scale = Vec3::splat(prog.0);
	}
}

fn handle_explosion_hits(
	mut explosions: Query<&Damage, With<Explosion>>,
	mut targets: Query<(&mut Health, &mut Life)>,
	mut collision_events: EventReader<CollisionEvent>,
) {
	for event in collision_events.read() {
		if let CollisionEvent::Started(entity_a, entity_b, _) = event {
			if let Ok(damage) = explosions.get_mut(*entity_a) {
				if let Ok((mut health, mut life)) = targets.get_mut(*entity_b) {
					apply_damage(&mut health, &mut life, damage);
				}
			} else if let Ok(damage) = explosions.get_mut(*entity_b) {
				if let Ok((mut health, mut life)) = targets.get_mut(*entity_a) {
					apply_damage(&mut health, &mut life, damage);
				}
			}
		}
	}
}
