use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{
	ai::{AI, AITarget, ChargeAI, ChargeInfo, ChargeState, ChaseAI, HoverAI},
	stats::{Health, Life, MoveSpeed, MoveSpeedMultiplier},
};

use super::player::Player;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			PreUpdate,
			(set_ai_chase_target, set_ai_hover_target, set_ai_charge_target),
		);
		app.add_systems(Update, move_ai);
		app.add_systems(PostUpdate, (process_life, ai_charge_collision, ai_chase_collision));

		//Debugging
		#[cfg(feature = "ai")]
		#[cfg(debug_assertions)]
		app.add_systems(Update, (debug_ai, debug_hover_ai, debug_charge_ai));
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
		} else {
			vel.linvel = Vec2::ZERO;
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

#[cfg(feature = "ai")]
#[cfg(debug_assertions)]
fn debug_charge_ai(
	query: Query<(&ChargeAI, &ChargeInfo, &Transform)>,
	mut gizmos: Gizmos,
	player: Single<&Transform, With<Player>>,
) {
	for (charge, info, transform) in query {
		let color = match info.state {
			ChargeState::Chase => Color::linear_rgba(1., 0., 0., 0.1),
			ChargeState::Aim => Color::linear_rgba(0., 1., 0., 0.1),
			ChargeState::Charge => Color::linear_rgba(0., 0., 1., 0.1),
		};
		gizmos.circle_2d(transform.translation.xy(), charge.distance, color);
		match info.state {
			ChargeState::Aim => {
				gizmos.arrow_2d(
					transform.translation.xy(),
					player.translation.xy(),
					Color::linear_rgb(0.0, 1.0, 0.0),
				);
			}
			ChargeState::Charge => {
				gizmos.arrow_2d(
					transform.translation.xy(),
					transform.translation.xy() + transform.up().xy() * 20.,
					Color::linear_rgb(0.0, 0.0, 1.0),
				);
			}
			_ => (),
		}
	}
}

fn set_ai_chase_target(
	query: Query<(&mut AITarget, &AI, &Life), With<ChaseAI>>,
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
	query: Query<(&mut AITarget, &AI, &Transform, &HoverAI, &Life)>,
	player: Single<&Transform, With<Player>>,
) {
	for (mut tgt, ai, transform, hover, life) in query {
		if ai.is_disabled() || life.is_dead() {
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

fn set_ai_charge_target(
	query: Query<(
		&mut AITarget,
		&mut ChargeInfo,
		&mut MoveSpeedMultiplier,
		&mut Life,
		&AI,
		&Transform,
		&ChargeAI,
	)>,
	player: Single<&Transform, With<Player>>,
	time: Res<Time>,
) {
	for (mut tgt, mut info, mut move_multi, mut life, ai, transform, charge) in query {
		if ai.is_disabled() || life.is_dead() {
			continue;
		}

		match info.state {
			ChargeState::Chase => {
				let dist = Vec2::distance_squared(player.translation.xy(), transform.translation.xy());
				if dist <= (charge.distance * charge.distance) {
					info.state = ChargeState::Aim;
					info.charge.reset();
					tgt.move_to = transform.translation.xy();
				} else {
					tgt.look_and_move(player.translation.xy());
				}
			}
			ChargeState::Aim => {
				tgt.look_at = player.translation.xy();
				info.charge.tick(time.delta());
				if info.charge.finished() {
					info.state = ChargeState::Charge;
					info.cooldown.reset();
					info.charge_dir = (player.translation.xy() - transform.translation.xy()).normalize_or_zero();
					move_multi.0 = charge.speed_multi;
				}
			}
			ChargeState::Charge => {
				info.cooldown.tick(time.delta());
				tgt.look_and_move(transform.translation.xy() + info.charge_dir * 100.);
				if info.cooldown.finished() {
					life.0 = false;
				}
			}
		}
	}
}

fn ai_charge_collision(
	mut chargers: Query<(&ChargeInfo, &ChargeAI, &mut Life)>,
	mut other_entity: Query<&mut Health>,
	mut collisiion_events: EventReader<CollisionEvent>,
) {
	for event in collisiion_events.read() {
		if let CollisionEvent::Started(a, b, _) = event {
			if let Ok((info, charge, mut life)) = chargers.get_mut(*a) {
				process_collision(info, &mut life);
				if let Ok(mut health) = other_entity.get_mut(*b) {
					health.0 -= charge.hit_damage;
				}
			} else if let Ok((info, charge, mut life)) = chargers.get_mut(*b) {
				process_collision(info, &mut life);
				if let Ok(mut health) = other_entity.get_mut(*a) {
					health.0 -= charge.hit_damage;
				}
			}
		}
	}
	fn process_collision(info: &ChargeInfo, life: &mut Life) {
		if let ChargeState::Charge = info.state {
			life.0 = false;
		}
	}
}

fn ai_chase_collision(
	mut chasers: Query<&mut Life, With<ChaseAI>>,
	player: Query<(), With<Player>>,
	mut collisiion_events: EventReader<CollisionEvent>,
) {
	for event in collisiion_events.read() {
		if let CollisionEvent::Started(a, b, _) = event {
			if let Ok(mut life) = chasers.get_mut(*a) {
				if let Ok(()) = player.get(*b) {
					life.0 = false;
				}
			} else if let Ok(mut life) = chasers.get_mut(*b) {
				if let Ok(()) = player.get(*a) {
					life.0 = false;
				}
			}
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
