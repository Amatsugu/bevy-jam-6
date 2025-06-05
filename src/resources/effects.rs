use bevy::prelude::*;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ExplosionMeshData {
	pub mesh: Handle<Mesh>,
	pub material: Handle<ColorMaterial>,
}
