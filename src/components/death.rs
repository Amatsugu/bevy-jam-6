use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct DeathExplosion {
	pub range: f32,
	pub speed: f32,
	pub damage: f32,
}

#[derive(Component, Reflect)]
pub struct DeathScatter {
	pub count: u32,
	pub pattern: ScatterPattern,
	pub damage: f32,
}

#[derive(Reflect, Clone, Copy)]
pub enum ScatterPattern {
	Radial,
	Spread(f32, Targeting),
	Spiral(f32, f32),
}

#[derive(Reflect, Clone, Copy)]
pub enum Targeting {
	Forward,
	Random,
	Player,
	Closest,
}

#[derive(Component, Reflect, Default)]
pub struct SpiralSpawner {
	pub timer: Timer,
	pub count: u32,
	pub spawn_count: u32,
	pub arc: f32,
	pub damage: f32,
	pub mesh: Handle<Mesh>,
	pub material: Handle<ColorMaterial>,
}
