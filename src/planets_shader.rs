use crate::color::Color;
use crate::fragment::Fragment;
use crate::Uniforms;
use crate::shader::{depth_based_fragment_shader, noise_based_fragment_shader, moving_clouds_shader};
use nalgebra_glm::{Vec3};
use fastnoise_lite::{FastNoiseLite, NoiseType};


/// Primer shader de planeta: simula un planeta rocoso con textura granular
pub fn rocky_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Configuración de colores base
    let base_color = Color::new(153, 102, 51);      // Marrón
    let rock_color = Color::new(139, 69, 19);       // Marrón oscuro
    let mountain_color = Color::new(105, 105, 105); // Gris oscuro
    let atmosphere_color = Color::new(200, 180, 170); // Color tenue para atmósfera

    // Generador de ruido configurado como Perlin
    let mut noise = FastNoiseLite::new();
    noise.set_noise_type(Some(fastnoise_lite::NoiseType::Perlin)); // Envolvemos en `Some`

    // Obtener un valor FBM para el ruido con FastNoiseLite
    let fbm_value = fbm_with_fastnoise(
        fragment.vertex_position.x * 0.5,
        fragment.vertex_position.y * 0.5,
        &noise,
    );
    let normalized_noise = (fbm_value + 1.0) / 2.0;

    // Ajuste de color según el relieve usando `normalized_noise`
    let terrain_color = if normalized_noise < 0.4 {
        rock_color.blend_multiply(&base_color)
    } else if normalized_noise < 0.7 {
        base_color.blend_add(&mountain_color)
    } else {
        mountain_color
    };

    // Iluminación
    let light_dir = Vec3::new(1.0, -0.5, 1.0).normalize();
    let intensity = fragment.normal.dot(&light_dir).max(0.0);
    let shaded_color = terrain_color * (0.4 + 0.6 * intensity);

    // Efecto de atmósfera en el borde del planeta
    let distance_from_center = fragment.vertex_position.norm();
    let atmosphere_effect = ((1.0 - distance_from_center.clamp(0.8, 1.0)) / 0.2).clamp(0.0, 1.0);

    // Mezcla de color de atmósfera
    shaded_color.blend_screen(&atmosphere_color) * atmosphere_effect
}

// Función de ruido FBM usando FastNoiseLite para más rugosidad
fn fbm_with_fastnoise(x: f32, y: f32, noise: &FastNoiseLite) -> f32 {
    let mut total = 0.0;
    let mut frequency = 1.0;
    let mut amplitude = 0.5;

    for _ in 0..5 {
        total += noise.get_noise_2d(x * frequency, y * frequency) * amplitude; // Cambiado a `get_noise_2d`
        frequency *= 2.0;
        amplitude *= 0.5;
    }

    total
}


/// Segundo shader de planeta: simula un planeta gaseoso con nubes en movimiento
pub fn gaseous_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Color base para el planeta, más azulado para un aspecto gaseoso
    let base_color = Color::new(0, 0, 139); // Azul oscuro como fondo del planeta
    let cloud_color = moving_clouds_shader(fragment, uniforms); // Usa el shader de nubes en movimiento

    // Mezcla entre el color base y las nubes para un efecto gaseoso
    let blended_color = base_color.lerp(&cloud_color, 0.7);

    // Aplicar un ajuste de profundidad para mejorar el aspecto de las nubes y el fondo gaseoso
    depth_based_fragment_shader(fragment, blended_color)
}




pub fn frozen_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Colores base: azul claro para el hielo y blanco para la nieve
    let ice_color = Color::new(212, 246, 255); // Azul hielo (mayoría)
    let snow_color = Color::new(255, 255, 255); // Blanco nieve
    let crack_color = Color::new(198, 231, 255); // Gris azulado para grietas
    let fog_color = Color::new(0, 50, 5); // Color para la neblina (casi blanco con un toque de azul)

    // Configurar el ruido para crear textura de hielo
    let mut noise = FastNoiseLite::new();
    noise.set_noise_type(Some(NoiseType::Perlin));
    noise.set_frequency(Some(0.02));

    // Obtener valores de ruido para la textura de hielo y nieve
    let noise_value = noise.get_noise_2d(fragment.position.x, fragment.position.y);
    let normalized_noise = 0.1 * ((noise_value + 1.0) / 5.0);

    // Ajuste de color basado en la textura de hielo y nieve
    let surface_color = if normalized_noise < 0.02 {
        snow_color // Nieve en áreas dispersas
    } else if normalized_noise < 0.09 {
        crack_color // Grietas en el hielo
    } else {
        ice_color // Azul hielo como color principal
    };

    // Efecto de brillo y reflejo
    let light_dir = Vec3::new(1.0, 1.0, -1.0).normalize();
    let intensity = fragment.normal.dot(&light_dir).max(0.0);
    let reflective_color = surface_color * (0.7 + 0.3 * intensity); // Brillo ajustado según la luz

    // Configuración del tiempo para animar la neblina
    let time_factor = uniforms.time as f32 * 0.1; // Ajusta la velocidad de movimiento de la neblina

    // Obtener el valor de ruido para la neblina animada
    let fog_noise_value = uniforms.noise.get_noise_2d(
        fragment.position.x + time_factor,
        fragment.position.y + time_factor,
    );

    // Convertir el ruido en un rango de 0 a 1 para usarlo como valor de opacidad de la neblina
    let fog_movement = (fog_noise_value + 1.0) / 0.01;

    // Cálculo de la neblina basado en la distancia desde el centro del planeta
    let distance_from_center = fragment.vertex_position.norm();
    let fog_intensity = ((distance_from_center - 0.8) / 0.2).clamp(0.0, 1.0) * fog_movement; // Intensidad modulada por el ruido animado

    // Interpolar entre el color del fragmento y el color de la neblina en movimiento
    let final_color = reflective_color.lerp(&fog_color, fog_intensity);

    // Aplicar un ajuste de profundidad para el sombreado
    depth_based_fragment_shader(fragment, final_color)
}

