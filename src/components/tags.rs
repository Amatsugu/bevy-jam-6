use bevy::prelude::*;

use super::stats::{Health, MaxHealth};

#[derive(Component, Default)]
#[require(MaxHealth, Health, Transform, Visibility)]
pub struct Enemy;

#[derive(Component, Reflect)]
pub struct PlayerOwned;

#[derive(Component, Reflect)]
pub struct EnemyOwned;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Reflect)]
pub struct KillOnContact;

#[derive(Component, Reflect)]
pub struct Projectile;
