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

pub fn fragment_shader(fragment: &Fragment) -> Fragment {
    // Aquí puedes modificar el color del fragmento, por ejemplo
    // aplicando una simple reducción de brillo basada en la profundidad.

    let brightness_factor: f32 = 1.0 - (fragment.depth * 0.5); // Ejemplo de ajuste por profundidad
    let brightness_factor = brightness_factor.clamp(0.0, 1.0);

    let new_color = Color {
        r: (fragment.color.r as f32 * brightness_factor) as u8,
        g: (fragment.color.g as f32 * brightness_factor) as u8,
        b: (fragment.color.b as f32 * brightness_factor) as u8,
    };

    // Aquí es donde agregamos los campos 'normal' e 'intensity' faltantes.
    Fragment {
        position: fragment.position,
        color: new_color,
        depth: fragment.depth,
        normal: fragment.normal,  // Mantener el valor original
        intensity: fragment.intensity,  // Mantener el valor original
    }
}
