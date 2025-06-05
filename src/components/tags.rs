use bevy::prelude::*;

use super::stats::{Health, MaxHealth};

#[derive(Component, Default)]
#[require(MaxHealth, Health, Transform, Visibility)]
pub struct Enemy;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Reflect)]
pub struct KillOnContact;

#[derive(Component, Reflect)]
pub struct ContactLimit(pub u32);

impl Default for ContactLimit {
	fn default() -> Self {
		ContactLimit(1)
	}
}

#[derive(Component, Reflect, Default)]
#[require(ContactLimit)]
pub struct Projectile(pub Owner);

impl Projectile {
	#[allow(dead_code)]
	pub fn player() -> Self {
		Projectile(Owner::Player)
	}
	#[allow(dead_code)]
	pub fn enemy() -> Self {
		Projectile(Owner::Enemy)
	}
}

impl From<Owner> for Projectile {
	fn from(value: Owner) -> Self {
		Projectile(value)
	}
}

#[derive(Reflect, Default, Clone, Copy)]
pub enum Owner {
	#[default]
	Player,
	Enemy,
}
