use bevy::prelude::*;

use super::stats::{Health, MoveSpeedMultiplier, MoveSpeedStat};

#[derive(Component, Reflect)]
#[require(Transform, MoveSpeedStat, MoveSpeedMultiplier, Health, AITarget)]
pub struct AI {
	pub enabled: bool,
}

impl Default for AI {
	fn default() -> Self {
		AI { enabled: true }
	}
}
impl AI {
	pub fn is_disabled(&self) -> bool {
		!self.enabled
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
#[require(AI, ChargeInfo, MoveSpeedMultiplier)]
pub struct ChargeAI {
	pub distance: f32,
	pub speed_multi: f32,
	pub hit_damage: f32,
}

#[derive(Component, Reflect)]
pub struct ChargeInfo {
	pub charge: Timer,
	pub cooldown: Timer,
	pub state: ChargeState,
	pub charge_dir: Vec2,
}

impl Default for ChargeInfo {
	fn default() -> Self {
		ChargeInfo {
			charge: Timer::from_seconds(1., TimerMode::Once),
			cooldown: Timer::from_seconds(1., TimerMode::Once),
			state: ChargeState::Chase,
			charge_dir: Vec2::ZERO,
		}
	}
}

#[derive(Default, Reflect)]
pub enum ChargeState {
	#[default]
	Chase,
	Aim,
	Charge,
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
