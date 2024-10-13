use crate::framebuffer::Framebuffer;
use crate::vertex::Vertex;
use crate::fragment::Fragment; // Cambiar para usar Fragment en lugar de Vertex
use crate::color::Color;
use crate::uniforms::Uniforms;
use nalgebra_glm::Vec3;

// Simulación de un simple vertex shader que transforma las posiciones
pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    // Aplicamos transformaciones modelo, vista y proyección
    let transformed_position = uniforms.projection_matrix
        * uniforms.view_matrix
        * uniforms.model_matrix
        * vertex.position.to_homogeneous();

    // Retornamos el vértice transformado
    Vertex {
        transformed_position: Vec3::new(
            transformed_position.x,
            transformed_position.y,
            transformed_position.z,
        ),
        ..*vertex // Copiar el resto de las propiedades del vértice
    }
}

// Simulación de un fragment shader básico que cambia el color
pub fn fragment_shader(fragment: &Fragment) -> Color {  // Cambiamos para aceptar un Fragment
    // Cambiar el color del fragmento basado en su posición, como un ejemplo simple
    Color::new(
        (fragment.position.x.abs() * 255.0) as u8,
        (fragment.position.y.abs() * 255.0) as u8,
        128, // Un valor constante en azul
    )
}

// Función para renderizar los vértices utilizando los shaders
pub fn render_with_shaders(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
) {
    let mut transformed_vertices = Vec::new();

    // Vertex Shader Stage: aplicamos las transformaciones a los vértices
    for vertex in vertex_array {
        let transformed_vertex = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed_vertex);
    }

    // Rasterization Stage: convertimos los vértices en fragmentos (triángulos)
    let mut fragments = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            let v1 = &transformed_vertices[i];
            let v2 = &transformed_vertices[i + 1];
            let v3 = &transformed_vertices[i + 2];

            // Usamos la función triangle para rasterizar los triángulos en fragmentos
            fragments.extend(crate::triangle::triangle(v1, v2, v3));
        }
    }

    // Fragment Shader Stage: procesamos los fragmentos y los dibujamos en el framebuffer
    for fragment in fragments {
        let color = fragment_shader(&fragment); // Aplicamos el fragment shader
        framebuffer.set_current_color(color);
        framebuffer.point(fragment.position.x as isize, fragment.position.y as isize);
    }
}
