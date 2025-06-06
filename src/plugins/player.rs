use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

use crate::{
	PLAYER_GROUP,
	components::{
		stats::{Health, Life, MaxHealth, MoveSpeed, MoveSpeedStat},
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
		app.insert_resource(DefaultProjTypes {
			basic: ProjectileType::Basic {
				damage: 50.,
				speed: 500.,
				multishot: 1,
			},
			bouncing: ProjectileType::Bouncing {
				damage: 50.,
				speed: 500.,
				multishot: 1,
				bounce_limit: 4,
			},
			grenade: ProjectileType::Grenade {
				damage: 50.,
				speed: 1000.,
				multishot: 1,
				bounce_limit: 5,
				fuse: 2.,
				drag: 3.4,
				explosive_range: 100.,
				explosive_speed: 100.,
			},
			piercing: ProjectileType::Piercing {
				damage: 50.,
				speed: 500.,
				multishot: 1,
				penetration: 5,
			},
		});

		app.add_systems(Startup, spawn_player);
		app.add_systems(
			Update,
			(player_movement, look_at_mouse, fire_projectile, change_projectile),
		);
		app.add_systems(PostUpdate, infinite_health);
	}
}

#[allow(dead_code)]
fn infinite_health(player: Single<(&mut Health, &mut Life, &MaxHealth), With<Player>>) {
	let (mut health, mut life, max) = player.into_inner();
	health.0 = max.0;
	life.0 = true;
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct DefaultProjTypes {
	basic: ProjectileType,
	piercing: ProjectileType,
	bouncing: ProjectileType,
	grenade: ProjectileType,
}

fn spawn_player(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	default_proj_types: Res<DefaultProjTypes>,
) {
	commands.spawn((
		Player,
		Weapon::Auto,
		default_proj_types.basic,
		RigidBody::KinematicPositionBased,
		ActiveEvents::COLLISION_EVENTS,
		Collider::ball(10.),
		Name::new("Player"),
		Mesh2d(meshes.add(Circle::new(10.))),
		MaxHealth(1000.),
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
	player: Single<(&mut Transform, &MoveSpeed, &Life), With<Player>>,
	key: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
) {
	let (mut transform, move_speed, life) = player.into_inner();
	if life.is_dead() {
		return;
	}
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

fn change_projectile(
	player: Single<&mut ProjectileType, With<Player>>,
	key: Res<ButtonInput<KeyCode>>,
	default_proj_types: Res<DefaultProjTypes>,
) {
	let mut proj_type = player.into_inner();
	if key.just_pressed(KeyCode::Digit1) {
		*proj_type = default_proj_types.basic;
	} else if key.just_pressed(KeyCode::Digit2) {
		*proj_type = default_proj_types.piercing;
	} else if key.just_pressed(KeyCode::Digit3) {
		*proj_type = default_proj_types.bouncing;
	} else if key.just_pressed(KeyCode::Digit4) {
		*proj_type = default_proj_types.grenade;
	}
}
