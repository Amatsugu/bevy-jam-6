use std::f32::consts::PI;

use bevy::{audio::Volume, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{
	ENEMY_PROJECTILE_GROUP,
	components::{
		death::{DeathScatter, ScatterPattern, SpiralSpawner, Targeting},
		effects::Explosion,
		stats::{Damage, Life},
		tags::Projectile,
		utils::Lifetime,
	},
	plugins::utils::play_audio_onshot,
	resources::{
		audio::AudioClips,
		utils::{DeathEvent, KillCount, RandomGen},
	},
	state_management::{GameOverSystems, GameplaySystems},
};

use super::player::Player;

pub struct DeathPlugin;

impl Plugin for DeathPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<KillCount>();
		app.add_event::<DeathEvent>();
		app.add_systems(Startup, init_meshes);
		app.add_systems(
			Update,
			(death_events, death_scatter, sprial_spawner)
				.chain()
				.in_set(GameplaySystems),
		);
		app.add_systems(PostUpdate, (death_scatter, sprial_spawner).in_set(GameOverSystems));
		app.add_systems(Update, kill_count.in_set(GameplaySystems));
	}
}

fn kill_count(mut events: EventReader<DeathEvent>, mut kill_count: ResMut<KillCount>) {
	for event in events.read() {
		if !event.is_player {
			kill_count.0 += 1;
		}
	}
}

fn death_events(query: Query<(&mut Life, &Transform, Option<&Player>)>, mut events: EventWriter<DeathEvent>) {
	for (mut life, transform, player) in query {
		if life.is_alive() || life.1 {
			continue;
		}
		life.1 = true;
		events.write(DeathEvent {
			pos: transform.translation.xy(),
			is_player: player.is_some(),
		});
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
		mat: materials.add(Color::linear_rgb(3.0, 0.0, 0.48)),
	});
}

fn death_scatter(
	query: Query<(&Transform, &DeathScatter, &Life, Entity)>,
	player: Single<&Transform, With<Player>>,
	mut commands: Commands,
	mesh_data: Res<Projectiles>,
	mut rng: ResMut<RandomGen>,
	audio: Res<AudioClips>,
) {
	for (transform, scatter, life, entity) in query {
		if life.is_alive() {
			continue;
		}

		match scatter.pattern {
			ScatterPattern::Explosion { range, speed } => {
				commands.spawn((
					Explosion {
						range,
						epansion_rate: speed,
					},
					Damage(scatter.damage),
					Transform::from_translation(transform.translation).with_scale(Vec3::splat(0.01)),
					ActiveEvents::COLLISION_EVENTS,
					CollisionGroups::new(ENEMY_PROJECTILE_GROUP, Group::ALL),
					Collider::ball(1.),
				));
				play_audio_onshot(&mut commands, audio.explosion.clone());
				commands.entity(entity).despawn();
			}
			ScatterPattern::Spread { arc, targeting } => {
				let aim = match targeting {
					Targeting::Forward => transform.up().as_vec3(),
					Targeting::Random => {
						let angle = rng.range(-PI..PI);
						Vec3::new(angle.cos(), angle.sin(), 0.0)
					}
					Targeting::Player => (player.translation - transform.translation).normalize_or(Vec3::Y),
				};
				let interval = arc / scatter.count as f32;
				let mesh = mesh_data.mesh.clone();
				let mat = mesh_data.mat.clone();
				let dmg = scatter.damage;
				let base_pos = transform.translation.xy();
				let bulk = (0..scatter.count).map(move |i| {
					let angle = (i as f32 * interval) - arc / 2.;
					let dir = (Quat::from_axis_angle(Vec3::Z, angle.to_radians()) * aim).xy();
					return get_projectile(base_pos + dir * 20., dir * 200., dmg, mesh.clone(), mat.clone());
				});
				commands.spawn_batch(bulk);
				commands.entity(entity).despawn();
			}
			ScatterPattern::Spiral { angle, rate } => {
				commands
					.entity(entity)
					.with_child((
						Name::new("Spiral"),
						Transform::IDENTITY,
						SpiralSpawner {
							angle,
							timer: Timer::from_seconds(1.0 / rate, TimerMode::Repeating),
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
	audio: Res<AudioClips>,
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
				let angle = spiral.angle * spiral.spawn_count as f32;
				let dir = Vec2::from_angle(angle.to_radians());
				commands.spawn((
					get_projectile(
						transform.translation().xy() + dir * 20.,
						dir * 200.,
						spiral.damage,
						spiral.mesh.clone(),
						spiral.material.clone(),
					),
					AudioPlayer::new(audio.spiral.clone()),
					PlaybackSettings::ONCE.with_volume(Volume::Linear(0.5)),
				));
			}
		}
	}
}

#[derive(Bundle)]
struct ProjBundle {
	proj: Projectile,
	damage: Damage,
	transform: Transform,
	rigidbody: RigidBody,
	vel: Velocity,
	mesh: Mesh2d,
	life: Lifetime,
	material: MeshMaterial2d<ColorMaterial>,
	collider: Collider,
	sensor: Sensor,
	groups: CollisionGroups,
	active: ActiveEvents,
}

fn get_projectile(
	origin: Vec2,
	vel: Vec2,
	damage: f32,
	mesh: Handle<Mesh>,
	material: Handle<ColorMaterial>,
) -> ProjBundle {
	return ProjBundle {
		proj: Projectile::enemy(),
		active: ActiveEvents::COLLISION_EVENTS,
		damage: Damage(damage),
		transform: Transform::from_translation(origin.extend(0.)),
		life: Lifetime::new(5.),
		rigidbody: RigidBody::Dynamic,
		vel: Velocity::linear(vel),
		mesh: Mesh2d(mesh.clone()),
		material: MeshMaterial2d(material.clone()),
		collider: Collider::ball(0.5),
		groups: CollisionGroups::new(ENEMY_PROJECTILE_GROUP, Group::ALL ^ ENEMY_PROJECTILE_GROUP),
		sensor: Sensor,
	};
}
