use nalgebra_glm::{look_at, perspective, Vec3, Mat4, Mat3};  // Importa la función perspective
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
mod camera;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use triangle::triangle;
use shader::{vertex_shader, cracked_earth_shader, pattern_fragment_shader};  
use camera::Camera;  
use fastnoise_lite::{FastNoiseLite, NoiseType, FractalType};

// Corrige la estructura `Uniforms` especificando los tipos y valores de manera adecuada
pub struct Uniforms<'a> {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    transformation_matrix: Mat4,
    normal_matrix: Mat3,
    time: u32,
    noise: &'a FastNoiseLite,
}

fn create_cracked_earth_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(42);
    noise.set_noise_type(Some(NoiseType::Cellular));
    noise.set_fractal_type(Some(FractalType::FBm)); // Fractal para detalles
    noise.set_frequency(Some(0.03)); // Ajustar frecuencia para simular "tierra quebrada"
    noise
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn create_projection_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = PI / 4.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 100.0;
    perspective(fov, aspect_ratio, near, far)
}

fn create_model_matrix() -> Mat4 {
    Mat4::identity()
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

    // Fragment Processing Stage con el shader de tierra quebrada
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            // Aplicar el shader de tierra quebrada a cada fragmento
            let shaded_color = pattern_fragment_shader(&fragment);
            framebuffer.set_current_color(shaded_color.color.to_hex());
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn main() {
    let window_width = 1000;
    let window_height = 1000;
    let framebuffer_width = 1000;
    let framebuffer_height = 1000;
    let frame_delay = Duration::from_millis(10);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Cracked Earth - Renderer",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    framebuffer.set_background_color(0x433878);

    let mut camera = Camera::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));

    // Inicializar el ruido para tierra quebrada
    let noise = create_cracked_earth_noise();

    let obj = Obj::load("src/assets/sphere.obj").expect("Failed to load obj");
    let vertex_arrays = obj.get_vertex_array();

    // Contador de tiempo para el shader
    let mut time_counter = 0;

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Manejar la entrada de la cámara para orbit y zoom
        handle_input(&window, &mut camera);

        framebuffer.clear();

        // Calcular matrices
        let model_matrix = create_model_matrix();
        let view_matrix = camera.get_view_matrix();
        let projection_matrix = create_projection_matrix(window_width as f32, window_height as f32);
        let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);

        // La matriz de transformación principal
        let transformation_matrix = projection_matrix * view_matrix * model_matrix;

        // Calcular la matriz de transformación de normales:
        let normal_matrix = model_matrix.fixed_resize::<3, 3>(0.0).try_inverse().unwrap().transpose();

        // Actualizar el contador de tiempo
        time_counter += 1;

        // Pasar noise, time y normal_matrix al Uniforms
        let uniforms = Uniforms {
            model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            transformation_matrix,
            normal_matrix,
            time: time_counter,
            noise: &noise,
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
fn handle_input(window: &Window, camera: &mut Camera) {
    let orbit_speed = PI / 50.0;  // Ajustar la velocidad de la órbita
    let zoom_speed = 0.5;  // Ajustar la velocidad del zoom

    // Orbitar con las teclas de flecha
    if window.is_key_down(Key::Left) {
        camera.orbit(orbit_speed, 0.0);  // Rotar alrededor del eje Y
    }
    if window.is_key_down(Key::Right) {
        camera.orbit(-orbit_speed, 0.0);  // Rotar alrededor del eje Y en la otra dirección
    }
    if window.is_key_down(Key::Up) {
        camera.orbit(0.0, orbit_speed);  // Rotar alrededor del eje X (arriba/abajo)
    }
    if window.is_key_down(Key::Down) {
        camera.orbit(0.0, -orbit_speed);  // Rotar hacia abajo
    }

    // Zoom con W y S
    if window.is_key_down(Key::W) {
        camera.zoom(-zoom_speed);  // Acercar
    }
    if window.is_key_down(Key::S) {
        camera.zoom(zoom_speed);  // Alejar
    }
}
