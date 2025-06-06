use bevy::prelude::*;

use crate::components::stats::Life;

#[derive(Component, Reflect)]
pub struct DeathExplosion {
	pub range: f32,
	pub speed: f32,
	pub damage: f32,
}

#[derive(Component, Reflect, Default, Clone, Copy)]
#[require(Life, Transform)]
pub struct DeathScatter {
	pub count: u32,
	pub pattern: ScatterPattern,
	pub damage: f32,
}

#[derive(Reflect, Clone, Copy)]
pub enum ScatterPattern {
	Explosion { range: f32, speed: f32 },
	Spread { arc: f32, targeting: Targeting },
	Spiral { angle: f32, rate: f32 },
}
impl Default for ScatterPattern {
	fn default() -> Self {
		Self::Explosion {
			range: 100.,
			speed: 100.,
		}
	}
}

#[derive(Reflect, Clone, Copy)]
pub enum Targeting {
	Forward,
	Random,
	Player,
}

#[derive(Component, Reflect, Default)]
pub struct SpiralSpawner {
	pub timer: Timer,
	pub count: u32,
	pub spawn_count: u32,
	pub angle: f32,
	pub damage: f32,
	pub mesh: Handle<Mesh>,
	pub material: Handle<ColorMaterial>,
}
