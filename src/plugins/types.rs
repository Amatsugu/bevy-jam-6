use bevy::prelude::*;

use crate::components::{stats::*, utils::*};

pub struct TypesPlugin;

impl Plugin for TypesPlugin {
	fn build(&self, app: &mut App) {
		app.register_type::<Health>();
		app.register_type::<Damage>();
		app.register_type::<FireRate>();
		app.register_type::<Lifetime>();
		app.register_type::<MoveSpeed>();
		app.register_type::<MaxHealth>();
	}
}
