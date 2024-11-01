use crate::color::Color;
use crate::fragment::Fragment;
use crate::Uniforms;
use crate::shader::{depth_based_fragment_shader, noise_based_fragment_shader, moving_clouds_shader, ocean_currents_shader};
use nalgebra_glm::{Vec3};
use fastnoise_lite::{FastNoiseLite, NoiseType};


/// Primer shader de planeta: simula un planeta rocoso con textura granular
pub fn rocky_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Colores base para simular una superficie árida y terrosa
    let sand_color = Color::new(210, 180, 140);    // Color de arena ocre 210, 180, 140
    let rock_color = Color::new(139, 115, 85);     // Color de roca marrón oscuro
    let cracked_sand_color = Color::new(148, 129, 104); // Arena agrietada beige 220, 200, 160 

    // Configuración de ruido para simular dunas de arena y formaciones rocosas
    let mut dune_noise = FastNoiseLite::new();
    dune_noise.set_noise_type(Some(NoiseType::OpenSimplex2));  // Patrón fluido para dunas
    dune_noise.set_frequency(Some(0.02));                      // Frecuencia baja para dunas grandes 0.02

    let mut rock_noise = FastNoiseLite::new();
    rock_noise.set_noise_type(Some(NoiseType::Cellular));      // Celular para rocas y grietas
    rock_noise.set_frequency(Some(0.1));                       // Frecuencia media para detalles en roca

    // Animación del polvo en el viento
    let time_factor = uniforms.time as f32 * 0.5;            // Control de velocidad de movimiento del polvo
    let dust_movement = dune_noise.get_noise_2d(
        fragment.position.x + time_factor,
        fragment.position.y + time_factor,
    );

    // Obtener el valor de ruido para las dunas y las formaciones rocosas
    let dune_value = dune_noise.get_noise_2d(fragment.position.x, fragment.position.y);
    let rock_value = rock_noise.get_noise_2d(fragment.position.x, fragment.position.y);

    // Asignación de color en función del valor de ruido
    let surface_color = if dune_value > 0.3 {
        sand_color.lerp(&rock_color, rock_value.abs())         // Mezcla entre arena y roca
    } else {
        sand_color.lerp(&cracked_sand_color, rock_value.abs()) // Grietas en la arena
    };

    // Efecto de desplazamiento suave para simular polvo en movimiento
    let dusty_color = surface_color.lerp(&sand_color, dust_movement.abs() * 0.2);

    // Iluminación intensa y sombras para resaltar el relieve
    let light_dir = Vec3::new(1.0, -1.0, 0.5).normalize();
    let intensity = fragment.normal.dot(&light_dir).max(0.0);
    let illuminated_color = dusty_color * (0.6 + 0.4 * intensity);

    // Efecto de niebla atmosférica en el horizonte
    let distance_from_center = fragment.vertex_position.norm();
    let fog_intensity = ((distance_from_center - 0.7) / 0.3).clamp(0.0, 1.0);
    let fog_color = Color::new(255, 245, 230); // Color de la niebla
    let final_color = illuminated_color.lerp(&fog_color, fog_intensity);

    // Aplicar sombreado basado en profundidad para simular la curvatura del planeta
    depth_based_fragment_shader(fragment, final_color)
}

/// Segundo shader de planeta: simula un planeta como Jupiter
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

/// Tercer shader de planeta: simula un planeta congelado
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

/// Cuarto shader de planeta: simula el planeta Tierra
pub fn earth_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Colores base para continentes
    let land_color = Color::new(34, 139, 34);          // Verde para continentes
    let mountain_color = Color::new(139, 69, 19);      // Marrón para montañas y elevaciones

    // Configurar ruido para texturas de continentes
    let mut terrain_noise = FastNoiseLite::new();
    terrain_noise.set_noise_type(Some(NoiseType::OpenSimplex2S));
    terrain_noise.set_frequency(Some(0.01));            // Frecuencia para detalles del terreno

    // Generar valores de ruido para el terreno y el océano
    let terrain_value = terrain_noise.get_noise_2d(fragment.position.x, fragment.position.y);

    // Asignación de colores en función del ruido para el océano y los continentes
    let base_terrain_color = if terrain_value < -0.3 {
        ocean_currents_shader(fragment, uniforms) // Llamada al shader de corrientes oceánicas
    } else if terrain_value < 0.1 {
        land_color.lerp(&mountain_color, terrain_value.abs() * 0.4)    // Textura de relieve en tierra
    } else {
        mountain_color
    };

    // Nubes en movimiento usando ruido desplazado
    let mut cloud_noise = FastNoiseLite::new();
    cloud_noise.set_noise_type(Some(NoiseType::Perlin));
    cloud_noise.set_frequency(Some(0.02));

    // Animación de las nubes con desplazamiento por tiempo
    let cloud_time_factor = uniforms.time as f32 * 1.1;
    let cloud_value = cloud_noise.get_noise_2d(fragment.position.x + cloud_time_factor, fragment.position.y);
    let cloud_opacity = ((cloud_value + 1.0) / 2.0).clamp(0.0, 1.0);

    let cloud_color = Color::new(255, 255, 255);
    let cloud_layer_color = base_terrain_color.lerp(&cloud_color, cloud_opacity * 0.5);

    // Efecto atmosférico
    let distance_from_center = fragment.vertex_position.norm();
    let atmosphere_intensity = ((distance_from_center - 0.8) / 0.3).clamp(0.0, 1.0);
    let atmosphere_color = Color::new(135, 206, 250);

    // Interpolación final con la atmósfera
    let final_color = cloud_layer_color.lerp(&atmosphere_color, atmosphere_intensity);

    // Sombreado final
    depth_based_fragment_shader(fragment, final_color)
}

