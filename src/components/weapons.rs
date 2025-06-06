use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[require(WeaponAuto, WeaponBeam, WeaponBurst, WeaponSpread, ProjectileType, WeaponFiring)]
pub enum Weapon {
	#[default]
	Auto,
	Spread,
	Burst,
	Beam,
}

#[derive(Component, Reflect, Default)]
pub struct WeaponFiring(pub bool);

impl WeaponFiring {
	pub fn is_not_firing(&self) -> bool {
		!self.0
	}
}

#[derive(Component, Reflect)]
pub struct WeaponAuto {
	pub damage_multi: f32,
	pub speed_multi: f32,
	pub fire_rate: Timer,
	//Degrees
	pub accuracy: f32,
}

impl Default for WeaponAuto {
	fn default() -> Self {
		Self {
			damage_multi: 0.7,
			speed_multi: 1.,
			fire_rate: Timer::from_seconds(1. / 5., TimerMode::Repeating),
			accuracy: 5.,
		}
	}
}

#[derive(Component, Reflect)]
pub struct WeaponSpread {
	pub damage_multi: f32,
	pub speed_multi: f32,
	pub shot_count: u32,
	pub accuracy: f32,
	pub arc: f32,
	pub fire_rate: Timer,
	pub recoil: f32,
}

impl Default for WeaponSpread {
	fn default() -> Self {
		Self {
			damage_multi: 0.5,
			speed_multi: 0.8,
			shot_count: 3,
			accuracy: 2.,
			arc: 40.,
			fire_rate: Timer::from_seconds(0.5, TimerMode::Repeating),
			recoil: 40.,
		}
	}
}

#[derive(Component, Reflect)]
pub struct WeaponBurst {
	pub damage_multi: f32,
	pub speed_multi: f32,
	pub fire_rate: Timer,
	pub accuracy: f32,
	pub burst: u32,
	pub burst_rate: Timer,
	pub cur_burst: u32,
}

impl Default for WeaponBurst {
	fn default() -> Self {
		Self {
			damage_multi: 0.5,
			speed_multi: 1.5,
			fire_rate: Timer::from_seconds(1., TimerMode::Repeating),
			accuracy: 3.,
			burst: 3,
			burst_rate: Timer::from_seconds(1. / 10., TimerMode::Repeating),
			cur_burst: 0,
		}
	}
}

#[derive(Component, Reflect)]
pub struct WeaponBeam {
	pub damage_multi: f32,
	pub hit_rate: Timer,
	pub max_range: f32,
}

impl Default for WeaponBeam {
	fn default() -> Self {
		Self {
			damage_multi: 0.7,
			hit_rate: Timer::from_seconds(1. / 5., TimerMode::Repeating),
			max_range: 300.,
		}
	}
}

#[derive(Component, Reflect, Clone, Copy)]
pub enum ProjectileType {
	Basic {
		damage: f32,
		speed: f32,
		multishot: u32,
	},
	Piercing {
		damage: f32,
		speed: f32,
		multishot: u32,
		penetration: u32,
	},
	Bouncing {
		damage: f32,
		speed: f32,
		multishot: u32,
		bounce_limit: u32,
	},
	Grenade {
		damage: f32,
		speed: f32,
		multishot: u32,
		bounce_limit: u32,
		fuse: f32,
		drag: f32,
		explosive_range: f32,
		explosive_speed: f32,
	},
}

impl Default for ProjectileType {
	fn default() -> Self {
		ProjectileType::Basic {
			damage: 40.,
			speed: 200.,
			multishot: 1,
		}
	}
}

impl ProjectileType {
	pub fn multishot(&self) -> u32 {
		*match self {
			ProjectileType::Basic { multishot, .. } => multishot,
			ProjectileType::Piercing { multishot, .. } => multishot,
			ProjectileType::Bouncing { multishot, .. } => multishot,
			ProjectileType::Grenade { multishot, .. } => multishot,
		}
	}
}
