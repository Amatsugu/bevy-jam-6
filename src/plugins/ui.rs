use bevy::prelude::*;

use crate::components::{
	stats::{Health, MaxHealth},
	ui::{HealthBar, HealthBarText, HealthTextDisplayMode},
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, (update_health_bars, update_health_text));
	}
}

fn update_health_bars(health_bars: Query<(&mut Transform, &HealthBar)>, healths: Query<(&Health, &MaxHealth)>) {
	for (mut transform, bar) in health_bars {
		if let Ok((health, max_health)) = healths.get(bar.0) {
			let scale = health.0 / max_health.0;
			transform.scale = Vec3::new(scale.max(0.), 1.0, 1.0);
		}
	}
}

fn update_health_text(texts: Query<(&mut Text2d, &HealthBarText)>, healths: Query<(&Health, &MaxHealth)>) {
	for (mut text, health_text) in texts {
		if let Ok((health, max)) = healths.get(health_text.health_entity) {
			match health_text.display {
				HealthTextDisplayMode::Raw => {
					if health_text.show_max {
						text.0 = format!("{}/{}", health.0, max.0);
					} else {
						text.0 = format!("{}", health.0);
					}
				}
				HealthTextDisplayMode::Percentage => {
					let p = (health.0 / max.0) * 100.;
					if health_text.show_max {
						text.0 = format!("{}%/100%", p.round());
					} else {
						text.0 = format!("{}%", p.round());
					}
				}
			}
		}
	}
}