/// Quinto shader de planeta: simula un planeta de agua 
pub fn oceanic_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Colores base para el océano
    let deep_ocean_color = Color::new(0, 64, 128);     // Azul profundo
    let shallow_ocean_color = Color::new(0, 105, 148); // Azul claro para áreas menos profundas
    let highlight_color = Color::new(255, 255, 255);   // Blanco para reflejos de luz

    // Configuración de ruido para simular ondas y patrones de flujo
    let mut wave_noise = FastNoiseLite::new();
    wave_noise.set_noise_type(Some(NoiseType::Perlin));
    wave_noise.set_frequency(Some(0.1));               // Frecuencia media para variación de ondas

    let mut large_wave_noise = FastNoiseLite::new();
    large_wave_noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    large_wave_noise.set_frequency(Some(0.02));        // Frecuencia baja para ondas amplias

    let mut swirl_noise = FastNoiseLite::new();
    swirl_noise.set_noise_type(Some(NoiseType::Cellular));
    swirl_noise.set_frequency(Some(0.05));             // Frecuencia para remolinos

    // Movimiento animado de las ondas con desplazamiento por tiempo
    let time_factor = uniforms.time as f32 * 0.1;
    let wave_value = wave_noise.get_noise_2d(fragment.position.x + time_factor, fragment.position.y);
    let large_wave_value = large_wave_noise.get_noise_2d(fragment.position.x, fragment.position.y + time_factor);
    let swirl_value = swirl_noise.get_noise_2d(fragment.position.x + time_factor * 0.5, fragment.position.y);

    // Combinar valores de ruido para ondas dinámicas y patrones de flujo
    let ocean_wave_color = deep_ocean_color.lerp(&shallow_ocean_color, (wave_value * 0.3 + large_wave_value * 0.7).abs());
    let ocean_with_swirl = ocean_wave_color.lerp(&highlight_color, swirl_value.abs() * 0.2);

    // Reflejos suaves y destellos de luz en la superficie
    let light_dir = Vec3::new(1.0, 1.0, -0.5).normalize();
    let view_dir = fragment.normal.normalize();
    let reflect_dir = 2.0 * fragment.normal.dot(&light_dir) * fragment.normal - light_dir;
    let spec_intensity = reflect_dir.dot(&view_dir).max(0.0).powf(50.0); // Ajuste de intensidad especular

    let specular_highlight = ocean_with_swirl.blend_add(&highlight_color) * spec_intensity;

    // Ajustar la transparencia para áreas menos profundas
    let depth_factor = ((fragment.position.norm() + 1.0) / 2.0).clamp(0.0, 1.0);
    let refracted_color = ocean_with_swirl.lerp(&highlight_color, depth_factor * 0.15);

    // Combinación final con reflejos especulares y efectos de transparencia
    let final_color = refracted_color.blend_overlay(&specular_highlight);

    // Aplicar sombreado basado en profundidad para mayor realismo en la superficie
    depth_based_fragment_shader(fragment, final_color)
}


/// Sexto shader: UFO
pub fn ufo_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Colores base para el metal y el aura
    let metallic_color = Color::new(192, 192, 192);      // Plateado metálico
    let highlight_color = Color::new(255, 255, 255);     // Blanco para reflejos en el metal
    let aura_color = Color::new(200, 200, 255);          // Azul tenue para el aura

    // Textura metálica usando ruido para simular variación de superficie
    let mut metal_noise = FastNoiseLite::new();
    metal_noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    metal_noise.set_frequency(Some(0.005));               // Frecuencia para textura del metal

    let reflection_value = (metal_noise.get_noise_2d(fragment.position.x, fragment.position.y) * 1.5).clamp(-1.0, 0.0); 
    let metallic_surface = metallic_color.lerp(&highlight_color, reflection_value.abs() * 0.5); // Ajuste de intensidad del reflejo

    // Efecto de aura alrededor del objeto
    let distance_from_center = fragment.vertex_position.norm();
    let aura_intensity = (0.6 - distance_from_center).clamp(0.0, 1.0) * 0.5; // Intensidad del aura suave
    let aura_effect = metallic_surface.lerp(&aura_color, aura_intensity);

    // Color final con textura metálica y aura
    depth_based_fragment_shader(fragment, aura_effect)
}


