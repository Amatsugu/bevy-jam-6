use bevy::prelude::*;

use super::stats::{Health, MoveSpeedMultiplier, MoveSpeedStat};

#[derive(Component, Reflect)]
#[require(Transform, MoveSpeedStat, MoveSpeedMultiplier, Health, AITarget)]
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
pub struct AITarget {
	pub move_to: Vec2,
	pub look_at: Vec2,
}

impl AITarget {
	pub fn look_and_move(&mut self, tgt: Vec2) {
		self.move_to = tgt;
		self.look_at = tgt;
	}
}

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

impl HoverAI {
	pub fn min_distance(&self) -> f32 {
		self.hover_distance - self.range
	}
	pub fn min_distance_squared(&self) -> f32 {
		let d = self.min_distance();
		return d * d;
	}

	pub fn max_distance(&self) -> f32 {
		self.hover_distance + self.range
	}

	pub fn max_distance_squared(&self) -> f32 {
		let d = self.max_distance();
		return d * d;
	}

	pub fn is_in_range_squared(&self, dist_squared: f32) -> bool {
		return self.min_distance_squared() <= dist_squared && self.max_distance_squared() >= dist_squared;
	}
}
