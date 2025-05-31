use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
pub struct Damage(pub f32);

#[derive(Component, Default, Reflect)]
pub struct MoveSpeed(pub f32);

#[derive(Component, Default, Reflect)]
pub struct Health(pub f32);

#[derive(Component, Reflect)]
#[require(Health)]
pub struct MaxHealth(pub f32);

impl Default for MaxHealth {
	fn default() -> Self {
		return MaxHealth(100.);
	}
}

impl Into<Health> for MaxHealth {
	fn into(self) -> Health {
		Health(self.0)
	}
}

#[derive(Component, Reflect)]
pub struct FireRate(pub Timer);
impl Default for FireRate {
	fn default() -> Self {
		FireRate::new(30.)
	}
}
impl FireRate {
	pub fn new(rate: f32) -> Self {
		if rate <= 0. {
			return FireRate(Timer::from_seconds(0., TimerMode::Repeating));
		}
		return FireRate(Timer::from_seconds(1.0 / rate, TimerMode::Repeating));
	}
}
