mod framebuffer;
mod color;
mod vertex;
mod fragment;
mod line;
mod triangle;
mod obj;
mod uniforms;
mod shader; // Importar el módulo shader

use framebuffer::Framebuffer;
use color::Color;
use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
use obj::Obj;
use uniforms::Uniforms;
use shader::render_with_shaders;

fn main() {
    // Tamaño del framebuffer (ventana)
    let width = 800;
    let height = 600;

    // Crear una instancia del framebuffer
    let mut framebuffer = Framebuffer::new(width, height);

    // Establecer un color de fondo (por ejemplo, negro)
    let background_color = Color::new(0, 0, 20);
    framebuffer.clear(background_color);

    // Cargar un archivo .obj
    let obj = Obj::load("src/assets/cube.obj").expect("Failed to load .obj file");

    // Obtener un array de vértices desde el archivo .obj
    let vertex_array = obj.get_vertex_array();

    // Configurar matrices de transformación
    let model_matrix = Mat4::identity(); // No se aplica transformación en el modelo
    let view_matrix = look_at(
        &Vec3::new(0.0, 0.0, 5.0), // Cámara en (0, 0, 5)
        &Vec3::new(0.0, 0.0, 0.0), // Mira hacia el origen
        &Vec3::new(0.0, 1.0, 0.0), // Vector "arriba" en Y
    );
    let projection_matrix = perspective(800.0 / 600.0, 45.0_f32.to_radians(), 0.1, 100.0);

    // Crear la estructura Uniforms con las matrices
    let uniforms = Uniforms::new(model_matrix, view_matrix, projection_matrix);

    // Renderizar el objeto utilizando los shaders
    render_with_shaders(&mut framebuffer, &uniforms, &vertex_array);

    // Renderizar la ventana con el contenido del framebuffer
    framebuffer.render_window();
}
