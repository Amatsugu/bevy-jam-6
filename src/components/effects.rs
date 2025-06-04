use bevy::prelude::*;
use bevy_rapier2d::prelude::Sensor;

#[derive(Component, Reflect)]
#[require(ExplosionProgress, Sensor, Transform)]
pub struct Explosion {
	pub range: f32,
	pub epansion_rate: f32,
}

#[derive(Component, Reflect, Default)]
pub struct ExplosionProgress(pub f32);
