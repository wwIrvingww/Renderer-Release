use nalgebra_glm::{Vec3, Vec4};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment; // Importa la estructura Fragment
use crate::color::Color;       // Importa la estructura Color

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    // Aplicar la matriz de transformación completa (precomputada)
    let transformed = uniforms.transformation_matrix * Vec4::new(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);

    // Realizar la división por w para obtener las coordenadas NDC
    let w = transformed.w;
    let ndc_position = Vec4::new(
        transformed.x / w,
        transformed.y / w,
        transformed.z / w,
        1.0
    );

    // Crear un nuevo vértice con los atributos transformados
    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position: Vec3::new(ndc_position.x, ndc_position.y, ndc_position.z),
        transformed_normal: vertex.transformed_normal,
    }
}

// Función reutilizable para ajustar brillo basado en profundidad e intensidad
pub fn depth_based_fragment_shader(fragment: &Fragment, base_color: Color) -> Fragment {
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

    // Retornar el nuevo fragmento con el color modificado
    Fragment {
        position: fragment.position,
        color: adjusted_color,
        depth: fragment.depth,
        normal: fragment.normal,
        intensity: fragment.intensity,
    }
}

// Shader de patrones: líneas horizontales
pub fn pattern_fragment_shader(fragment: &Fragment) -> Fragment {
    // Definir colores base para las líneas horizontales
    let color1 = Color::new(91,153,194);
    let color2 = Color::new(26, 72, 112);

    // Definir el patrón de líneas horizontales basado en la posición Y
    let stripe_height = 10.0; // Tamaño de cada línea
    let t = (fragment.position.y / stripe_height).fract(); // Fracción dentro de la línea

    // Interpolar entre color1 y color2 usando la función lerp
    let new_color = color1.lerp(&color2, t);

    // Aplicar el shader de profundidad para ajustar brillo e intensidad
    depth_based_fragment_shader(fragment, new_color)
}




pub fn time_based_fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Option<Fragment> {
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

    // Aplicar el shader basado en la profundidad para ajustar sombras
    Some(depth_based_fragment_shader(fragment, transparent_color))
}

pub fn simple_pattern_shader(fragment: &Fragment) -> Fragment {
    let stripe_width = 20.0; // Define el ancho de cada franja
    let t = (fragment.position.x / stripe_width).fract(); // Fracción dentro de la franja

    // Alternar entre blanco y negro basado en la fracción
    let color = if t > 0.5 {
        Color::new(255, 255, 255) // Blanco
    } else {
        Color::new(0, 0, 0) // Negro
    };

    Fragment {
        position: fragment.position,
        color,
        depth: fragment.depth,
        normal: fragment.normal,
        intensity: fragment.intensity,
    }
}

pub fn moving_pattern_shader(fragment: &Fragment, uniforms: &Uniforms) -> Fragment {
    let time_factor = (uniforms.time as f32 * 0.1).sin(); // Factor de tiempo basado en seno

    let stripe_width = 20.0; // Define el ancho de la franja
    let movement_offset = time_factor * 50.0; // Movimiento basado en el tiempo
    let t = ((fragment.position.x + movement_offset) / stripe_width).fract(); // Fracción dentro de la franja

    // Alternar entre rojo y azul basado en la fracción
    let color = if t > 0.5 {
        Color::new(255, 0, 0) // Rojo
    } else {
        Color::new(0, 0, 255) // Azul
    };

    Fragment {
        position: fragment.position,
        color,
        depth: fragment.depth,
        normal: fragment.normal,
        intensity: fragment.intensity,
    }
}

pub fn combined_pattern_shader(fragment: &Fragment, uniforms: &Uniforms) -> Fragment {
    // Aplicar el primer patrón
    let simple_shader_result = simple_pattern_shader(fragment);

    // Si el color es blanco (255, 255, 255), aplicamos el segundo patrón
    if simple_shader_result.color == Color::new(255, 255, 255) {
        moving_pattern_shader(&simple_shader_result, uniforms)
    } else {
        // Si no, dejamos el resultado del primer shader
        simple_shader_result
    }
}


pub fn multiply_shader(fragment: &Fragment, blend_color: Color) -> Fragment {
    // Aplicar un color base normal
    let base_color = fragment.color.blend_normal(&blend_color);
    
    // Aplicar multiplicación en áreas iluminadas
    let blended_color = base_color.blend_multiply(&Color::new(50, 100, 150)); // Color base para multiplicar
    
    // Aplicar el shader de profundidad
    depth_based_fragment_shader(fragment, blended_color)
}

pub fn screen_shader(fragment: &Fragment, blend_color: Color) -> Fragment {
    // Aplicar el color base con una mezcla de pantalla
    let base_color = fragment.color.blend_screen(&blend_color);

    // Aplicar el shader de profundidad
    depth_based_fragment_shader(fragment, base_color)
}

pub fn add_shader(fragment: &Fragment, blend_color: Color) -> Fragment {
    // Sumar los colores para un brillo intenso
    let base_color = fragment.color.blend_add(&blend_color);

    // Aplicar el shader de profundidad
    depth_based_fragment_shader(fragment, base_color)
}

pub fn overlay_shader(fragment: &Fragment, blend_color: Color) -> Fragment {
    // Aplicar el modo overlay para aumentar el contraste
    let base_color = fragment.color.blend_overlay(&blend_color);

    // Aplicar el shader de profundidad
    depth_based_fragment_shader(fragment, base_color)
}

pub fn time_movement_shader(fragment: &Fragment, uniforms: &Uniforms) -> Fragment {
    // Cambiar el color con el tiempo para crear movimiento
    let time_factor = (uniforms.time as f32 * 0.1).sin();
    let blend_color = Color::new(100, (time_factor * 255.0) as u8, 150);

    // Usar el modo overlay para crear contraste dinámico
    let overlay_color = fragment.color.blend_overlay(&blend_color);

    // Crear un efecto de pantalla en las áreas más claras
    let final_color = overlay_color.blend_screen(&Color::new(200, 200, 200));

    // Aplicar el shader de profundidad para agregar sombras
    depth_based_fragment_shader(fragment, final_color)
}


pub fn exceptional_fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Fragment {
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

    // Crear y retornar el fragmento modificado
    Fragment {
        position: fragment.position,
        color: vibrated_color,
        depth: fragment.depth,
        normal: fragment.normal,
        intensity: fragment.intensity,
    }
}