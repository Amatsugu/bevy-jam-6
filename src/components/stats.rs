use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Default, Reflect)]
pub struct Damage(pub f32);

impl From<f32> for Damage {
	fn from(value: f32) -> Self {
		Self(value)
	}
}

#[derive(Component, Default, Reflect)]
#[require(RigidBody, Velocity, ExternalForce, Damping)]
pub struct MoveSpeed(pub f32);

#[derive(Component, Default, Reflect)]
#[require(MoveSpeed)]
pub struct MoveSpeedStat(pub f32);

#[derive(Component, Reflect)]
#[require(MoveSpeedStat)]
pub struct MoveSpeedMultiplier(pub f32);

impl Default for MoveSpeedMultiplier {
	fn default() -> Self {
		MoveSpeedMultiplier(1.)
	}
}

#[derive(Component, Reflect)]
#[require(Life)]
pub struct Health(pub f32);

impl Default for Health {
	fn default() -> Self {
		Health(100.)
	}
}

#[derive(Component, Reflect)]
pub struct Life(pub bool, pub bool);

impl Default for Life {
	fn default() -> Self {
		Life(true, false)
	}
}

impl Life {
	pub fn is_alive(&self) -> bool {
		self.0
	}
	pub fn is_dead(&self) -> bool {
		!self.0
	}
}

#[derive(Component, Reflect)]
#[require(Health)]
pub struct MaxHealth(pub f32);

#[derive(Component, Reflect)]
#[require(Health, MaxHealth)]
pub struct HealthRegen(pub f32);

impl Default for MaxHealth {
	fn default() -> Self {
		return MaxHealth(100.);
	}
}

impl From<MaxHealth> for Health {
	fn from(val: MaxHealth) -> Self {
		Health(val.0)
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
