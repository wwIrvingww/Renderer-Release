use nalgebra_glm::{Vec3, Vec4, Vec2, vec2};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;
use rand::Rng;
use std::f32::consts::PI;
use fastnoise_lite::{FastNoiseLite, NoiseType, FractalType};

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    // Aplicar la matriz de transformación completa (precomputada)
    let transformed = uniforms.transformation_matrix * Vec4::new(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);
    
    let w = transformed.w;
    let ndc_position = Vec4::new(
        transformed.x / w,
        transformed.y / w,
        transformed.z / w,
        1.0
    );

    let transformed_position = uniforms.viewport_matrix * ndc_position;

    // Crear un nuevo vértice con los atributos transformados
    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position: Vec3::new(transformed_position.x, transformed_position.y, transformed_position.z),
        transformed_normal: vertex.normal,
    }
}

// Retorna un Color en lugar de un Fragment
pub fn fragment_shader(_fragment: &Fragment) -> Color {
    Color::new(255, 0, 0)
}

// Modificado para retornar un Color
pub fn depth_based_fragment_shader(fragment: &Fragment, base_color: Color) -> Color {
    // Ajuste de brillo basado en la profundidad
    let brightness_factor: f32 = 1.0 - (fragment.depth * 0.5);
    let brightness_factor = brightness_factor.clamp(0.0, 1.0);

    // Aplicar el ajuste de brillo al color base
    let mut adjusted_color = Color {
        r: (base_color.r as f32 * brightness_factor) as u8,
        g: (base_color.g as f32 * brightness_factor) as u8,
        b: (base_color.b as f32 * brightness_factor) as u8,
    };

    // Multiplicar el color ajustado por la intensidad de la luz
    adjusted_color = adjusted_color * fragment.intensity;
    adjusted_color
}

// Modificado para retornar un Color
pub fn pattern_fragment_shader(fragment: &Fragment) -> Color {
    // Definir colores base para las líneas horizontales
    let color1 = Color::new(91, 153, 194);
    let color2 = Color::new(26, 72, 112);

    // Definir el patrón de líneas horizontales basado en la posición Y
    let stripe_height = 10.0; // Tamaño de cada línea
    let t = ((fragment.vertex_position.y * 100.0).abs() / stripe_height).fract(); // Fracción dentro de la línea

    // Interpolar entre color1 y color2 usando la función lerp
    let new_color = color1.lerp(&color2, t);

    // Ajustar el color de profundidad y retornar el color final
    depth_based_fragment_shader(fragment, new_color)
}



pub fn time_based_fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Usar el tiempo para controlar el aumento progresivo de la intensidad del color rojo
    let red_intensity = (uniforms.time as f32 * 0.05).sin();  // Cambia el rojo con el tiempo
    let dynamic_red_intensity = (red_intensity + 1.0) / 2.0;  // Convertir de -1 a 1 en 0 a 1 (rango entre 0 y 1)

    // Mantener una mezcla entre el color original y el rojo creciente
    let base_color = fragment.color;
    let red_color = Color::new(255, 0, 0);  // Color rojo puro

    // Interpolar entre el color base y el rojo creciente
    let mixed_color = base_color.lerp(&red_color, dynamic_red_intensity);

    // Mantener algo de transparencia para conservar la figura
    let transparency_factor = 0.5;  // Mantener la transparencia en un nivel constante
    let transparent_color = mixed_color * transparency_factor;  // Aplicar la transparencia

    // Aplicar el ajuste de brillo en función de la profundidad (sombra)
    depth_based_fragment_shader(fragment, transparent_color)
}

pub fn simple_pattern_shader(fragment: &Fragment) -> Color {
    let stripe_width = 20.0; // Define el ancho de cada franja
    let t = (fragment.position.x / stripe_width).fract(); // Fracción dentro de la franja

    // Alternar entre blanco y negro basado en la fracción
    if t > 0.5 {
        Color::new(255, 255, 255) // Blanco
    } else {
        Color::new(0, 0, 0) // Negro
    }
}


pub fn moving_pattern_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let time_factor = (uniforms.time as f32 * 0.1).sin(); // Factor de tiempo basado en seno

    let stripe_width = 20.0; // Define el ancho de la franja
    let movement_offset = time_factor * 50.0; // Movimiento basado en el tiempo
    let t = ((fragment.position.x + movement_offset) / stripe_width).fract(); // Fracción dentro de la franja

    // Alternar entre rojo y azul basado en la fracción
    if t > 0.5 {
        Color::new(255, 0, 0) // Rojo
    } else {
        Color::new(0, 0, 255) // Azul
    }
}


