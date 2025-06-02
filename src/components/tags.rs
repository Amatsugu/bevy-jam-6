use bevy::prelude::*;

use super::stats::{Health, MaxHealth};

#[derive(Component, Default)]
#[require(MaxHealth, Health, Transform, Visibility)]
pub struct Enemy;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Reflect)]
pub struct KillOnContact;

#[derive(Component, Reflect, Default)]
pub struct Projectile(pub Owner);

impl Projectile {
	pub fn player() -> Self {
		Projectile(Owner::Player)
	}
	pub fn enemy() -> Self {
		Projectile(Owner::Enemy)
	}
}

#[derive(Reflect, Default, Clone, Copy)]
pub enum Owner {
	#[default]
	Player,
	Enemy,
}
