use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
	ENEMY_OWNED_GROUP, PLAYER_OWNED_GROUP, PLAYER_PROJECTILE_GROUP,
	components::{
		death::{DeathScatter, ScatterPattern},
		stats::{Damage, Life},
		tags::{ContactLimit, Owner, Projectile},
		utils::Lifetime,
		weapons::*,
	},
	resources::utils::RandomGen,
};

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, weapon_firing);
	}
}

fn weapon_firing(
	query: Query<(
		&Transform,
		&WeaponFiring,
		&Life,
		&Weapon,
		&mut WeaponBeam,
		&mut WeaponAuto,
		&mut WeaponBurst,
		&mut WeaponSpread,
		&ProjectileType,
	)>,
	time: Res<Time>,
	mut commands: Commands,
	mut rng: ResMut<RandomGen>,
) {
	for (transform, firing, life, weapon, mut _beam, mut auto, mut _burst, mut _spread, proj) in query {
		if life.is_dead() || firing.is_not_firing() {
			continue;
		}
		let aim = transform.up().as_vec3();
		match weapon {
			Weapon::Auto => {
				auto.fire_rate.tick(time.delta());
				if auto.fire_rate.finished() {
					let volley = proj.multishot() * auto.fire_rate.times_finished_this_tick();
					let bundle = prepare_auto_volley(volley, aim, transform.translation, &auto, proj, &mut rng);
					match bundle {
						ProjBatch::Normal(proj_bundles) => commands.spawn_batch(proj_bundles),
						ProjBatch::Sensor(sensor_projs) => commands.spawn_batch(sensor_projs),
						ProjBatch::Scatter(scatter_projs) => commands.spawn_batch(scatter_projs),
					}
				}
			}
			Weapon::Spread => todo!(),
			Weapon::Burst => todo!(),
			Weapon::Beam => todo!(),
		}
	}
}

#[derive(Bundle, Default)]
struct ProjBundle {
	proj: Projectile,
	dmg: Damage,
	transform: Transform,
	vel: Velocity,
	rigidbody: RigidBody,
	active_events: ActiveEvents,
	collider: Collider,
	groups: CollisionGroups,
	contacts: ContactLimit,
	life: Lifetime,
	drag: Damping,
}

#[derive(Bundle, Default)]
struct SensorProj(ProjBundle, Sensor);

#[derive(Bundle)]
struct ScatterProj(ProjBundle, DeathScatter);

enum ProjBatch {
	Normal(Vec<ProjBundle>),
	Sensor(Vec<SensorProj>),
	Scatter(Vec<ScatterProj>),
}

impl Default for ProjBatch {
	fn default() -> Self {
		Self::Normal(Vec::new())
	}
}
const PROJECTILE_SIZE: f32 = 2.;
const PROJECTILE_LIFETIME: f32 = 5.;

fn prepare_auto_volley(
	volley: u32,
	aim: Vec3,
	pos: Vec3,
	auto: &WeaponAuto,
	proj: &ProjectileType,
	rng: &mut RandomGen,
) -> ProjBatch {
	match proj {
		ProjectileType::Basic { damage, speed, .. } => {
			let bundles = determine_aim_and_pos(pos, aim, 10., auto.accuracy / 2., volley, rng)
				.iter()
				.map(|(aim, pos)| {
					fire_projectile(
						*pos,
						aim * speed * auto.speed_multi,
						damage * auto.damage_multi,
						PROJECTILE_LIFETIME,
						PROJECTILE_SIZE,
						PLAYER_PROJECTILE_GROUP,
						Group::ALL ^ PLAYER_OWNED_GROUP,
						1,
						0.0,
						Owner::Player,
					)
				})
				.collect();
			ProjBatch::Normal(bundles)
		}
		ProjectileType::Piercing {
			damage,
			speed,
			penetration,
			..
		} => {
			let bundles = determine_aim_and_pos(pos, aim, 10., auto.accuracy / 2., volley, rng)
				.iter()
				.map(|(aim, pos)| {
					fire_sensor_projectile(
						*pos,
						aim * speed * auto.speed_multi,
						damage * auto.damage_multi,
						PROJECTILE_LIFETIME,
						PROJECTILE_SIZE,
						PLAYER_PROJECTILE_GROUP,
						ENEMY_OWNED_GROUP,
						*penetration,
						0.0,
						Owner::Player,
					)
				})
				.collect();
			ProjBatch::Sensor(bundles)
		}
		ProjectileType::Bouncing {
			damage,
			speed,
			bounce_limit,
			..
		} => {
			let bundles = determine_aim_and_pos(pos, aim, 10., auto.accuracy / 2., volley, rng)
				.iter()
				.map(|(aim, pos)| {
					fire_projectile(
						*pos,
						aim * speed * auto.speed_multi,
						damage * auto.damage_multi,
						PROJECTILE_LIFETIME,
						PROJECTILE_SIZE,
						PLAYER_PROJECTILE_GROUP,
						ENEMY_OWNED_GROUP,
						*bounce_limit,
						0.0,
						Owner::Player,
					)
				})
				.collect();
			ProjBatch::Normal(bundles)
		}
		ProjectileType::Grenade {
			damage,
			speed,
			bounce_limit,
			drag,
			explosive_range,
			explosive_speed,
			..
		} => {
			let bundles = determine_aim_and_pos(pos, aim, 10., auto.accuracy / 2., volley, rng)
				.iter()
				.map(|(aim, pos)| {
					fire_scatter_projectile(
						*pos,
						aim * speed * auto.speed_multi,
						0.0,
						PROJECTILE_LIFETIME,
						PROJECTILE_SIZE,
						PLAYER_PROJECTILE_GROUP,
						ENEMY_OWNED_GROUP,
						*bounce_limit,
						*drag,
						DeathScatter {
							damage: damage * auto.damage_multi,
							pattern: ScatterPattern::Explosion {
								range: *explosive_range,
								speed: *explosive_speed,
							},
							..default()
						},
						Owner::Player,
					)
				})
				.collect();
			ProjBatch::Scatter(bundles)
		}
	}
}