pub fn combined_pattern_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Aplicar el primer patrón
    let simple_color = simple_pattern_shader(fragment);

    // Si el color es blanco (255, 255, 255), aplicamos el segundo patrón
    if simple_color == Color::new(255, 255, 255) {
        moving_pattern_shader(fragment, uniforms)
    } else {
        // Si no, dejamos el resultado del primer shader
        simple_color
    }
}



pub fn multiply_shader(fragment: &Fragment, blend_color: Color) -> Color {
    // Aplicar un color base normal
    let base_color = fragment.color.blend_normal(&blend_color);
    
    // Aplicar multiplicación en áreas iluminadas
    let blended_color = base_color.blend_multiply(&Color::new(50, 100, 150)); // Color base para multiplicar
    
    // Retornar el color final
    blended_color
}


pub fn screen_shader(fragment: &Fragment, blend_color: Color) -> Color {
    // Aplicar el color base con una mezcla de pantalla
    let base_color = fragment.color.blend_screen(&blend_color);

    // Retornar el color final
    base_color
}


pub fn add_shader(fragment: &Fragment, blend_color: Color) -> Color {
    // Sumar los colores para un brillo intenso
    let base_color = fragment.color.blend_add(&blend_color);

    // Retornar el color final
    base_color
}


pub fn overlay_shader(fragment: &Fragment, blend_color: Color) -> Color {
    // Aplicar el modo overlay para aumentar el contraste
    let base_color = fragment.color.blend_overlay(&blend_color);

    // Retornar el color final
    base_color
}


pub fn time_movement_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Cambiar el color con el tiempo para crear movimiento
    let time_factor = (uniforms.time as f32 * 0.1).sin();
    let blend_color = Color::new(100, (time_factor * 255.0) as u8, 150);

    // Usar el modo overlay para crear contraste dinámico
    let overlay_color = fragment.color.blend_overlay(&blend_color);

    // Crear un efecto de pantalla en las áreas más claras
    let final_color = overlay_color.blend_screen(&Color::new(200, 200, 200));

    // Retornar el color final
    final_color
}

pub fn exceptional_fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Oscilaciones para crear distorsión
    let wave = (uniforms.time as f32 * 0.05).sin() + (fragment.position.y * 0.1).sin();
    let time_factor = (uniforms.time as f32 * 0.05).cos();

    // Mezcla de colores base, dinámico con el tiempo
    let base_color1 = Color::new(255, 0, 128);  // Un magenta vibrante
    let base_color2 = Color::new(0, 128, 255);  // Un azul eléctrico
    let blended_color = base_color1.lerp(&base_color2, (wave + 1.0) / 2.0);  // Mezcla dinámica con el tiempo

    // Aplicar un efecto de brillo basado en la profundidad (más cerca, más brillante)
    let brightness_factor: f32 = 1.0 - (fragment.depth * 0.7);
    let brightness_factor = brightness_factor.clamp(0.0, 1.0);
    
    let bright_color = blended_color * brightness_factor;  // Color con brillo ajustado

    // Efecto de movimiento en los colores con wave y blend modes
    let secondary_color = Color::new(255, 255, 0);  // Amarillo brillante para el efecto de movimiento
    let moving_color = bright_color.blend_add(&secondary_color);

    // Efecto de fusión: Usamos el *blend multiply* para dar un aspecto distorsionado
    let final_color = moving_color.blend_multiply(&bright_color);

    // Aplicar la intensidad para un efecto de vibración
    let vibrated_color = final_color * (fragment.intensity * wave.abs());

    // Retornar el color vibrado
    vibrated_color
}



// Función para calcular un ruido simple 2D basado en coordenadas (x, y)
pub fn noise_2d(x: f32, y: f32) -> Color {
    // Mezclamos las coordenadas para generar un valor pseudoaleatorio pero consistente
    let seed = (x * 12.9898 + y * 78.233).sin() * 43758.5453;
    let noise_value = seed.fract();

    // Mapear el valor escalar de ruido a un color en escala de grises
    let intensity = (noise_value * 255.0) as u8;
    Color::new(intensity, intensity, intensity)
}

// Función de interpolación para suavizar el ruido, con salida en Color
pub fn smooth_noise(x: f32, y: f32, offset_x: f32, offset_y: f32) -> Color {
    // Ajustamos las coordenadas con el offset
    let new_x = x + offset_x;
    let new_y = y + offset_y;

    // Generamos el ruido para las coordenadas ajustadas
    noise_2d(new_x, new_y)
}





