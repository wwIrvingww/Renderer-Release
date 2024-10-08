mod framebuffer;
mod color;
mod vertex;
mod fragment;
mod line;
mod triangle;
mod obj;
mod shader;
mod uniforms; // Importar uniforms.rs

use framebuffer::Framebuffer;
use color::Color;
use nalgebra_glm::{Vec3, Mat4};
use obj::Obj;
use shader::vertex_shader;
use uniforms::Uniforms; // Importar Uniforms desde uniforms.rs
use vertex::Vertex;     // Asegurarse de importar Vertex correctamente


fn create_model_matrix(translation: Vec3, scale: f32, _rotation: Vec3) -> Mat4 {
    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );
    transform_matrix
}

fn main() {
    // Tamaño del framebuffer (ventana)
    let width = 800;
    let height = 600;

    // Crear una instancia del framebuffer
    let mut framebuffer = Framebuffer::new(width, height);

    // Establecer un color de fondo (por ejemplo, negro)
    let background_color = Color::new(0, 0, 20);
    framebuffer.clear(background_color);

    // Cargar el archivo OBJ
    let obj_model = Obj::load("assets/cube.obj").expect("Error cargando el archivo OBJ");

    // Obtener el array de vértices
    let vertices = obj_model.get_vertex_array();
    println!("Número de vértices cargados: {}", vertices.len());

    // Crear la matriz de modelo
    let translation = Vec3::new(0.0, 0.0, 0.0);  // Sin traslación por ahora
    let scale = 1.0;  // Escala normal
    let rotation = Vec3::new(0.0, 0.0, 0.0);  // Sin rotación

    let model_matrix = create_model_matrix(translation, scale, rotation);
    
    // Crear la estructura Uniforms
    let uniforms = Uniforms {
        model_matrix,
    };

    // Procesar los vértices mediante el vertex shader y dibujarlos
    let transformed_vertices: Vec<Vertex> = vertices
        .iter()
        .map(|vertex| vertex_shader(&vertex, &uniforms))
        .collect();

    // Escalar para asegurar que los vértices estén dentro de la ventana
    let scale_factor = 100.0;
    let offset_x = width as f32 / 2.0;
    let offset_y = height as f32 / 2.0;

    // Dibujar los vértices en el framebuffer
    for vertex in transformed_vertices {
        let x = vertex.position.x * scale_factor + offset_x;
        let y = vertex.position.y * scale_factor + offset_y;

        // Nos aseguramos de que los vértices están dentro de los límites de la ventana antes de dibujarlos
        if x >= 0.0 && x < width as f32 && y >= 0.0 && y < height as f32 {
            framebuffer.set_current_color(vertex.color);  // Usamos el color del vértice
            framebuffer.point(x as isize, y as isize);  // Dibujamos el vértice
        }
    }

    // Renderizar la ventana con el contenido del framebuffer
    framebuffer.render_window();
}

