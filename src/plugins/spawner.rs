use std::time::Duration;

use crate::{
	components::{spawner::SpawnBatch, stats::MaxHealth, utils::Cleanable},
	resources::utils::RandomGen,
	state_management::{GameStartSystems, GameplaySystems},
};
use bevy::{ecs::entity_disabling::Disabled, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{
	ENEMY_GROUP,
	components::{
		ai::{ChargeAI, ChaseAI, HoverAI},
		death::{DeathScatter, ScatterPattern, Targeting},
		spawner::Spawner,
		stats::MoveSpeedStat,
		tags::Enemy,
	},
};

const SPAWNER_COUNT: usize = 3;
const SPAWNER_ANGLE: f32 = 360. / SPAWNER_COUNT as f32;

pub struct EnemySpawnerPlugin;

impl Plugin for EnemySpawnerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, prepare_prefabs);
		app.add_systems(Update, create_spawners.in_set(GameStartSystems));
		app.add_systems(Update, (spawners_batching, spawners_spawning).in_set(GameplaySystems));
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
			Name::new("Charger"),
			ChargeAI {
				distance: 200.,
				speed_multi: 20.,
				hit_damage: 70.,
			},
			ActiveEvents::COLLISION_EVENTS,
			CollisionGroups::new(ENEMY_GROUP, Group::ALL),
			MaxHealth(100.),
			Mesh2d(meshes.add(Capsule2d::new(5.0, 10.0))),
			MeshMaterial2d(materials.add(Color::linear_rgb(1.0, 1.0, 0.0))),
			RigidBody::Dynamic,
			Damping {
				linear_damping: 1.,
				..default()
			},
			MoveSpeedStat(30.),
			Collider::ball(4.),
			DeathScatter {
				count: 20,
				pattern: ScatterPattern::Spread {
					arc: 30.,
					targeting: Targeting::Forward,
				},
				damage: 20.,
			},
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
			MaxHealth(100.),
			Mesh2d(meshes.add(RegularPolygon::new(5., 6))),
			MeshMaterial2d(materials.add(Color::linear_rgb(0.0, 1.0, 0.0))),
			RigidBody::Dynamic,
			Damping {
				linear_damping: 1.,
				..default()
			},
			MoveSpeedStat(50.),
			Collider::ball(4.),
			Disabled,
			DeathScatter {
				count: 40,
				pattern: ScatterPattern::Spiral { angle: 370., rate: 50. },
				damage: 10.,
			},
		))
		.id();

	let chaser = commands
		.spawn((
			Enemy,
			ChaseAI,
			CollisionGroups::new(ENEMY_GROUP, Group::ALL),
			MaxHealth(50.),
			Mesh2d(meshes.add(Circle::new(5.))),
			MeshMaterial2d(materials.add(Color::linear_rgb(0.0, 0.0, 1.0))),
			RigidBody::Dynamic,
			Damping {
				linear_damping: 1.,
				..default()
			},
			MoveSpeedStat(40.),
			Collider::ball(4.),
			Disabled,
			DeathScatter {
				count: 50,
				pattern: ScatterPattern::Explosion {
					range: 100.,
					speed: 300.,
				},
				damage: 40.,
			},
		))
		.id();

	commands.insert_resource(Prefabs { charger, hover, chaser });
}

fn create_spawners(mut commands: Commands, prefabs: Res<Prefabs>) {
	for i in 0..SPAWNER_COUNT {
		let dir = (Vec2::from_angle((i as f32 * SPAWNER_ANGLE).to_radians()) * 400.).extend(0.);
		let mut timer = Timer::from_seconds(10., TimerMode::Repeating);
		timer.set_elapsed(Duration::from_secs(8));
		commands.spawn((
			Transform::from_translation(dir),
			Cleanable,
			Spawner {
				max_batch_size: 8,
				min_batch_size: 2,
				prefabs: vec![prefabs.chaser, prefabs.charger, prefabs.hover],
				spawn_effect: Entity::PLACEHOLDER,
				spawn_range: 100.,
				spawn_rate: timer,
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

fn spawners_batching(query: Query<(&mut Spawner, &mut SpawnBatch)>, time: Res<Time>, mut rng: ResMut<RandomGen>) {
	for (mut spawner, mut batch) in query {
		spawner.spawn_rate.tick(time.delta());
		if spawner.spawn_rate.finished() {
			batch.0 = rng.range(spawner.min_batch_size..spawner.max_batch_size);
			if rng.range(0..100) <= 25 {
				spawner.max_batch_size += 1;
			}
		}
	}
}

fn spawners_spawning(
	query: Query<(&Transform, &mut Spawner, &mut SpawnBatch)>,
	time: Res<Time>,
	mut commands: Commands,
	mut rng: ResMut<RandomGen>,
) {
	for (transform, mut spawner, mut batch) in query {
		if batch.0 == 0 {
			continue;
		}

		spawner.spawn_speed.tick(time.delta());
		if spawner.spawn_speed.finished() {
			let pos = transform.translation + rng.point_on_circle_vec3(spawner.spawn_range);
			let idx = rng.range(0..spawner.prefabs.len());
			commands
				.entity(spawner.prefabs[idx])
				.clone_and_spawn_with(|builder| {
					builder.deny::<Disabled>();
				})
				.insert(Transform::from_translation(pos));
			batch.0 -= 1;
		}
	}
}
