use bevy::prelude::*;
use bevy_inspector_egui::{InspectorOptions, prelude::ReflectInspectorOptions, quick::ResourceInspectorPlugin};
use bevy_rapier2d::{
	plugin::configuration,
	prelude::{RigidBody, Velocity},
};

use crate::components::tags::Enemy;

use super::player::Player;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(BoidsSettings {
			protected_range: 53.,
			max_range: MAX_RANGE,
			avoid_range: 12.5,
			avoid_factor: 2.0,
			align_range: 122.5,
			align_factor: 0.4,
			cohesion_range: 212.8,
			cohesion_factor: 0.7,
			max_speed: 50.,
			chase_strength: 0.001,
		});
		app.register_type::<BoidsSettings>();
		#[cfg(debug_assertions)]
		app.add_plugins(ResourceInspectorPlugin::<BoidsSettings>::default());
		app.add_systems(Startup, init)
			.add_systems(Update, (simulate_boids, align_with_velocity));
		#[cfg(debug_assertions)]
		app.add_systems(Update, boids_debug);
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
				Transform::from_xyz(x as f32 * 8.0, y as f32 * 8.0, 0.0),
				RigidBody::KinematicVelocityBased,
				Velocity::zero(),
			));
		}
	}
}
#[derive(Resource, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct BoidsSettings {
	pub protected_range: f32,
	pub max_range: f32,
	pub avoid_range: f32,
	pub avoid_factor: f32,
	pub align_range: f32,
	pub align_factor: f32,
	pub cohesion_range: f32,
	pub cohesion_factor: f32,
	pub max_speed: f32,
	#[inspector(min = 0.0, max = 1.0)]
	pub chase_strength: f32,
}

const MAX_RANGE: f32 = 500.;

fn simulate_boids(
	mut query: Query<(&Transform, &mut Velocity), With<Enemy>>,
	player: Single<&Transform, With<Player>>,
	settings: Res<BoidsSettings>,
	time: Res<Time>,
) {
	let mut combos = query.iter_combinations_mut();
	while let Some([mut a, mut b]) = combos.fetch_next() {
		let mut a_total_vel = a.1.linvel;
		let mut b_total_vel = b.1.linvel;
		//Align
		let dist = (a.0.translation - b.0.translation).xy();
		let d = dist.length();
		if d <= settings.align_range && d > settings.protected_range {
			let avg_vel = (a.1.linvel + b.1.linvel) / 2.;
			a_total_vel -= (avg_vel - a_total_vel) * settings.align_factor * time.delta_secs();
			b_total_vel -= (avg_vel - b_total_vel) * settings.align_factor * time.delta_secs();
		}

		//Collision Avoidance
		if d < settings.avoid_range {
			a_total_vel += dist * time.delta_secs() * settings.avoid_factor;
			b_total_vel += -dist * time.delta_secs() * settings.avoid_factor;
		}
		if d < settings.cohesion_range && d > settings.protected_range {
			//Choesian
			let avg_pos = (((b.0.translation - a.0.translation) / 2.) + a.0.translation).xy();
			a_total_vel += tend_to_point(a.0.translation.xy(), avg_pos) * settings.cohesion_factor * time.delta_secs();
			b_total_vel += tend_to_point(b.0.translation.xy(), avg_pos) * settings.cohesion_factor * time.delta_secs();
		}

		a.1.linvel = a_total_vel;
		b.1.linvel = b_total_vel;
	}

	for (t, mut v) in &mut query {
		//Tend to Player
		v.linvel +=
			tend_to_point(t.translation.xy(), player.translation.xy()) * time.elapsed_secs() * settings.chase_strength;
		//Limit Velocity
		if v.linvel == Vec2::ZERO {
			continue;
		}
		v.linvel = limit_velocity(v.linvel, settings.max_speed);
	}
}

fn align_with_velocity(mut query: Query<(&mut Transform, &Velocity), With<Enemy>>) {
	for (mut t, v) in &mut query.iter_mut() {
		t.rotation = Quat::from_rotation_arc_2d(Vec2::Y, v.linvel.normalize());
	}
}

#[cfg(debug_assertions)]
fn boids_debug(
	mut gizmos: Gizmos,
	query: Query<&Transform, With<Enemy>>,
	player: Single<&Transform, With<Player>>,
	settings: Res<BoidsSettings>,
) {
	const A: f32 = 0.01;
	for enemy in query.iter() {
		gizmos.circle_2d(
			enemy.translation.xy(),
			settings.protected_range,
			Color::linear_rgba(1., 0., 0., A),
		);
		gizmos.circle_2d(
			enemy.translation.xy(),
			settings.align_range,
			Color::linear_rgba(1., 1., 0., A),
		);
		gizmos.circle_2d(
			enemy.translation.xy(),
			settings.avoid_range,
			Color::linear_rgba(1., 0., 1., A),
		);
		gizmos.circle_2d(
			enemy.translation.xy(),
			settings.cohesion_range,
			Color::linear_rgba(0., 1., 1., A),
		);
	}
	gizmos.circle_2d(
		player.translation.xy(),
		settings.max_range,
		Color::linear_rgb(0., 0., 0.),
	);
}

fn tend_to_point(from: Vec2, to: Vec2) -> Vec2 {
	let diff = to - from;
	return diff.normalize();
}

fn limit_velocity(v: Vec2, max: f32) -> Vec2 {
	if v.length() > max {
		return v.normalize() * max;
	}
	return v;
}
