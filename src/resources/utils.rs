use std::f32::consts::TAU;

use bevy::prelude::*;
use rand::{
	Rng,
	distr::uniform::{SampleRange, SampleUniform},
};
use rand_chacha::ChaChaRng;

#[derive(Resource)]
pub struct RandomGen(pub ChaChaRng);

impl RandomGen {
	pub fn range<T, R>(&mut self, range: R) -> T
	where
		T: SampleUniform,
		R: SampleRange<T>,
	{
		self.0.random_range(range)
	}

	pub fn point_on_circle_vec3(&mut self, range: f32) -> Vec3 {
		return self.point_on_circle_vec2(range).extend(0.0);
	}

	pub fn point_on_circle_vec2(&mut self, range: f32) -> Vec2 {
		let len = self.0.random_range(0.0..range);
		return self.point_on_unit_circle() * len;
	}

	pub fn point_on_unit_circle(&mut self) -> Vec2 {
		let angle = self.0.random_range(0.0..TAU);
		return Vec2::new(angle.cos(), angle.sin());
	}
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct Fonts {
	pub noto: Handle<Font>,
	pub noto_regular: Handle<Font>,
	pub noto_thin: Handle<Font>,
}
