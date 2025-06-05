use bevy::prelude::*;

use crate::{
	components::{
		ai::*,
		death::*,
		effects::{Explosion, ExplosionProgress},
		spawner::*,
		stats::*,
		tags::*,
		utils::*,
		weapons::{ProjectileType, Weapon, WeaponAuto, WeaponBeam, WeaponBurst, WeaponFiring, WeaponSpread},
	},
	resources::effects::ExplosionMeshData,
};

pub struct TypesPlugin;

impl Plugin for TypesPlugin {
	fn build(&self, app: &mut App) {
		app.register_type::<Health>();
		app.register_type::<MaxHealth>();
		app.register_type::<Damage>();
		app.register_type::<Projectile>();
		app.register_type::<FireRate>();
		app.register_type::<Lifetime>();
		app.register_type::<Life>();
		app.register_type::<MoveSpeed>();
		app.register_type::<MoveSpeedStat>();
		app.register_type::<MoveSpeedMultiplier>();
		app.register_type::<AI>();
		app.register_type::<ChaseAI>();
		app.register_type::<ChargeAI>();
		app.register_type::<ChargeInfo>();
		app.register_type::<HoverAI>();
		app.register_type::<AITarget>();
		app.register_type::<DeathExplosion>();
		app.register_type::<DeathScatter>();
		app.register_type::<ScatterPattern>();
		app.register_type::<Targeting>();
		app.register_type::<Spawner>();
		app.register_type::<SpawnBatch>();
		app.register_type::<Explosion>();
		app.register_type::<ExplosionProgress>();
		app.register_type::<ExplosionMeshData>();
		app.register_type::<Weapon>();
		app.register_type::<WeaponFiring>();
		app.register_type::<WeaponAuto>();
		app.register_type::<WeaponBeam>();
		app.register_type::<WeaponBurst>();
		app.register_type::<WeaponSpread>();
		app.register_type::<ProjectileType>();
	}
}
