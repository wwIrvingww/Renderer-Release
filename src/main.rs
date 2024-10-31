use nalgebra_glm::{look_at, perspective, Vec3, Mat4, Mat3};
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
mod planets_shader;  // Importamos el nuevo módulo de planetas

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use triangle::triangle;
use shader::{vertex_shader, pattern_fragment_shader};  
use planets_shader::{rocky_planet_shader, gaseous_planet_shader, frozen_planet_shader, earth_planet_shader, oceanic_planet_shader};  // Importamos los shaders de planetas
use camera::Camera;
use fastnoise_lite::{FastNoiseLite, NoiseType, FractalType};

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

// Inicializamos el shader predeterminado como el primer shader (rocoso)
enum PlanetShader {
    Rocky,
    Gaseous,
    Frozen,
    Earth,
    Oceanic,
}

fn create_cracked_earth_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(42);
    noise.set_noise_type(Some(NoiseType::Cellular));
    noise.set_fractal_type(Some(FractalType::FBm));
    noise.set_frequency(Some(0.03));
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

// Cambia la firma de `render` para aceptar una referencia a `PlanetShader`
fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], planet_shader: &PlanetShader) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

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

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            // Aplicar el shader de planeta seleccionado
            let shaded_color = match planet_shader {
                PlanetShader::Rocky => rocky_planet_shader(&fragment, uniforms),
                PlanetShader::Gaseous => gaseous_planet_shader(&fragment, uniforms),
                PlanetShader::Frozen => frozen_planet_shader(&fragment, uniforms),
                PlanetShader::Earth => earth_planet_shader(&fragment, uniforms),
                PlanetShader::Oceanic => oceanic_planet_shader(&fragment, uniforms),
            };
            framebuffer.set_current_color(shaded_color.to_hex());
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
        "Planet Renderer",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    framebuffer.set_background_color(0x433878);

    let mut camera = Camera::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    let noise = create_cracked_earth_noise();

    let obj = Obj::load("src/assets/sphere.obj").expect("Failed to load obj");
    let vertex_arrays = obj.get_vertex_array();

    let mut time_counter = 0;
    let mut current_planet_shader = PlanetShader::Rocky;  // Shader por defecto

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        handle_input(&window, &mut camera);
        handle_shader_selection(&window, &mut current_planet_shader);

        framebuffer.clear();

        let model_matrix = create_model_matrix();
        let view_matrix = camera.get_view_matrix();
        let projection_matrix = create_projection_matrix(window_width as f32, window_height as f32);
        let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);

        let transformation_matrix = projection_matrix * view_matrix * model_matrix;
        let normal_matrix = model_matrix.fixed_resize::<3, 3>(0.0).try_inverse().unwrap().transpose();

        time_counter += 1;

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
        render(&mut framebuffer, &uniforms, &vertex_arrays, &current_planet_shader);  // Pasar `planet_shader` como referencia

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

// Función para manejar la selección de shaders de planeta
fn handle_shader_selection(window: &Window, current_shader: &mut PlanetShader) {
    if window.is_key_down(Key::Key1) {
        *current_shader = PlanetShader::Rocky;
    }
    if window.is_key_down(Key::Key2) {
        *current_shader = PlanetShader::Gaseous;
    }
    if window.is_key_down(Key::Key3) {
        *current_shader = PlanetShader::Frozen;
    }
    if window.is_key_down(Key::Key4) {
        *current_shader = PlanetShader::Earth;
    }
    if window.is_key_down(Key::Key5) {
        *current_shader = PlanetShader::Oceanic;
    }
}


// Manejo de entrada para mover la cámara
fn handle_input(window: &Window, camera: &mut Camera) {
    let orbit_speed = PI / 50.0;
    let zoom_speed = 0.5;

    if window.is_key_down(Key::Left) {
        camera.orbit(orbit_speed, 0.0);
    }
    if window.is_key_down(Key::Right) {
        camera.orbit(-orbit_speed, 0.0);
    }
    if window.is_key_down(Key::Up) {
        camera.orbit(0.0, orbit_speed);
    }
    if window.is_key_down(Key::Down) {
        camera.orbit(0.0, -orbit_speed);
    }

    if window.is_key_down(Key::W) {
        camera.zoom(-zoom_speed);
    }
    if window.is_key_down(Key::S) {
        camera.zoom(zoom_speed);
    }
}

