use bevy::prelude::*;

#[derive(Component, Reflect)]
#[require(Transform)]
pub struct HealthBar(pub Entity);

#[derive(Component, Reflect)]
#[require(Text2d)]
pub struct HealthBarText {
	pub health_entity: Entity,
	pub show_max: bool,
	pub display: HealthTextDisplayMode,
}

#[derive(Reflect)]
pub enum HealthTextDisplayMode {
	Raw,
	Percentage,
}
