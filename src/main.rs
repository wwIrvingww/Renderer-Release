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
mod planets_shader;
mod texture;
mod normal_map;
mod skybox;

use skybox::Skybox;
use normal_map::{init_normal_map, with_normal_map};
use texture::{init_texture, with_texture};
use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use triangle::triangle;
use shader::{vertex_shader, pattern_fragment_shader};  
use planets_shader::{rocky_planet_shader, 
                    gaseous_planet_shader, 
                    frozen_planet_shader, 
                    earth_planet_shader, 
                    oceanic_planet_shader,
                    ufo_shader,
                    gargantua_shader,
                    wormhole_shader};  
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
    emission_intensity: f32,  // Escala de emisión global
}

enum PlanetShader {
    Rocky,
    Gaseous,
    Frozen,
    Earth,
    Oceanic,
    Ufo,
    Gargantua,
    Wormhole
}

enum CurrentModel {
    Sphere,
    Ufo,
    Eye,
}


fn blend_screen(base: u32, emission: u32) -> u32 {
    let base_r = (base >> 16) & 0xFF;
    let base_g = (base >> 8) & 0xFF;
    let base_b = base & 0xFF;

    let emission_r = (emission >> 16) & 0xFF;
    let emission_g = (emission >> 8) & 0xFF;
    let emission_b = emission & 0xFF;

    let screen_r = 255 - (((255 - base_r) * (255 - emission_r)) / 255);
    let screen_g = 255 - (((255 - base_g) * (255 - emission_g)) / 255);
    let screen_b = 255 - (((255 - base_b) * (255 - emission_b)) / 255);

    (screen_r << 16) | (screen_g << 8) | screen_b
}


// Inicializa la cámara en función del modelo seleccionado
fn initialize_camera(model: &CurrentModel) -> Camera {
    match model {
        CurrentModel::Sphere => Camera::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
        CurrentModel::Ufo => Camera::new(Vec3::new(0.0, 0.0, 35.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
        CurrentModel::Eye => Camera::new(Vec3::new(0.0, 0.0, 15.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),

    }
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

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], planet_shader: &PlanetShader) {
    
    // Vertex Shader Stage:
    // Transforma cada vértice utilizando las matrices de transformación y proyecta a coordenadas de la cámara.
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly Stage:
    // Agrupa los vértices transformados en triángulos. Cada grupo de tres vértices se convierte en un triángulo.
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

    // Rasterization Stage:
    // Convierte los triángulos en fragmentos (píxeles de la pantalla) que serán procesados individualmente.
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Aplica el shader seleccionado a cada fragmento para determinar el color final del pixel.
    // Este shader se selecciona según el tipo de planeta u objeto (por ejemplo, `PlanetShader::Rocky` para un planeta rocoso).
    // Fragment Processing Stage:
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            let (shaded_color, emission_color) = match planet_shader {
                PlanetShader::Rocky => (rocky_planet_shader(&fragment, uniforms), None),
                PlanetShader::Gaseous => (gaseous_planet_shader(&fragment, uniforms), None),
                PlanetShader::Frozen => (frozen_planet_shader(&fragment, uniforms), None),
                PlanetShader::Earth => (earth_planet_shader(&fragment, uniforms), None),
                PlanetShader::Oceanic => (oceanic_planet_shader(&fragment, uniforms), None),
                PlanetShader::Ufo => (ufo_shader(&fragment, uniforms), None),
                PlanetShader::Gargantua => gargantua_shader(&fragment, uniforms),
                PlanetShader::Wormhole => wormhole_shader(&fragment, uniforms),
            };
            
    
            framebuffer.set_current_color(shaded_color.to_hex());
            framebuffer.point(x, y, fragment.depth);
    
            if let Some(emission) = emission_color {
                framebuffer.set_emission_color(x, y, emission.to_hex());
            }
        }
    }
    
    // Segunda pasada: Combinar el buffer de emisión en el buffer principal
    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let emission_color = framebuffer.emission_buffer[y * framebuffer.width + x];
            let current_color = framebuffer.buffer[y * framebuffer.width + x];
    
            // Mezclamos los colores con el buffer de emisión usando un modo de mezcla (screen blending)
            framebuffer.buffer[y * framebuffer.width + x] = blend_screen(current_color, emission_color);
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

    framebuffer.set_background_color(0x000000);

    let noise = create_cracked_earth_noise();
    
    let sphere_obj = Obj::load("src/assets/sphere.obj").expect("Failed to load sphere.obj");
    let ufo_obj = Obj::load("src/assets/ufo.obj").expect("Failed to load ufo.obj");
    let eye_obj = Obj::load("src/assets/eye.obj").expect("Failed to load sphere.obj");

    // Inicializar la textura
    init_texture("src/assets/textures/water.png").expect("Failed to initialize texture");

    // Inicializar el mapa normal
    init_normal_map("src/assets/textures/water.png").expect("Failed to load normal map");

    // Inicializar el Skybox después de cargar otros recursos
    // let skybox = Skybox::new("src/assets/textures/stars.jpg");
    let skybox = Skybox::new(500); // Genera 500 estrellas



    let mut time_counter = 0;
    let mut current_planet_shader = PlanetShader::Rocky;
    let mut current_model = CurrentModel::Sphere; // Empezamos con el modelo esfera

    // Inicializa la cámara en función del modelo actual
    let mut camera = initialize_camera(&current_model);

    // Inicializar el nivel de emision
    let mut emission_intensity = 1.00;

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }
    
        handle_input(&window, &mut camera);
        handle_key_input(&window, &mut current_planet_shader, &mut current_model, &mut camera);
                
        framebuffer.clear();
    
        // Crear uniforms antes de renderizar el skybox
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
            emission_intensity: emission_intensity,
        };
    
        // Renderizar el skybox antes de los objetos
        skybox.render(&mut framebuffer, &uniforms, camera.eye);
    
        // Seleccionar el vertex array basado en el modelo actual
        let vertex_arrays = match current_model {
            CurrentModel::Sphere => sphere_obj.get_vertex_array(),
            CurrentModel::Ufo => ufo_obj.get_vertex_array(),
            CurrentModel::Eye => eye_obj.get_vertex_array(),
        };
    
        framebuffer.set_current_color(0xFFDDDD);
        render(&mut framebuffer, &uniforms, &vertex_arrays, &current_planet_shader);
    
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();
    
        std::thread::sleep(frame_delay);
    }   
    
}

