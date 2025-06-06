use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{
	stats::{Damage, Health, Life},
	tags::{ContactLimit, Owner, Projectile},
};

pub struct ProjectilesPlugin;

impl Plugin for ProjectilesPlugin {
	fn build(&self, app: &mut App) {
		app.register_type::<Projectiles>();
		app.add_systems(Startup, init_meshes);
		app.add_systems(Update, handle_projectiles);
		app.add_systems(PostUpdate, init_projectiles);
	}
}
#[derive(Resource, Reflect, Default)]
struct Projectiles {
	mesh: Handle<Mesh>,
	mat: Handle<ColorMaterial>,
}

fn init_meshes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
	commands.insert_resource(Projectiles {
		mesh: meshes.add(Circle::new(2.)),
		mat: materials.add(Color::linear_rgb(1.0, 0.6, 0.16)),
	});
}
fn init_projectiles(
	projectiles: Query<(&Projectile, Entity), Without<Mesh2d>>,
	proj_mesh: Res<Projectiles>,
	mut commands: Commands,
) {
	for (projectile, entity) in projectiles {
		match projectile.0 {
			Owner::Player => {
				commands
					.entity(entity)
					.insert((Mesh2d(proj_mesh.mesh.clone()), MeshMaterial2d(proj_mesh.mat.clone())));
			}
			Owner::Enemy => (),
		}
	}
}

fn handle_projectiles(
	mut projectiles: Query<(Entity, &Damage, &mut ContactLimit), With<Projectile>>,
	mut targets: Query<(&mut Health, &mut Life)>,
	mut collision_events: EventReader<CollisionEvent>,
	mut commands: Commands,
) {
	for event in collision_events.read() {
		if let CollisionEvent::Started(entity_a, entity_b, _) = event {
			if let Ok((e, damage, mut contacts)) = projectiles.get_mut(*entity_a) {
				if let Ok((mut health, mut life)) = targets.get_mut(*entity_b) {
					apply_damage(&mut health, &mut life, damage);
				}
				process_contacts(&mut contacts, e, &mut commands);
			} else if let Ok((e, damage, mut contacts)) = projectiles.get_mut(*entity_b) {
				if let Ok((mut health, mut life)) = targets.get_mut(*entity_a) {
					apply_damage(&mut health, &mut life, damage);
				}
				process_contacts(&mut contacts, e, &mut commands);
			}
		}
	}
}
pub fn process_contacts(contacts: &mut ContactLimit, entity: Entity, commands: &mut Commands) {
	if contacts.0 > 0 {
		contacts.0 -= 1;
	}
	if contacts.0 == 0 {
		commands.entity(entity).try_despawn();
	}
}

pub fn apply_damage(health: &mut Health, life: &mut Life, damage: &Damage) {
	health.0 -= damage.0;
	if health.0 <= 0. {
		life.0 = false;
	}
}
