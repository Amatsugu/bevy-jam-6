use bevy::{ecs::entity_disabling::Disabled, prelude::*};
use bevy_rapier2d::prelude::{Collider, CollisionEvent, CollisionGroups, RigidBody, Velocity};

use crate::{
	PLAYER_GROUP,
	components::{
		stats::{Health, MaxHealth},
		tags::Pickup,
		utils::Lifetime,
		weapons::{Weapon, WeaponAuto, WeaponBurst, WeaponSpread},
	},
	plugins::player::Player,
	resources::utils::{DeathEvent, DefaultProjTypes, RandomGen},
	state_management::GameplaySystems,
};

pub struct DropsPlugin;

impl Plugin for DropsPlugin {
	fn build(&self, app: &mut App) {
		app.add_event::<PickupEvent>();
		app.add_systems(Startup, prepare_prefabs);
		app.add_systems(
			Update,
			(process_deaths, update_pickups, (pickup, pickup_events).chain()).in_set(GameplaySystems),
		);
	}
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct Prefabs {
	pub weapon_auto: Entity,
	pub weapon_burst: Entity,
	pub weapon_spread: Entity,
	pub health: Entity,
	pub stat: Entity,
}

#[derive(Event, Clone, Copy)]
struct PickupEvent(pub Pickup);

fn prepare_prefabs(mut commands: Commands, asset_server: Res<AssetServer>) {
	let health_sprite = asset_server.load("sprites/health.png");
	let weapon_auto_sprite = asset_server.load("sprites/auto.png");
	let weapon_spread_sprite = asset_server.load("sprites/spread.png");
	let weapon_burst_sprite = asset_server.load("sprites/burst.png");
	let stat_sprite = asset_server.load("sprites/upgrade.png");
	let weapon_auto = commands
		.spawn((
			Name::new("Pickup: Weapon-Auto"),
			Lifetime::new(30.),
			Disabled,
			Pickup::Weapon(Weapon::Auto),
			RigidBody::Dynamic,
			Collider::ball(10.),
			Sprite::from_image(weapon_auto_sprite),
			CollisionGroups::new(PLAYER_GROUP, PLAYER_GROUP),
		))
		.id();
	let weapon_burst = commands
		.spawn((
			Name::new("Pickup: Weapon-Burst"),
			Lifetime::new(30.),
			Disabled,
			Pickup::Weapon(Weapon::Burst),
			RigidBody::Dynamic,
			Collider::ball(10.),
			Sprite::from_image(weapon_burst_sprite),
			CollisionGroups::new(PLAYER_GROUP, PLAYER_GROUP),
		))
		.id();
	let weapon_spread = commands
		.spawn((
			Name::new("Pickup: Weapon-Spread"),
			Lifetime::new(10.),
			Disabled,
			Pickup::Weapon(Weapon::Spread),
			RigidBody::Dynamic,
			Collider::ball(10.),
			Sprite::from_image(weapon_spread_sprite),
			CollisionGroups::new(PLAYER_GROUP, PLAYER_GROUP),
		))
		.id();
	let health = commands
		.spawn((
			Name::new("Pickup: Weapon-Health"),
			Lifetime::new(30.),
			Disabled,
			Pickup::Health,
			RigidBody::Dynamic,
			Collider::ball(10.),
			Sprite::from_image(health_sprite),
			CollisionGroups::new(PLAYER_GROUP, PLAYER_GROUP),
		))
		.id();
	let stat = commands
		.spawn((
			Name::new("Pickup: Weapon-Stat"),
			Lifetime::new(30.),
			Disabled,
			Pickup::Stats,
			RigidBody::Dynamic,
			Collider::ball(10.),
			Sprite::from_image(stat_sprite),
			CollisionGroups::new(PLAYER_GROUP, PLAYER_GROUP),
		))
		.id();
	commands.insert_resource(Prefabs {
		weapon_auto,
		weapon_burst,
		weapon_spread,
		health,
		stat,
	});
}
const PICKUP_CHANCE: u32 = 10;
fn process_deaths(
	mut deaths: EventReader<DeathEvent>,
	prefabs: Res<Prefabs>,
	mut rng: ResMut<RandomGen>,
	mut commands: Commands,
) {
	for death in deaths.read() {
		if death.is_player {
			continue;
		}
		if rng.range(0..100) <= PICKUP_CHANCE {
			let pickup = rng.range(0..5);
			let mut entity_commands = match pickup {
				1 => commands.entity(prefabs.stat),
				2 => commands.entity(prefabs.weapon_auto),
				3 => commands.entity(prefabs.weapon_burst),
				4 => commands.entity(prefabs.weapon_spread),
				_ => commands.entity(prefabs.health),
			};
			entity_commands
				.clone_and_spawn_with(|b| {
					b.deny::<Disabled>();
				})
				.insert(Transform::from_translation(death.pos.extend(0.0)).with_scale(Vec3::splat(0.08)));
		}
	}
}

const PICKUP_RANGE: f32 = 200.;
const PICKUP_RANGE_SQ: f32 = PICKUP_RANGE * PICKUP_RANGE;

fn update_pickups(pickups: Query<(&mut Velocity, &Transform), With<Pickup>>, player: Single<&Transform, With<Player>>) {
	for (mut vel, transform) in pickups {
		let dir = player.translation.xy() - transform.translation.xy();
		if dir.length_squared() > PICKUP_RANGE_SQ {
			vel.linvel = Vec2::ZERO;
			continue;
		}
		vel.linvel = dir.normalize_or_zero() * 30.;
	}
}

fn pickup(
	mut collision_events: EventReader<CollisionEvent>,
	pickups: Query<(Entity, &Pickup)>,
	player: Single<Entity, With<Player>>,
	mut pickup_events: EventWriter<PickupEvent>,
	mut commands: Commands,
) {
	let player_entity = player.into_inner();
	for event in collision_events.read() {
		if let CollisionEvent::Started(a, b, _) = event {
			if let Ok((e, pickup)) = pickups.get(*a) {
				if *b == player_entity {
					commands.entity(e).despawn();
					pickup_events.write(PickupEvent(*pickup));
				}
			} else if let Ok((e, pickup)) = pickups.get(*b) {
				if *a == player_entity {
					commands.entity(e).despawn();
					pickup_events.write(PickupEvent(*pickup));
				}
			}
		}
	}
}

fn pickup_events(
	mut events: EventReader<PickupEvent>,
	player: Single<
		(
			&mut Health,
			&mut MaxHealth,
			&mut WeaponAuto,
			&mut WeaponBurst,
			&mut WeaponSpread,
		),
		With<Player>,
	>,
	mut projectiles: ResMut<DefaultProjTypes>,
) {
	let (mut health, mut max_health, mut auto, mut burst, mut spread) = player.into_inner();
	for event in events.read() {
		match event.0 {
			Pickup::Health => {
				health.0 += 10.;
				max_health.0 += 10.;
			}
			Pickup::Weapon(weapon) => match weapon {
				Weapon::Auto => auto.upgrade(0.1),
				Weapon::Spread => spread.upgrade(0.1),
				Weapon::Burst => burst.upgrade(0.1),
				Weapon::Beam => todo!(),
			},
			Pickup::Stats => {
				projectiles.upgrade(0.1);
			}
		};
	}
}
