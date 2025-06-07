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
	plugins::{player::Player, utils::play_audio_onshot},
	resources::{audio::AudioClips, utils::RandomGen},
	state_management::GameplaySet,
};

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, weapon_firing.in_set(GameplaySet));
	}
}

fn weapon_firing(
	query: Query<(
		&Transform,
		&mut Velocity,
		&WeaponFiring,
		&Life,
		&Weapon,
		&mut WeaponBeam,
		&mut WeaponAuto,
		&mut WeaponBurst,
		&mut WeaponSpread,
		&ProjectileType,
		Option<&Player>,
	)>,
	time: Res<Time>,
	mut commands: Commands,
	mut rng: ResMut<RandomGen>,
	audio: Res<AudioClips>,
) {
	for (transform, mut vel, firing, life, weapon, mut _beam, mut auto, mut burst, mut spread, proj, player) in query {
		if life.is_dead() || firing.is_not_firing() {
			continue;
		}
		let owner = if player.is_some() { Owner::Player } else { Owner::Enemy };
		let aim = transform.up().as_vec3();
		match weapon {
			Weapon::Auto => {
				auto.fire_rate.tick(time.delta());
				if auto.fire_rate.finished() {
					vel.linvel += transform.up().xy() * -auto.recoil;
					play_audio_onshot(&mut commands, audio.shoot_auto.clone());
					let volley = proj.multishot() * auto.fire_rate.times_finished_this_tick();
					prepare_auto_volley(volley, aim, transform.translation, &auto, proj, owner, &mut rng)
						.spawn(&mut commands);
				}
			}
			Weapon::Spread => {
				spread.fire_rate.tick(time.delta());
				if spread.fire_rate.finished() {
					vel.linvel += transform.up().xy() * -spread.recoil;
					play_audio_onshot(&mut commands, audio.shoot_spread.clone());
					let angle_offset = rng.range((-spread.accuracy)..spread.accuracy);
					let adjusted_aim = Quat::from_axis_angle(Vec3::Z, angle_offset.to_radians()) * aim;
					let volley = (proj.multishot() + spread.shot_count) * spread.fire_rate.times_finished_this_tick();
					prepare_spread_volley(volley, adjusted_aim, transform.translation, &spread, proj, owner)
						.spawn(&mut commands);
				}
			}
			Weapon::Burst => {
				if burst.cur_burst == 0 {
					burst.fire_rate.tick(time.delta());
					if burst.fire_rate.finished() {
						burst.cur_burst = (proj.multishot() + burst.burst) * burst.fire_rate.times_finished_this_tick();
					}
				} else {
					burst.burst_rate.tick(time.delta());
					if burst.burst_rate.finished() {
						vel.linvel += transform.up().xy() * -burst.recoil;
						play_audio_onshot(&mut commands, audio.shoot_burst.clone());
						let shots = burst.burst_rate.times_finished_this_tick().min(burst.cur_burst);
						burst.cur_burst -= shots;
						prepare_burst_volley(shots, aim, transform.translation, &burst, proj, owner, &mut rng)
							.spawn(&mut commands);
					}
				}
			}
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

impl ProjBatch {
	pub fn spawn(self, commands: &mut Commands) {
		match self {
			ProjBatch::Normal(proj_bundles) => commands.spawn_batch(proj_bundles),
			ProjBatch::Bounce(bounce_projs) => commands.spawn_batch(bounce_projs),
			ProjBatch::Sensor(sensor_projs) => commands.spawn_batch(sensor_projs),
			ProjBatch::Scatter(scatter_projs) => commands.spawn_batch(scatter_projs),
		}
	}
}

#[derive(Bundle, Default)]
struct SensorProj(ProjBundle, Sensor);

#[derive(Bundle)]
struct ScatterProj(ProjBundle, DeathScatter);

#[derive(Bundle)]
struct BounceProj(ProjBundle, Restitution);

enum ProjBatch {
	Normal(Vec<ProjBundle>),
	Bounce(Vec<BounceProj>),
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
const DEFAULT_MAX_CONTACT: u32 = 1;
const DEFAULT_DRAG: f32 = 0.0;

fn prepare_spread_volley(
	volley: u32,
	aim: Vec3,
	pos: Vec3,
	spread: &WeaponSpread,
	proj: &ProjectileType,
	owner: Owner,
) -> ProjBatch {
	let aim_pos = determine_spread_aim_and_pos(pos, aim, 10., spread.arc, volley);
	return create_projectile_batch(proj, owner, spread.speed_multi, spread.damage_multi, aim_pos);
}

fn prepare_auto_volley(
	volley: u32,
	aim: Vec3,
	pos: Vec3,
	auto: &WeaponAuto,
	proj: &ProjectileType,
	owner: Owner,
	rng: &mut RandomGen,
) -> ProjBatch {
	let aim_pos = determine_aim_and_pos(pos, aim, 10., auto.accuracy / 2., volley, rng);
	return create_projectile_batch(proj, owner, auto.speed_multi, auto.damage_multi, aim_pos);
}

fn prepare_burst_volley(
	volley: u32,
	aim: Vec3,
	pos: Vec3,
	burst: &WeaponBurst,
	proj: &ProjectileType,
	owner: Owner,
	rng: &mut RandomGen,
) -> ProjBatch {
	let aim_pos = determine_aim_and_pos(pos, aim, 10., burst.accuracy / 2., volley, rng);
	return create_projectile_batch(proj, owner, burst.speed_multi, burst.damage_multi, aim_pos);
}

fn create_projectile_batch(
	proj: &ProjectileType,
	owner: Owner,
	speed_multi: f32,
	damage_multi: f32,
	aim_pos: Vec<(Vec2, Vec3)>,
) -> ProjBatch {
	match proj {
		ProjectileType::Basic { damage, speed, .. } => {
			let bundles = aim_pos
				.iter()
				.map(|(aim, pos)| {
					fire_projectile(
						*pos,
						aim * speed * speed_multi,
						damage * damage_multi,
						PROJECTILE_LIFETIME,
						PROJECTILE_SIZE,
						PLAYER_PROJECTILE_GROUP,
						Group::ALL ^ PLAYER_OWNED_GROUP,
						DEFAULT_MAX_CONTACT,
						DEFAULT_DRAG,
						owner,
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
			let bundles = aim_pos
				.iter()
				.map(|(aim, pos)| {
					fire_sensor_projectile(
						*pos,
						aim * speed * speed_multi,
						damage * damage_multi,
						PROJECTILE_LIFETIME,
						PROJECTILE_SIZE,
						PLAYER_PROJECTILE_GROUP,
						ENEMY_OWNED_GROUP,
						*penetration,
						DEFAULT_DRAG,
						owner,
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
			let bundles = aim_pos
				.iter()
				.map(|(aim, pos)| {
					return BounceProj(
						fire_projectile(
							*pos,
							aim * speed * speed_multi,
							damage * damage_multi,
							PROJECTILE_LIFETIME,
							PROJECTILE_SIZE,
							PLAYER_PROJECTILE_GROUP,
							ENEMY_OWNED_GROUP,
							*bounce_limit,
							DEFAULT_DRAG,
							owner,
						),
						Restitution {
							coefficient: 1.0,
							combine_rule: CoefficientCombineRule::Max,
						},
					);
				})
				.collect();
			ProjBatch::Bounce(bundles)
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
			let bundles = aim_pos
				.iter()
				.map(|(aim, pos)| {
					fire_scatter_projectile(
						*pos,
						aim * speed * speed_multi,
						0.0,
						PROJECTILE_LIFETIME,
						PROJECTILE_SIZE,
						PLAYER_PROJECTILE_GROUP,
						ENEMY_OWNED_GROUP,
						*bounce_limit,
						*drag,
						DeathScatter {
							damage: damage * damage_multi,
							pattern: ScatterPattern::Explosion {
								range: *explosive_range,
								speed: *explosive_speed,
							},
							..default()
						},
						owner,
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

fn determine_spread_aim_and_pos(origin: Vec3, base_aim: Vec3, offset: f32, arc: f32, count: u32) -> Vec<(Vec2, Vec3)> {
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
