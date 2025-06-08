use bevy::prelude::*;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct AudioClips {
	pub explosion: Handle<AudioSource>,
	pub gameover: Handle<AudioSource>,
	pub spiral: Handle<AudioSource>,
	pub hit: Handle<AudioSource>,
	pub hurt: Handle<AudioSource>,
	pub shoot_auto: Handle<AudioSource>,
	pub shoot_burst: Handle<AudioSource>,
	pub shoot_spread: Handle<AudioSource>,
	pub start: Handle<AudioSource>,
	pub dash: Handle<AudioSource>,
	pub pickup: Handle<AudioSource>,
	pub heal: Handle<AudioSource>,
	pub weapon_switch: Handle<AudioSource>,
}
