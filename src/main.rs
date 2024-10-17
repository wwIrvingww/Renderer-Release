use nalgebra_glm::{look_at, perspective, Vec3, Mat4};  // Importa la función perspective
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;

mod framebuffer;
mod triangle;
mod line;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shader;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use triangle::triangle;
use shader::vertex_shader;

// Estructura de uniformes con la matriz de proyección
pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

// Función para crear la matriz de proyección
fn create_projection_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;  // Campo de visión en radianes
    let aspect_ratio = window_width / window_height;  // Relación de aspecto
    let near = 0.1;  // Plano de recorte cercano
    let far = 100.0;  // Plano de recorte lejano
    perspective(fov, aspect_ratio, near, far)
}

// Función para crear la matriz de vista
fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

// Función para crear la matriz de modelo (matriz identidad)
fn create_model_matrix() -> Mat4 {
    Mat4::identity()  // Devolver la matriz identidad
}

// Render loop
fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly Stage
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Fragment Processing Stage
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            let color = fragment.color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(10);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "INTERSTELLAR",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(0x433878);

    // Parámetros de modelo
    let mut translation = Vec3::new(300.0, 550.0, 0.0); 
    let mut rotation = Vec3::new(0.0, std::f32::consts::PI / 10.0, 0.0); 
    let mut scale = 25.0f32;

    // Parámetros de la cámara (eye, target, up)
    let mut eye = Vec3::new(0.0, 0.0, 20.0);  // Posición de la cámara
    let mut target = Vec3::new(0.0, 0.0, 0.0);  // A dónde está mirando
    let up = Vec3::new(0.0, 1.0, 0.0);  // Vector "arriba" de la cámara

    let obj = Obj::load("src/assets/spaceship.obj").expect("Failed to load obj");
    let vertex_arrays = obj.get_vertex_array(); 

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        handle_input(&window, &mut translation, &mut rotation, &mut scale, &mut eye, &mut target);

        framebuffer.clear();

        // Calcular matrices
        let model_matrix = create_model_matrix();
        let view_matrix = create_view_matrix(eye, target, up);  // Matriz de vista
        let projection_matrix = create_projection_matrix(window_width as f32, window_height as f32);  // Matriz de proyección
        let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);  // Matriz de viewport

        let uniforms = Uniforms {
            model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,  // Añadir matriz de viewport a los uniformes
        };

        framebuffer.set_current_color(0xFFDDDD);
        render(&mut framebuffer, &uniforms, &vertex_arrays);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

// Manejo de entrada para mover la cámara
fn handle_input(window: &Window, _translation: &mut Vec3, _rotation: &mut Vec3, _scale: &mut f32, eye: &mut Vec3, target: &mut Vec3) {
    let move_speed = 10.0;
    let rotate_speed = PI / 100.0;

    // Movimiento de la cámara (eye)
    if window.is_key_down(Key::W) {
        eye.z -= move_speed * 0.1;  // Mover la cámara hacia adelante
    }
    if window.is_key_down(Key::S) {
        eye.z += move_speed * 0.1;  // Mover la cámara hacia atrás
    }
    if window.is_key_down(Key::A) {
        eye.x -= move_speed * 0.1;  // Mover la cámara hacia la izquierda
    }
    if window.is_key_down(Key::D) {
        eye.x += move_speed * 0.1;  // Mover la cámara hacia la derecha
    }

    // Rotación de la cámara (cambiando el "target" para simular rotación)
    if window.is_key_down(Key::Left) {
        target.x -= rotate_speed;  // Rotar la vista hacia la izquierda
    }
    if window.is_key_down(Key::Right) {
        target.x += rotate_speed;  // Rotar la vista hacia la derecha
    }
    if window.is_key_down(Key::Up) {
        target.y += rotate_speed;  // Rotar la vista hacia arriba
    }
    if window.is_key_down(Key::Down) {
        target.y -= rotate_speed;  // Rotar la vista hacia abajo
    }
}