fn determine_aim_and_pos(
	origin: Vec3,
	base_aim: Vec3,
	offset: f32,
	spread: f32,
	count: u32,
	rng: &mut RandomGen,
) -> Vec<(Vec2, Vec3)> {
	(0..count)
		.map(|_| {
			let angle_offset = rng.range(-spread..spread);
			let aim = Quat::from_axis_angle(Vec3::Z, angle_offset.to_radians()) * base_aim;
			let pos = origin + aim * offset;
			return (aim.xy(), pos);
		})
		.collect()
}

#[allow(dead_code)]
fn determine_spread_aim_and_pos(
	origin: Vec3,
	base_aim: Vec3,
	offset: f32,
	arc: f32,
	count: usize,
) -> Vec<(Vec2, Vec3)> {
	let interval = arc / count as f32;
	(0..count)
		.map(|i| {
			let angle_offset = (interval * i as f32) - arc / 2.;
			let aim = Quat::from_axis_angle(Vec3::Z, angle_offset.to_radians()) * base_aim;
			let pos = origin + aim * offset;
			return (aim.xy(), pos);
		})
		.collect()
}

fn fire_projectile(
	pos: Vec3,
	vel: Vec2,
	damage: f32,
	lifetime: f32,
	size: f32,
	member_group: Group,
	filter_group: Group,
	max_contact: u32,
	drag: f32,
	owner: Owner,
) -> ProjBundle {
	ProjBundle {
		proj: owner.into(),
		dmg: damage.into(),
		rigidbody: RigidBody::Dynamic,
		active_events: ActiveEvents::COLLISION_EVENTS,
		transform: Transform::from_translation(pos),
		vel: Velocity::linear(vel),
		collider: Collider::ball(size),
		groups: CollisionGroups::new(member_group, filter_group),
		contacts: ContactLimit(max_contact),
		life: lifetime.into(),
		drag: Damping {
			linear_damping: drag,
			..default()
		},
	}
}

fn fire_scatter_projectile(
	pos: Vec3,
	vel: Vec2,
	damage: f32,
	lifetime: f32,
	size: f32,
	member_group: Group,
	filter_group: Group,
	max_contact: u32,
	drag: f32,
	scatter: DeathScatter,
	owner: Owner,
) -> ScatterProj {
	ScatterProj(
		fire_projectile(
			pos,
			vel,
			damage,
			lifetime,
			size,
			member_group,
			filter_group,
			max_contact,
			drag,
			owner,
		),
		scatter,
	)
}

fn fire_sensor_projectile(
	pos: Vec3,
	vel: Vec2,
	damage: f32,
	lifetime: f32,
	size: f32,
	member_group: Group,
	filter_group: Group,
	max_contact: u32,
	drag: f32,
	owner: Owner,
) -> SensorProj {
	SensorProj(
		fire_projectile(
			pos,
			vel,
			damage,
			lifetime,
			size,
			member_group,
			filter_group,
			max_contact,
			drag,
			owner,
		),
		Sensor,
	)
}
