use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{
	ai::{AI, AITarget, ChargeAI, ChaseAI, HoverAI},
	stats::{Life, MoveSpeed},
};

use super::player::Player;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, ((set_ai_chase_target, set_ai_hover_target), move_ai).chain());
		app.add_systems(Update, process_life);

		//Debugging
		#[cfg(feature = "ai")]
		#[cfg(debug_assertions)]
		app.add_systems(Update, (debug_ai, debug_hover_ai));
	}
}

fn move_ai(query: Query<(&mut Transform, &mut Velocity, &MoveSpeed, &AI, &AITarget, &Life)>) {
	for (mut transform, mut vel, speed, ai, tgt, life) in query {
		if ai.is_disabled() || life.is_dead() {
			continue;
		}
		let move_dir = (tgt.move_to - transform.translation.xy()).normalize_or_zero();
		if move_dir.length_squared() > f32::EPSILON {
			vel.linvel = move_dir * speed.0;
		}

		let look_dir = (tgt.look_at - transform.translation.xy()).normalize_or_zero();
		if look_dir.length_squared() > f32::EPSILON {
			transform.rotation = Quat::from_rotation_arc_2d(Vec2::Y, look_dir);
		}
	}
}

#[cfg(feature = "ai")]
#[cfg(debug_assertions)]
fn debug_ai(query: Query<(&Transform, &AITarget)>, mut gizmos: Gizmos) {
	for (transform, tgt) in query {
		gizmos.arrow_2d(transform.translation.xy(), tgt.move_to, Color::WHITE.with_alpha(0.2));
		gizmos.circle_2d(tgt.move_to.xy(), 1.0, Color::linear_rgb(1.0, 0.0, 0.0));
	}
}

#[cfg(feature = "ai")]
#[cfg(debug_assertions)]
fn debug_hover_ai(query: Query<&HoverAI>, mut gizmos: Gizmos, player: Single<&Transform, With<Player>>) {
	for hover in query {
		gizmos.circle_2d(
			player.translation.xy(),
			hover.min_distance(),
			Color::linear_rgba(0., 0., 1.0, 0.1),
		);
		gizmos.circle_2d(
			player.translation.xy(),
			hover.max_distance(),
			Color::linear_rgba(0., 1.0, 1.0, 0.1),
		);
	}
}

fn set_ai_chase_target(
	query: Query<(&mut AITarget, &AI, &Life), Or<(With<ChaseAI>, With<ChargeAI>)>>,
	player: Single<&Transform, With<Player>>,
) {
	for (mut tgt, ai, life) in query {
		if ai.is_disabled() || life.is_dead() {
			continue;
		}
		tgt.look_and_move(player.translation.xy());
	}
}

fn set_ai_hover_target(
	query: Query<(&mut AITarget, &AI, &Transform, &HoverAI)>,
	player: Single<&Transform, With<Player>>,
) {
	for (mut tgt, ai, transform, hover) in query {
		if ai.is_disabled() {
			continue;
		}
		let player_pos = player.translation.xy();
		let dir = (transform.translation.xy() - player_pos).xy();
		let dist = dir.length_squared();
		tgt.look_at = player_pos;
		if hover.is_in_range_squared(dist) {
			tgt.move_to = transform.translation.xy();
		} else {
			let dir_normalized = dir.normalize_or(Vec2::Y);
			tgt.move_to = player_pos + (dir_normalized * hover.hover_distance);
		}
	}
}

fn process_life(query: Query<(&mut AI, &Life)>) {
	for (mut ai, life) in query {
		if life.is_dead() {
			ai.enabled = false;
		}
	}
}
