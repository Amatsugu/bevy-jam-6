use crate::components::spawner::SpawnBatch;
use bevy::{ecs::entity_disabling::Disabled, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{
	ENEMY_GROUP,
	components::{
		ai::{ChargeAI, ChaseAI, HoverAI},
		death::{DeathScatter, ScatterPattern, Targeting},
		spawner::Spawner,
		stats::{Health, MoveSpeedStat},
		tags::Enemy,
	},
};

const SPAWNER_COUNT: usize = 5;
const SPAWNER_ANGLE: f32 = 360. / SPAWNER_COUNT as f32;

pub struct EnemySpawnerPlugin;

impl Plugin for EnemySpawnerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, (prepare_prefabs, create_spawners).chain());
		app.add_systems(Update, (spawners_batching, spawners_spawning));
		#[cfg(debug_assertions)]
		app.add_systems(Update, spawner_viz);
	}
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct Prefabs {
	pub charger: Entity,
	pub hover: Entity,
	pub chaser: Entity,
}

fn prepare_prefabs(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut meshes: ResMut<Assets<Mesh>>,
) {
	let charger = commands
		.spawn((
			Enemy,
			ChargeAI {
				charge_distance: 50.,
				charge_speed: 100.,
			},
			ActiveEvents::COLLISION_EVENTS,
			CollisionGroups::new(ENEMY_GROUP, Group::ALL),
			Health(100.),
			Mesh2d(meshes.add(Capsule2d::new(5.0, 10.0))),
			MeshMaterial2d(materials.add(Color::linear_rgb(1.0, 1.0, 0.0))),
			RigidBody::Dynamic,
			Velocity::zero(),
			MoveSpeedStat(30.),
			Collider::ball(4.),
			Disabled,
		))
		.id();

	let hover = commands
		.spawn((
			Enemy,
			HoverAI {
				hover_distance: 150.,
				range: 40.,
			},
			CollisionGroups::new(ENEMY_GROUP, Group::ALL),
			Health(100.),
			Mesh2d(meshes.add(RegularPolygon::new(5., 6))),
			MeshMaterial2d(materials.add(Color::linear_rgb(0.0, 1.0, 0.0))),
			RigidBody::Dynamic,
			Velocity::zero(),
			MoveSpeedStat(50.),
			Collider::ball(4.),
			Disabled,
			DeathScatter {
				count: 40,
				pattern: ScatterPattern::Spiral(30., 25.),
				damage: 10.,
			},
		))
		.id();

	let chaser = commands
		.spawn((
			Enemy,
			ChaseAI,
			CollisionGroups::new(ENEMY_GROUP, Group::ALL),
			Health(50.),
			Mesh2d(meshes.add(Circle::new(5.))),
			MeshMaterial2d(materials.add(Color::linear_rgb(0.0, 0.0, 1.0))),
			Velocity::zero(),
			MoveSpeedStat(40.),
			Collider::ball(4.),
			Disabled,
			DeathScatter {
				count: 50,
				pattern: ScatterPattern::Spread(360., Targeting::Forward),
				damage: 30.,
			},
		))
		.id();

	commands.insert_resource(Prefabs { charger, hover, chaser });
}

fn create_spawners(mut commands: Commands, prefabs: Res<Prefabs>) {
	for i in 0..SPAWNER_COUNT {
		let dir = (Vec2::from_angle((i as f32 * SPAWNER_ANGLE).to_radians()) * 400.).extend(0.);

		commands.spawn((
			Transform::from_translation(dir),
			Spawner {
				max_batch_size: 5,
				min_batch_size: 2,
				prefabs: vec![prefabs.hover],
				spawn_effect: Entity::PLACEHOLDER,
				spawn_range: 100.,
				spawn_rate: Timer::from_seconds(10., TimerMode::Repeating),
				spawn_speed: Timer::from_seconds(0.5, TimerMode::Repeating),
			},
		));
	}
}

#[cfg(debug_assertions)]
fn spawner_viz(mut gizmos: Gizmos, query: Query<(&Transform, &Spawner, &SpawnBatch)>) {
	for (transform, spawner, batch) in query {
		use std::f32::consts::PI;

		let color = if batch.0 > 0 {
			LinearRgba::GREEN
		} else {
			LinearRgba::RED
		};
		gizmos.circle_2d(transform.translation.xy(), spawner.spawn_range, color);
		gizmos.arc_2d(
			transform.translation.xy(),
			spawner.spawn_rate.fraction() * PI,
			spawner.spawn_range + 2.,
			LinearRgba::BLUE,
		);
		gizmos.arc_2d(
			transform.translation.xy(),
			spawner.spawn_speed.fraction() * -1. * PI,
			spawner.spawn_range - 2.,
			LinearRgba::rgb(0.0, 1.0, 1.0),
		);
	}
}

fn spawners_batching(query: Query<(&mut Spawner, &mut SpawnBatch)>, time: Res<Time>) {
	for (mut spawner, mut batch) in query {
		spawner.spawn_rate.tick(time.delta());
		if spawner.spawn_rate.finished() {
			batch.0 = spawner.max_batch_size;
			info!("Adding Batch");
		}
	}
}

fn spawners_spawning(
	query: Query<(&Transform, &mut Spawner, &mut SpawnBatch)>,
	time: Res<Time>,
	mut commands: Commands,
) {
	for (transform, mut spawner, mut batch) in query {
		if batch.0 == 0 {
			continue;
		}

		spawner.spawn_speed.tick(time.delta());
		if spawner.spawn_speed.finished() {
			commands
				.entity(spawner.prefabs[0])
				.clone_and_spawn_with(|builder| {
					builder.deny::<Disabled>();
				})
				.insert(Transform::from_translation(transform.translation));
			batch.0 -= 1;
		}
	}
}
