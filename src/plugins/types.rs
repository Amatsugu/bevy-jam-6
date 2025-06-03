#[cfg(debug_assertions)]
use bevy::prelude::*;

#[cfg(debug_assertions)]
use crate::components::{ai::*, death::*, spawner::*, stats::*, tags::*, utils::*};

#[cfg(debug_assertions)]
pub struct TypesPlugin;

#[cfg(debug_assertions)]
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
		app.register_type::<HoverAI>();
		app.register_type::<AITarget>();
		app.register_type::<DeathExplosion>();
		app.register_type::<DeathScatter>();
		app.register_type::<ScatterPattern>();
		app.register_type::<Targeting>();
		app.register_type::<Spawner>();
		app.register_type::<SpawnBatch>();
	}
}