// Shader basado en ruido que devuelve un Color
pub fn noise_based_fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Obtener la posición de los fragmentos en el espacio de coordenadas de ruido
    let noise_value = uniforms.noise.get_noise_2d(fragment.position.x, fragment.position.y);

    // Convertir el valor del ruido (que está entre -1 y 1) a un rango de color (0 a 255)
    let normalized_noise = ((noise_value + 1.0) / 2.0 * 255.0).clamp(0.0, 255.0) as u8;

    // Asignar colores en función del valor de ruido en escala de grises
    Color::new(normalized_noise, normalized_noise, normalized_noise)
}

// Shader para crear nubes en movimiento sobre una superficie de planeta, devolviendo un Color
pub fn moving_clouds_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Usar tiempo para animar las nubes en el eje x
    let time_factor = uniforms.time as f32 * 1.1; // Ajusta la velocidad de movimiento de las nubes

    // Obtener el valor de ruido para la posición actual del fragmento (x, y), desplazado por el tiempo para simular movimiento
    let noise_value = uniforms.noise.get_noise_2d(fragment.position.x + time_factor, fragment.position.y);

    // Convertir el ruido en un rango de 0 a 1 para usarlo como valor de opacidad
    let cloud_opacity = (noise_value + 1.0) / 2.0;

    // Definir el color del planeta como un azul profundo y el de las nubes como blanco
    let planet_color = Color::new(0, 0, 139);   // Azul oscuro para la superficie del planeta
    let cloud_color = Color::new(255, 255, 255); // Blanco para las nubes

    // Interpolar entre el color del planeta y el color de las nubes usando el valor de opacidad de las nubes
    planet_color.lerp(&cloud_color, cloud_opacity)
}

///--------ACA TODAVIA RETORNAN FRAGMENTS-------///

pub fn create_plant_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(742); // Puedes probar diferentes seeds aquí
    noise.set_noise_type(Some(NoiseType::Cellular));
    noise.set_frequency(Some(0.05)); // Baja frecuencia para detalles grandes
    noise.set_fractal_type(Some(FractalType::FBm)); // Fractal para agregar capas de detalle
    noise.set_fractal_octaves(Some(3)); // Ajusta octavas para variar la complejidad
    noise
}

pub fn plant_texture_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Obtiene el valor de ruido basado en la posición de los fragmentos
    let noise_value = uniforms.noise.get_noise_2d(fragment.position.x, fragment.position.y);

    // Normaliza el valor de ruido de [-1, 1] a [0, 1]
    let normalized_noise = (noise_value + 1.0) / 2.0;

    // Definir colores de las plantas (verde oscuro y claro)
    let color_dark_green = Color::new(34, 139, 34); // Verde oscuro
    let color_light_green = Color::new(144, 238, 144); // Verde claro

    // Interpolación entre colores de plantas
    color_dark_green.lerp(&color_light_green, normalized_noise)
}

pub fn create_cracked_earth_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(2312); // Probar diferentes seeds para el mejor efecto
    // Configurar el ruido celular
    noise.set_noise_type(Some(NoiseType::Cellular)); // Usamos Cellular Noise para las grietas
    noise.set_frequency(Some(10.1));  // Aumentar la frecuencia hace que las grietas sean más densas
    noise.set_fractal_type(Some(FractalType::FBm));  // Fractal para añadir más detalle en pequeñas escalas
    noise.set_fractal_octaves(Some(10));  // Añade más octavas para aumentar el nivel de detalle
    noise.set_fractal_lacunarity(Some(10.0));  // Aumenta la lacunarity para más irregularidad
    noise.set_fractal_gain(Some(3.5));  // Ajusta el gain para que las pequeñas grietas sean más notables
    
    noise
}

pub fn cracked_earth_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Escala de UV para obtener mayor densidad en las fracturas
    let scale_factor = 1.0; 
    let uv = nalgebra_glm::vec2(fragment.position.x, fragment.position.y) * scale_factor;

    // Obtener valor de ruido celular para simular grietas
    let noise_value = uniforms.noise.get_noise_2d(uv.x, uv.y);
    let normalized_noise = (noise_value + 1.0) / 2.0; // Normalizar ruido a 0-1

    // Definir colores base para tierra y grietas
    let base_color = Color::new(150, 111, 80); // Tierra
    let crack_color = Color::new(100, 80, 60); // Grietas profundas

    // Aplicar umbrales para acentuar fracturas
    if normalized_noise < 0.4 {
        crack_color // Grieta
    } else if normalized_noise > 0.7 {
        base_color.lerp(&Color::new(200, 160, 130), 0.3) // Suavizado en bordes
    } else {
        base_color // Tierra
    }
}

