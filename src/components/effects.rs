use bevy::prelude::*;

#[derive(Component, Reflect)]
#[require(ExplosionProgress, Transform)]
pub struct Explosion {
	pub range: f32,
	pub epansion_rate: f32,
}

#[derive(Component, Reflect, Default)]
pub struct ExplosionProgress(pub f32);
