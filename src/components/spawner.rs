use bevy::prelude::*;

#[derive(Component, Reflect)]
#[require(Transform, SpawnBatch)]
pub struct Spawner {
	pub spawn_range: f32,
	//Time between spawn batches
	pub spawn_rate: Timer,
	//Time between spawning individual enemies
	pub spawn_speed: Timer,
	pub min_batch_size: usize,
	pub max_batch_size: usize,
	pub prefabs: Vec<Entity>,
	pub spawn_effect: Entity,
}

#[derive(Component, Default, Reflect)]
pub struct SpawnBatch(pub usize);
