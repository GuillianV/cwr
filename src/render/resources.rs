
use bevy::{pbr::ExtendedMaterial, prelude::*};
use super::materials::ArrayTextureMaterial;

#[derive(Resource)]
pub struct BlockTextureArray(pub Handle<ExtendedMaterial<StandardMaterial, ArrayTextureMaterial>>);