// Función para manejar la selección de shaders y modelos de planeta
fn handle_key_input(window: &Window, current_shader: &mut PlanetShader, current_model: &mut CurrentModel, camera: &mut Camera) {
    if window.is_key_down(Key::Key1) {
        *current_shader = PlanetShader::Rocky;
        *current_model = CurrentModel::Sphere; // Volver al modelo Sphere
        *camera = initialize_camera(current_model); // Reinicializar la cámara
    }
    if window.is_key_down(Key::Key2) {
        *current_shader = PlanetShader::Gaseous;
        *current_model = CurrentModel::Sphere; // Volver al modelo Sphere
        *camera = initialize_camera(current_model); // Reinicializar la cámara
    }
    if window.is_key_down(Key::Key3) {
        *current_shader = PlanetShader::Frozen;
        *current_model = CurrentModel::Sphere; // Volver al modelo Sphere
        *camera = initialize_camera(current_model); // Reinicializar la cámara
    }
    if window.is_key_down(Key::Key4) {
        *current_shader = PlanetShader::Earth;
        *current_model = CurrentModel::Sphere; // Volver al modelo Sphere
        *camera = initialize_camera(current_model); // Reinicializar la cámara
    }
    if window.is_key_down(Key::Key5) {
        *current_shader = PlanetShader::Oceanic;
        *current_model = CurrentModel::Eye; // Volver al modelo Sphere
        *camera = initialize_camera(current_model); // Reinicializar la cámara
    }
    if window.is_key_down(Key::Key6) {
        *current_shader = PlanetShader::Ufo;
        *current_model = CurrentModel::Ufo; // Cambiar al modelo UFO
        *camera = initialize_camera(current_model); // Reinicializar la cámara
    }
    if window.is_key_down(Key::Key7) {
        *current_shader = PlanetShader::Gargantua;
        *current_model = CurrentModel::Eye; // Volver al modelo Sphere
        *camera = initialize_camera(current_model); // Reinicializar la cámara
    }
    if window.is_key_down(Key::Key8) {
        *current_shader = PlanetShader::Wormhole;
        *current_model = CurrentModel::Sphere; // Volver al modelo Sphere
        *camera = initialize_camera(current_model); // Reinicializar la cámara
    }

}


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
