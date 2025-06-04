use std::f32::consts::TAU;

use bevy::prelude::*;
use rand::{
	Rng,
	distr::uniform::{SampleRange, SampleUniform},
};
use rand_chacha::ChaChaRng;

#[derive(Resource)]
pub struct RNG(pub ChaChaRng);

impl RNG {
	pub fn range<T, R>(&mut self, range: R) -> T
	where
		T: SampleUniform,
		R: SampleRange<T>,
	{
		self.0.random_range(range)
	}

	pub fn point_on_circle_vec3(&mut self, range: f32) -> Vec3 {
		let angle = self.0.random_range(0.0..TAU);
		let len = self.0.random_range(0.0..range);
		return Vec3::new(angle.cos(), angle.sin(), 0.0) * len;
	}

	#[allow(dead_code)]
	pub fn point_on_circle_vec2(&mut self, range: f32) -> Vec2 {
		let angle = self.0.random_range(0.0..TAU);
		let len = self.0.random_range(0.0..range);
		return Vec2::new(angle.cos(), angle.sin()) * len;
	}
}
