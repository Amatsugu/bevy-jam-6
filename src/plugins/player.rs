use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

use crate::{
	PLAYER_GROUP,
	components::{
		stats::{Health, Life, MaxHealth, MoveSpeed, MoveSpeedStat},
		tags::MainCamera,
		ui::{HealthBar, HealthBarText, HealthTextDisplayMode},
		utils::Cleanable,
		weapons::{ProjectileType, Weapon, WeaponFiring},
	},
	resources::utils::{DefaultProjTypes, Fonts},
	state_management::{GameStartSystems, GameplayState, GameplaySystems},
};

pub struct PlayerPlugin;
#[derive(Component, Default, Reflect)]
#[require(MaxHealth(200.), MoveSpeedStat(100.), Transform, Visibility, Weapon, Cleanable)]
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
				damage: 40.,
				speed: 500.,
				multishot: 2,
				bounce_limit: 4,
			},
			grenade: ProjectileType::Grenade {
				damage: 100.,
				speed: 1000.,
				multishot: 1,
				bounce_limit: 5,
				fuse: 2.,
				drag: 3.4,
				explosive_range: 100.,
				explosive_speed: 300.,
			},
			piercing: ProjectileType::Piercing {
				damage: 20.,
				speed: 500.,
				multishot: 1,
				penetration: 5,
			},
		});

		app.add_systems(Update, spawn_player.in_set(GameStartSystems));
		app.add_systems(
			Update,
			(
				player_movement,
				look_at_mouse,
				fire_projectile,
				change_projectile,
				health_regen,
			)
				.in_set(GameplaySystems),
		);
		app.add_systems(PostUpdate, gameover_transition.in_set(GameplaySystems));
		// #[cfg(debug_assertions)]
		// app.add_systems(PostUpdate, infinite_health.in_set(GameplaySystems));
	}
}

fn gameover_transition(player: Single<&Life, With<Player>>, mut next: ResMut<NextState<GameplayState>>) {
	if player.is_dead() {
		next.set(GameplayState::GameOver);
		info!("Moving to GameOver");
	}
}

#[cfg(debug_assertions)]
#[allow(dead_code)]
fn infinite_health(player: Single<(&mut Health, &mut Life, &MaxHealth), With<Player>>) {
	let (mut health, mut life, max) = player.into_inner();
	health.0 = max.0;
	life.0 = true;
	life.1 = true;
}

fn health_regen(player: Single<(&mut Health, &Life, &MaxHealth), With<Player>>, time: Res<Time>) {
	let (mut health, life, max) = player.into_inner();
	if life.is_dead() {
		return;
	}
	health.0 += max.0 * 0.02 * time.delta_secs();
}

fn spawn_player(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	window: Single<&Window, With<PrimaryWindow>>,
	default_proj_types: Res<DefaultProjTypes>,
	fonts: Res<Fonts>,
) {
	let player = commands
		.spawn((
			Player,
			Weapon::Auto,
			default_proj_types.basic,
			RigidBody::Dynamic,
			ActiveEvents::COLLISION_EVENTS,
			Collider::ball(10.),
			Name::new("Player"),
			Mesh2d(meshes.add(Circle::new(10.))),
			Damping {
				linear_damping: 1.,
				..default()
			},
			MaxHealth(1000.),
			MeshMaterial2d(materials.add(Color::linear_rgb(1.0, 0.0, 0.39))),
			children![(
				Transform::from_translation(Vec3::Y * 7.),
				Mesh2d(meshes.add(Rectangle::new(5., 10.))),
				MeshMaterial2d(materials.add(Color::linear_rgb(1.0, 0.6, 0.16))),
			)],
			CollisionGroups::new(PLAYER_GROUP, Group::ALL),
		))
		.id();

	let size = window.size();
	const HEALTH_SIZE: Vec2 = Vec2::new(300., 20.);
	let pos = Vec3::new(0., (size.y / -2.) + 50., 10.);
	commands.spawn((
		Name::new("Health Bar"),
		Transform::from_translation(pos),
		HealthBar(player),
		Mesh2d(meshes.add(Rectangle::from_size(HEALTH_SIZE))),
		MeshMaterial2d(materials.add(Color::linear_rgb(0.0, 1.0, 0.0))),
		Cleanable,
	));
	commands.spawn((
		Name::new("Health Bar Text"),
		Transform::from_translation(pos + Vec3::Z),
		HealthBarText {
			display: HealthTextDisplayMode::Raw,
			show_max: true,
			health_entity: player,
		},
		TextFont {
			font: fonts.noto_regular.clone(),
			font_size: HEALTH_SIZE.y * 0.8,
			..default()
		},
		TextColor(LinearRgba::BLACK.into()),
		TextLayout::new_with_justify(JustifyText::Center),
		Cleanable,
	));
	commands.spawn((
		Name::new("Health Bar Fill"),
		Transform::from_translation(pos - Vec3::Z),
		Mesh2d(meshes.add(Rectangle::from_size(HEALTH_SIZE))),
		MeshMaterial2d(materials.add(Color::linear_rgb(1.0, 0.0, 0.0))),
		Cleanable,
	));
}

fn player_movement(
	player: Single<(&MoveSpeed, &Life, &mut ExternalForce), With<Player>>,
	key: Res<ButtonInput<KeyCode>>,
) {
	let (move_speed, life, mut force) = player.into_inner();
	if life.is_dead() {
		return;
	}
	let mut move_dir = Vec2::ZERO;

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

	force.force = move_dir.normalize_or_zero() * move_speed.0 * 500.;
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
