use bevy::prelude::*;
use bevy_rapier2d::prelude::{RigidBody, Velocity};

use crate::components::tags::Enemy;

use super::player::Player;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, init);
	}
}

fn init(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
	let mesh = meshes.add(Triangle2d::new(
		Vec2::Y * 5.0,
		Vec2::new(-5.0, -5.0),
		Vec2::new(5.0, -5.0),
	));
	const GRID_SIZE: usize = 10;
	for x in 0..GRID_SIZE {
		for y in 0..GRID_SIZE {
			let color = Color::hsl(360.0 * (x as f32 / GRID_SIZE as f32), 1., 0.7);
			commands.spawn((
				Enemy,
				Name::new("Enemey"),
				Mesh2d(mesh.clone()),
				MeshMaterial2d(materials.add(color)),
				Transform::from_xyz(x as f32 * 10.0, y as f32 * 10.0, 0.0),
				RigidBody::KinematicVelocityBased,
				Velocity::zero(),
			));
		}
	}
}
