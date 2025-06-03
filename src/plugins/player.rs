use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

use crate::{
	PLAYER_GROUP, PLAYER_OWNED_GROUP, PLAYER_PROJECTILE_GROUP,
	components::{
		stats::{Damage, FireRate, MaxHealth, MoveSpeed, MoveSpeedStat},
		tags::{MainCamera, Projectile},
		utils::Lifetime,
	},
};

pub struct PlayerPlugin;
#[derive(Component, Default, Reflect)]
#[require(MaxHealth(200.), MoveSpeedStat(100.), FireRate, Transform, Visibility)]
pub struct Player;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.register_type::<Projectiles>();
		app.add_systems(Startup, (spawn_player, init_meshes));
		app.add_systems(Update, (player_movement, look_at_mouse, fire_projectile));
	}
}

#[derive(Resource, Reflect, Default)]
struct Projectiles {
	mesh: Handle<Mesh>,
	mat: Handle<ColorMaterial>,
}

fn init_meshes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
	commands.insert_resource(Projectiles {
		mesh: meshes.add(Circle::new(2.)),
		mat: materials.add(Color::linear_rgb(1.0, 0.6, 0.16)),
	});
}

fn spawn_player(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	commands.spawn((
		Player,
		RigidBody::KinematicPositionBased,
		FireRate::new(5.),
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

fn fire_projectile(
	player: Single<(&Transform, &mut FireRate), With<Player>>,
	mouse: Res<ButtonInput<MouseButton>>,
	mut commands: Commands,
	projectiles: Res<Projectiles>,
	time: Res<Time>,
) {
	let (player_transform, mut firerate) = player.into_inner();
	if !firerate.0.finished() {
		firerate.0.tick(time.delta());
	}
	if firerate.0.finished() && mouse.pressed(MouseButton::Left) {
		firerate.0.tick(time.delta());
		commands.spawn((
			Projectile::player(),
			Damage(30.),
			Transform::from_translation(player_transform.translation + player_transform.up() * 20.),
			Lifetime::new(5.),
			RigidBody::Dynamic,
			Velocity::linear(player_transform.up().xy() * 200.),
			Mesh2d(projectiles.mesh.clone()),
			ActiveEvents::COLLISION_EVENTS,
			MeshMaterial2d(projectiles.mat.clone()),
			Collider::ball(2.),
			Sensor,
			CollisionGroups::new(PLAYER_PROJECTILE_GROUP, Group::ALL ^ PLAYER_OWNED_GROUP),
		));
	}
}
