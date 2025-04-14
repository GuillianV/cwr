use bevy::{pbr::ExtendedMaterial, prelude::*};
use materials::ArrayTextureMaterial;

pub mod constants;
pub mod materials;
pub mod resources;
pub mod systems;

pub struct ChunkRenderPlugin;

impl Plugin for ChunkRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, ArrayTextureMaterial>,
        >::default())
            .add_systems(Startup, systems::build_tex_array);
    }
}
