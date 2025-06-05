use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct Lifetime(pub Timer);

impl Lifetime {
	pub fn new(duration: f32) -> Self {
		Lifetime(Timer::from_seconds(duration, TimerMode::Once))
	}
}

impl Default for Lifetime {
	fn default() -> Self {
		Self(Timer::from_seconds(5., TimerMode::Once))
	}
}

impl From<f32> for Lifetime {
	fn from(value: f32) -> Self {
		Self(Timer::from_seconds(value, TimerMode::Once))
	}
}
