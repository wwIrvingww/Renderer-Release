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
    // Colores base para las bandas de Júpiter
    let dark_brown = Color::new(139, 69, 19);      // Marrón oscuro
    let beige = Color::new(222, 184, 135);         // Beige claro
    let orange = Color::new(255, 140, 0);          // Naranja
    let white = Color::new(255, 255, 255);         // Blanco

    // Color para la Gran Mancha Roja
    let red_spot_color = Color::new(255, 69, 0);   // Rojo para la Gran Mancha
    let spot_highlight = Color::new(255, 160, 122); // Tono rosado para detalles

    // Parámetros de ruido para simular las bandas y la turbulencia
    let mut band_noise = FastNoiseLite::new();
    band_noise.set_noise_type(Some(NoiseType::Perlin));
    band_noise.set_frequency(Some(0.05));          // Frecuencia baja para bandas amplias

    let mut turbulence_noise = FastNoiseLite::new();
    turbulence_noise.set_noise_type(Some(NoiseType::Perlin));
    turbulence_noise.set_frequency(Some(0.2));     // Frecuencia más alta para detalles de turbulencia

    // Animación de bandas horizontales y rotación
    let time_factor = uniforms.time as f32 * 0.03; // Control de velocidad de rotación

    // Generar ruido para las bandas y turbulencia
    let band_value = band_noise.get_noise_2d(fragment.position.x, fragment.position.y + time_factor);
    let turbulence_value = turbulence_noise.get_noise_2d(fragment.position.x + time_factor, fragment.position.y);

    // Selección de color basado en el valor de ruido para bandas horizontales
    let band_color = if band_value < -0.2 {
        dark_brown
    } else if band_value < 0.1 {
        beige
    } else if band_value < 0.3 {
        orange
    } else {
        white
    };

    // Ajuste del color de la turbulencia mezclando ligeramente las bandas
    let turbulent_color = band_color.lerp(&beige, turbulence_value.abs() * 0.3);

    // Gran Mancha Roja - Agregada como una textura circular
    let spot_position = nalgebra_glm::vec2(0.3, -0.5); // Posición fija en coordenadas del planeta
    let fragment_position = nalgebra_glm::vec2(fragment.position.x, fragment.position.y);
    let spot_distance = nalgebra_glm::distance(&fragment_position, &spot_position);

    let red_spot_effect = if spot_distance < 0.1 {
        let spot_intensity = ((0.1 - spot_distance) / 0.1).clamp(0.0, 1.0); // Intensidad más alta en el centro
        red_spot_color.lerp(&spot_highlight, spot_intensity)
    } else {
        turbulent_color // Sin efecto fuera del área de la mancha
    };

    // Efecto de sombreado en los bordes para simular la curvatura
    let light_dir = Vec3::new(1.0, 1.0, -1.0).normalize();
    let intensity = fragment.normal.dot(&light_dir).max(0.0);
    let shaded_color = red_spot_effect * (0.7 + 0.3 * intensity); // Mezcla sombreada

    // Aplicar un ajuste de profundidad para una apariencia tridimensional
    depth_based_fragment_shader(fragment, shaded_color)
}

pub fn frozen_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Colores base: azul claro para el hielo y blanco para la nieve
    let ice_color = Color::new(19, 62, 135); // Azul hielo (mayoría) rgb() rgb()
    let snow_color = Color::new(255, 255, 255); // Blanco nieve
    let crack_color = Color::new(198, 231, 255); // Gris azulado para grietas
    let cloud_color = Color::new(247, 247, 248); // Usa el shader de nubes en movimiento


    // Configurar el ruido para crear textura de hielo
    let mut noise = FastNoiseLite::new();
    noise.set_noise_type(Some(NoiseType::Perlin));
    noise.set_frequency(Some(0.02));

    // Obtener valores de ruido para la textura de hielo y nieve
    let noise_value = noise.get_noise_2d(fragment.position.x, fragment.position.y);
    let normalized_noise = 0.1 * ((noise_value + 1.0) / 5.0);

    //Fog
    let time_factor = uniforms.time as f32 * 2.1; // Ajusta la velocidad de movimiento de las nubes
    let noiseFog_value = 0.5 * uniforms.noise.get_noise_2d(fragment.position.x + time_factor, fragment.position.y);
    let fog_opacity = (noiseFog_value + 1.0) / 1.0;


    // Ajuste de color basado en la textura de hielo y nieve (inversión de colores)
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
    let reflective_color = surface_color * (0.7 + 0.3 * intensity); // Ajuste de brillo reducido

    let blended_color = reflective_color.lerp(&cloud_color, fog_opacity);
    // Aplicar un ajuste de profundidad para el sombreado
    depth_based_fragment_shader(fragment, blended_color)
}
