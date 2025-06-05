use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

use crate::{
	PLAYER_GROUP,
	components::{
		stats::{MaxHealth, MoveSpeed, MoveSpeedStat},
		tags::MainCamera,
		weapons::{ProjectileType, Weapon, WeaponFiring},
	},
};

pub struct PlayerPlugin;
#[derive(Component, Default, Reflect)]
#[require(MaxHealth(200.), MoveSpeedStat(100.), Transform, Visibility, Weapon)]
pub struct Player;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, spawn_player);
		app.add_systems(Update, (player_movement, look_at_mouse, fire_projectile));
	}
}

fn spawn_player(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	commands.spawn((
		Player,
		Weapon::Auto,
		ProjectileType::Basic {
			damage: 100.,
			speed: 200.,
			multishot: 1,
		},
		RigidBody::KinematicPositionBased,
		ActiveEvents::COLLISION_EVENTS,
		Collider::ball(10.),
		Name::new("Player"),
		Mesh2d(meshes.add(Circle::new(10.))),
		MeshMaterial2d(materials.add(Color::linear_rgb(1.0, 0.0, 0.39))),
		children![(
			Transform::from_translation(Vec3::Y * 7.),
			Mesh2d(meshes.add(Rectangle::new(5., 10.))),
			MeshMaterial2d(materials.add(Color::linear_rgb(1.0, 0.6, 0.16))),
		)],
		CollisionGroups::new(PLAYER_GROUP, Group::ALL),
	));
}

fn player_movement(
	player: Single<(&mut Transform, &MoveSpeed), With<Player>>,
	key: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
) {
	let (mut transform, move_speed) = player.into_inner();
	let mut move_dir = Vec3::ZERO;

	if key.pressed(KeyCode::KeyW) {
		move_dir.y = 1.0;
	} else if key.pressed(KeyCode::KeyS) {
		move_dir.y = -1.0;
	}

	if key.pressed(KeyCode::KeyD) {
		move_dir.x = 1.0;
	} else if key.pressed(KeyCode::KeyA) {
		move_dir.x = -1.0;
	}

	transform.translation += move_dir.normalize_or_zero() * time.delta_secs() * move_speed.0;
}

fn look_at_mouse(
	mut player: Single<&mut Transform, With<Player>>,
	cam: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
	window: Single<&Window, With<PrimaryWindow>>,
) {
	let (cam, cam_transform) = cam.into_inner();
	if let Some(m_pos) = window.cursor_position() {
		if let Ok(world_pos) = cam.viewport_to_world_2d(cam_transform, m_pos) {
			let dir = (player.translation.xy() - world_pos).normalize();

			let rot = Quat::from_rotation_arc_2d(Vec2::NEG_Y, dir);
			player.rotation = rot;
		}
	}
}

fn fire_projectile(mut player_firing: Single<&mut WeaponFiring, With<Player>>, mouse: Res<ButtonInput<MouseButton>>) {
	player_firing.0 = mouse.pressed(MouseButton::Left);
}
