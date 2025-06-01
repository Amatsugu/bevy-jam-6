use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;
use bevy_rapier2d::prelude::RigidBody;

use crate::components::{
	ai::{AI, AITarget, ChaseAI},
	stats::MoveSpeed,
	tags::Enemy,
};

use super::player::Player;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, init);
		app.add_systems(Update, test);
		app.add_systems(Update, (set_ai_target, chase_ai).chain());
	}
}

fn test(query: Query<&Transform, With<AI>>) {
	for t in query {
		if t.scale == Vec3::ZERO || t.scale.x.is_nan() || t.scale.y.is_nan() {
			panic!("Zero Scale");
		}
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
				Name::new("Enemy"),
				Transform::from_translation(Vec3::new(x as f32 * 10.0, y as f32 * 10.0, 0.0)),
				RigidBody::KinematicPositionBased,
				Mesh2d(mesh.clone()),
				MeshMaterial2d(materials.add(color)),
				ChaseAI,
				MoveSpeed(5.),
				Collider::ball(8.),
			));
		}
	}
}

fn chase_ai(mut query: Query<(&mut Transform, &MoveSpeed, &AI, &AITarget), With<ChaseAI>>, time: Res<Time>) {
	for (mut transform, speed, ai, tgt) in &mut query {
		if ai.is_dead() {
			continue;
		}
		let dir = (tgt.0 - transform.translation).normalize_or_zero();
		if dir.length_squared() <= f32::EPSILON {
			continue;
		}
		transform.translation += dir * speed.0 * time.delta_secs();
		transform.rotation = Quat::from_rotation_arc_2d(Vec2::Y, dir.xy());
	}
}

fn set_ai_target(mut query: Query<(&AI, &mut AITarget)>, player: Single<&Transform, With<Player>>) {
	for (ai, mut tgt) in &mut query {
		if ai.is_dead() {
			continue;
		}
		tgt.0 = player.translation;
	}
}
