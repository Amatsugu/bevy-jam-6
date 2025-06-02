use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
	ENEMY_PROJECTILE_GROUP,
	components::{
		ai::AI,
		death::{DeathScatter, ScatterPattern, SpiralSpawner, Targeting},
		stats::Damage,
		tags::Projectile,
		utils::Lifetime,
	},
};

use super::player::Player;

pub struct DeathPlugin;

impl Plugin for DeathPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, init_meshes);
		app.add_systems(Update, (death_scatter, sprial_spawner));
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
		mat: materials.add(Color::linear_rgb(1.0, 0.0, 0.16)),
	});
}

fn death_scatter(
	query: Query<(&Transform, &DeathScatter, &AI, Entity)>,
	player: Single<&Transform, With<Player>>,
	mut commands: Commands,
	mesh_data: Res<Projectiles>,
) {
	for (transform, scatter, ai, entity) in query {
		if ai.is_alive {
			continue;
		}

		match scatter.pattern {
			ScatterPattern::Radial => todo!(),
			ScatterPattern::Spread(arc, targeting) => {
				let aim = match targeting {
					Targeting::Forward => transform.up().as_vec3(),
					Targeting::Random => Quat::from_axis_angle(Vec3::Z, 0.1) * transform.up().as_vec3(),
					Targeting::Player => player.translation,
					Targeting::Closest => Vec3::ZERO,
				};
				let interval = arc / scatter.count as f32;
				for i in 0..scatter.count {
					let angle = (i as f32 * interval) - arc / 2.;
					let dir = (Quat::from_axis_angle(Vec3::Z, angle.to_radians()) * aim).xy();
					fire_projectile(
						&mut commands,
						transform.translation.xy() + dir * 20.,
						dir * 200.,
						scatter.damage,
						mesh_data.mesh.clone(),
						mesh_data.mat.clone(),
					);
				}
				commands.entity(entity).despawn();
			}
			ScatterPattern::Spiral(arc, rate) => {
				commands
					.entity(entity)
					.with_child((
						Name::new("Spiral"),
						Transform::IDENTITY,
						SpiralSpawner {
							timer: Timer::from_seconds(1.0 / rate, TimerMode::Repeating),
							arc: arc,
							count: scatter.count,
							damage: scatter.damage,
							mesh: mesh_data.mesh.clone(),
							material: mesh_data.mat.clone(),
							..default()
						},
					))
					.remove::<DeathScatter>();
			}
		};
	}
}

fn sprial_spawner(
	mut query: Query<(&GlobalTransform, &mut SpiralSpawner, &ChildOf)>,
	time: Res<Time>,
	mut commands: Commands,
) {
	for (transform, mut spiral, parent) in &mut query {
		if spiral.spawn_count >= spiral.count {
			commands.entity(parent.0).despawn();
			return;
		}
		spiral.timer.tick(time.delta());
		if spiral.timer.finished() {
			for _ in 0..spiral.timer.times_finished_this_tick() {
				if spiral.spawn_count >= spiral.count {
					break;
				}
				spiral.spawn_count += 1;
				let angle = spiral.arc * spiral.spawn_count as f32;
				let dir = (Quat::from_axis_angle(Vec3::Z, angle.to_radians()) * transform.up()).xy();
				fire_projectile(
					&mut commands,
					transform.translation().xy() + dir * 20.,
					dir * 200.,
					spiral.damage,
					spiral.mesh.clone(),
					spiral.material.clone(),
				);
			}
		}
	}
}

fn fire_projectile(
	commands: &mut Commands,
	origin: Vec2,
	vel: Vec2,
	damage: f32,
	mesh: Handle<Mesh>,
	material: Handle<ColorMaterial>,
) {
	commands.spawn((
		Projectile::enemy(),
		Damage(damage),
		Transform::from_translation(origin.extend(0.)),
		Lifetime::new(5.),
		RigidBody::Dynamic,
		Velocity::linear(vel),
		Mesh2d(mesh.clone()),
		MeshMaterial2d(material.clone()),
		Collider::ball(0.5),
		CollisionGroups::new(ENEMY_PROJECTILE_GROUP, Group::ALL ^ ENEMY_PROJECTILE_GROUP),
	));
}
