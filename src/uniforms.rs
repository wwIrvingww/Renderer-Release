use crate::framebuffer::Framebuffer;
use crate::vertex::Vertex;
use crate::fragment::Fragment;

pub struct Uniforms {
    pub model_matrix: nalgebra_glm::Mat4,
    pub view_matrix: nalgebra_glm::Mat4,
    pub projection_matrix: nalgebra_glm::Mat4,
}

impl Uniforms {
    pub fn new(model_matrix: nalgebra_glm::Mat4, view_matrix: nalgebra_glm::Mat4, projection_matrix: nalgebra_glm::Mat4) -> Self {
        Uniforms {
            model_matrix,
            view_matrix,
            projection_matrix,
        }
    }
}

pub fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // Vertex Processing Stage: transformar los vértices
    let mut transformed_vertices = Vec::new();
    for vertex in vertex_array {
        // Aplicar las transformaciones del pipeline (modelo, vista y proyección)
        let transformed_position = uniforms.projection_matrix
            * uniforms.view_matrix
            * uniforms.model_matrix
            * vertex.position.to_homogeneous();

        let transformed_vertex = Vertex {
            transformed_position: nalgebra_glm::Vec3::new(
                transformed_position.x,
                transformed_position.y,
                transformed_position.z,
            ),
            ..*vertex // Copiar el resto de las propiedades del vértice
        };

        transformed_vertices.push(transformed_vertex);
    }

    // Rasterization Stage: convertir los vértices en fragmentos (líneas o triángulos)
    let mut fragments = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            let v1 = &transformed_vertices[i];
            let v2 = &transformed_vertices[i + 1];
            let v3 = &transformed_vertices[i + 2];

            // Aquí usamos la función triangle para rasterizar los vértices en fragmentos
            fragments.extend(crate::triangle::triangle(v1, v2, v3));
        }
    }

    // Fragment Processing Stage: dibujar los fragmentos en el framebuffer
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        framebuffer.set_current_color(fragment.color);
        framebuffer.point(x as isize, y as isize);
    }
}
