use bevy::{
    pbr::ExtendedMaterial,
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        storage::ShaderStorageBuffer,
    },
};

use super::{materials::ArrayTextureMaterial, resources::BlockTextureArray};

pub fn build_tex_array(
    mut commands: Commands,
    mut textures: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, ArrayTextureMaterial>>>,
    mut shader_buffers: ResMut<Assets<ShaderStorageBuffer>>,
) {
    // Create a single-color texture with multiple layers to form a texture array
    let color = [255, 255, 255, 255]; // RGBA color
    let extent = Extent3d {
        width: 2,
        height: 2,
        depth_or_array_layers: 6, // Set to 6 layers to match the expected array size
    };

    // Create data for each layer (you can customize this as needed)
    let mut texture_data =
        vec![0; (extent.width * extent.height * extent.depth_or_array_layers * 4) as usize];
    for layer in 0..extent.depth_or_array_layers {
        for y in 0..extent.height {
            for x in 0..extent.width {
                let index =
                    ((layer * extent.height * extent.width + y * extent.width + x) * 4) as usize;
                texture_data[index..index + 4].copy_from_slice(&color);
            }
        }
    }

    let array_texture = Image::new(
        extent,
        TextureDimension::D2, // Use D2 dimension
        texture_data,
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::default(),
    );

    // Add the texture array to the assets
    let texture_handle = textures.add(array_texture);

    // Create the material using the texture array
    let material_handle = materials.add(ExtendedMaterial {
        base: StandardMaterial {
            perceptual_roughness: 1.0,
            reflectance: 0.1,
            alpha_mode: AlphaMode::AlphaToCoverage,
            ..Default::default()
        },
        extension: ArrayTextureMaterial {
            array_texture: texture_handle,
            anim_offsets: shader_buffers.add(ShaderStorageBuffer::from(vec![1])),
        },
    });

    // Insert the material resource
    commands.insert_resource(BlockTextureArray(material_handle));
}
