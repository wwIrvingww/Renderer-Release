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


