use bevy::prelude::*;

use super::stats::{Health, MoveSpeed};

#[derive(Component, Reflect)]
#[require(Transform, MoveSpeed, Health, AITarget)]
pub struct AI {
	pub is_alive: bool,
}

impl Default for AI {
	fn default() -> Self {
		AI { is_alive: true }
	}
}
impl AI {
	pub fn is_dead(&self) -> bool {
		!self.is_alive
	}
}

#[derive(Component, Reflect, Default)]
pub struct AITarget(pub Vec3);

#[derive(Component, Reflect)]
#[require(AI)]
pub struct ChargeAI {
	pub charge_distance: f32,
	pub charge_speed: f32,
}

#[derive(Component, Reflect)]
#[require(AI)]
pub struct ChaseAI;

#[derive(Component, Reflect)]
#[require(AI)]
pub struct HoverAI {
	pub hover_distance: f32,
	pub range: f32,
}
