use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct Lifetime(pub Timer);

impl Lifetime {
	pub fn new(duration: f32) -> Self {
		Lifetime(Timer::from_seconds(duration, TimerMode::Once))
	}
}
