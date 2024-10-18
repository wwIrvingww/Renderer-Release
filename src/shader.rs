use nalgebra_glm::{Vec3, Vec4};
use crate::vertex::Vertex;
use crate::Uniforms;

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
