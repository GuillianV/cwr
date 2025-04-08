use bevy::prelude::*;
use std::f32::consts::PI;

/// Convertit un vecteur 2D en un angle en degrés.
///
/// # Arguments
///
/// * `vec` - Un `Vec2` représentant une direction dans l'espace 2D.
///
/// # Retourne
///
/// * Un `f32` représentant l'angle en degrés par rapport à l'axe des x positif.
pub fn vec2_to_degrees(vec: Vec2) -> f32 {
    // Calcule l'angle en radians entre le vecteur et l'axe des x positif.
    let angle_radians = vec.y.atan2(vec.x);

    // Convertit l'angle de radians en degrés.
    let angle_degrees = angle_radians * (180.0 / PI);

    angle_degrees
}

/// Combine une direction donnée par des valeurs normalisées avec une rotation précédente et retourne l'angle combiné en degrés.
///
/// # Arguments
///
/// * `direction_x` - La composante x de la direction normalisée entre -1 et 1.
/// * `direction_y` - La composante y de la direction normalisée entre -1 et 1.
/// * `previous_rotation_degrees` - La rotation précédente en degrés.
///
/// # Retourne
///
/// * Un `f32` représentant l'angle combiné en degrés.
#[allow(dead_code)]
fn combine_direction_with_rotation_to_degrees(
    direction_x: f32,
    direction_y: f32,
    previous_rotation_degrees: f32,
) -> f32 {
    // Convertit les valeurs normalisées en angles en degrés.
    // Ici, on suppose que -1 correspond à -90 degrés et 1 correspond à 90 degrés.
    let angle_x = direction_x * 90.0;
    let angle_y = direction_y * 90.0;

    // Calcule l'angle de la direction actuelle en radians en utilisant `atan2`.
    // `atan2` retourne l'angle en radians entre le vecteur (angle_x, angle_y) et l'axe des x positif.
    let direction_angle_radians = angle_y.atan2(angle_x);

    // Convertit l'angle de la direction actuelle de radians en degrés.
    let direction_angle_degrees = direction_angle_radians * (180.0 / PI);

    // Ajoute la rotation précédente à l'angle de la direction actuelle pour obtenir l'angle combiné.
    let combined_angle = direction_angle_degrees + previous_rotation_degrees;

    combined_angle
}

/// Combine une direction donnée par des valeurs normalisées avec une rotation précédente.
///
/// # Arguments
///
/// * `direction_x` - La composante x de la direction normalisée entre -1 et 1.
/// * `direction_y` - La composante y de la direction normalisée entre -1 et 1.
/// * `previous_rotation_degrees` - La rotation précédente en degrés.
///
/// # Retourne
///
/// * Un tuple `(f32, f32)` représentant les composantes x et y du vecteur combiné, normalisées entre -1 et 1.
pub fn combine_direction_with_rotation_to_eulers(
    direction_x: f32,
    direction_y: f32,
    previous_rotation_degrees: f32,
) -> (f32, f32) {
    // Convertit les valeurs normalisées en angles en degrés.
    // Ici, on suppose que -1 correspond à -90 degrés et 1 correspond à 90 degrés.
    let angle_x = direction_x * 90.0;
    let angle_y = direction_y * 90.0;

    // Calcule l'angle de la direction actuelle en radians.
    let direction_angle_radians = angle_y.atan2(angle_x);

    // Convertit l'angle de la direction actuelle en degrés.
    let direction_angle_degrees = direction_angle_radians * (180.0 / PI);

    // Ajoute la rotation précédente à l'angle de la direction actuelle.
    let combined_angle_degrees = direction_angle_degrees + previous_rotation_degrees;

    // Convertit l'angle combiné en radians.
    let combined_angle_radians = combined_angle_degrees * (PI / 180.0);

    // Calcule les composantes x et y du vecteur résultant.
    let combined_x = combined_angle_radians.cos();
    let combined_y = combined_angle_radians.sin();

    // Normalise les valeurs entre -1 et 1 pour s'assurer qu'elles restent dans les limites.
    (combined_x.clamp(-1.0, 1.0), combined_y.clamp(-1.0, 1.0))
}
