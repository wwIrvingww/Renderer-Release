use nalgebra_glm::{look_at, perspective, Vec3, Mat4, Mat3, Vec4};
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
    emission_intensity: f32,
}


struct Model<'a> {
    vertex_array: &'a [Vertex],
    shader: PlanetShader,
    position: Vec3,
    scale: f32,
    rotation: Vec3,
    rotation_speed: Vec3,
    collision_radius: f32, // Radio de colisión
}

#[derive(PartialEq)]
enum PlanetShader {
    Rocky,
    Gaseous,
    Frozen,
    Earth,
    Oceanic,
    Ufo,
    Gargantua,
    Wormhole,
}


enum CurrentModel {
    Sphere,
    Ufo,
    Eye,
}


fn render_orbits(framebuffer: &mut Framebuffer, models: &[Model], view_matrix: &Mat4, projection_matrix: &Mat4) {
    let orbit_points = 100; // Número de segmentos para aproximar el círculo de la órbita
    let center = Vec3::new(0.0, 0.0, 0.0); // Centro del sistema de órbitas (origen)

    for model in models {
        // Excluir los modelos que no deben tener órbita
        if model.shader == PlanetShader::Ufo || model.shader == PlanetShader::Wormhole {
            continue;
        }

        // Calcular el radio de la órbita como la distancia del planeta al centro
        let orbit_radius = nalgebra_glm::distance(&center, &model.position);

        // Generar puntos de la órbita
        let mut orbit_vertices = Vec::with_capacity(orbit_points);
        for i in 0..orbit_points {
            let angle = 2.0 * PI * (i as f32 / orbit_points as f32);
            let x = center.x + orbit_radius * angle.cos();
            let z = center.z + orbit_radius * angle.sin();
            let y = center.y; // Mantener la órbita en el mismo plano Y

            let orbit_point = Vec3::new(x, y, z);

            // Transformar el punto de la órbita usando las matrices de vista y proyección
            let model_position = nalgebra_glm::translation(&orbit_point);
            let clip_position = projection_matrix * view_matrix * model_position * Vec4::new(1.0, 1.0, 1.0, 1.0);

            // Imprimir la posición en clip space para depuración
            println!("Clip Position: {:?}", clip_position);

            orbit_vertices.push(clip_position);
        }

        // Renderizar la órbita conectando los puntos calculados
        for i in 0..orbit_points {
            let start = orbit_vertices[i];
            let end = orbit_vertices[(i + 1) % orbit_points];

            // Verificar el valor de w
            if start[3] <= 0.0 || end[3] <= 0.0 {
                continue;
            }

            // Normalizar las coordenadas
            let normalized_start_x = start[0] / start[3];
            let normalized_start_y = start[1] / start[3];
            let normalized_end_x = end[0] / end[3];
            let normalized_end_y = end[1] / end[3];

            // Mapear a coordenadas de pantalla
            let screen_start_x = ((normalized_start_x + 1.0) * 0.5 * framebuffer.width as f32) as usize;
            let screen_start_y = ((1.0 - (normalized_start_y + 1.0) * 0.5) * framebuffer.height as f32) as usize;
            let screen_end_x = ((normalized_end_x + 1.0) * 0.5 * framebuffer.width as f32) as usize;
            let screen_end_y = ((1.0 - (normalized_end_y + 1.0) * 0.5) * framebuffer.height as f32) as usize;

            // Dibujar la órbita si las coordenadas están dentro del rango del framebuffer
            if screen_start_x < framebuffer.width && screen_start_y < framebuffer.height &&
               screen_end_x < framebuffer.width && screen_end_y < framebuffer.height {
                framebuffer.draw_line(screen_start_x, screen_start_y, screen_end_x, screen_end_y, 0xFFFFFF);
            }
        }
    }
}


fn check_collision(model_a: &Model, model_b: &Model) -> bool {
    let distance = nalgebra_glm::distance(&model_a.position, &model_b.position);
    let combined_radius = model_a.collision_radius + model_b.collision_radius;
    distance < combined_radius
}

// Función para mover la nave a una posición cerca del planeta seleccionado
fn warp_to_planet(camera: &mut Camera, planet: &Model) {
    let warp_distance = 15.0; // Distancia fija para ubicarse frente al planeta

    // Actualizar la posición de la cámara: frente al planeta, a la misma altura y posición lateral
    camera.eye = Vec3::new(planet.position.x, planet.position.y, planet.position.z + warp_distance);

    // Apuntar la cámara hacia el planeta
    camera.center = planet.position;

    // Ajustar los ángulos de rotación
    let forward = camera.get_forward_vector();
    camera.pitch = forward.y.atan2((forward.x.powi(2) + forward.z.powi(2)).sqrt());
    camera.yaw = forward.z.atan2(forward.x);

    // Indicar que la cámara ha cambiado
    camera.has_changed = true;
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
        CurrentModel::Sphere => Camera::new(Vec3::new(0.0, 20.0, 30.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
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

fn create_model_matrix_with_rotation(position: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    // Crear la matriz de traslación usando `translation`
    let translation = nalgebra_glm::translation(&position);

    // Crear la matriz de escala usando `scaling`
    let scaling = nalgebra_glm::scaling(&Vec3::new(scale, scale, scale));

    // Crear la matriz de rotación alrededor del eje Y
    let rotation_matrix = nalgebra_glm::rotate_y(&Mat4::identity(), rotation.y);

    // Multiplicar las matrices: traslación * rotación * escala
    translation * rotation_matrix * scaling
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

fn create_model_matrix(position: Vec3, scale: f32) -> Mat4 {
    let translation_matrix = nalgebra_glm::translation(&position);
    let scale_matrix = nalgebra_glm::scaling(&Vec3::new(scale, scale, scale));
    translation_matrix * scale_matrix
}

fn generate_spiral_position(index: usize, base_radius: f32, height_step: f32) -> Vec3 {
    let angle = index as f32 * 0.8 * PI; // Aumentamos el ángulo para cubrir más espacio
    let radius = base_radius + index as f32 * 10.0; // Incremento del radio aumentado a 10.0
    let x = radius * angle.cos();
    let z = radius * angle.sin();
    let y = (height_step * index as f32 * 2.0) - 10.0; // Ajuste de altura para mayor separación

    Vec3::new(x, y, z)
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
                PlanetShader::Rocky => (rocky_planet_shader(&fragment, uniforms), None), //ya
                PlanetShader::Gaseous => (gaseous_planet_shader(&fragment, uniforms), None), //ya
                PlanetShader::Frozen => (frozen_planet_shader(&fragment, uniforms), None),
                PlanetShader::Earth => (earth_planet_shader(&fragment, uniforms), None), //ya
                PlanetShader::Oceanic => (oceanic_planet_shader(&fragment, uniforms), None), //ya
                PlanetShader::Ufo => (ufo_shader(&fragment, uniforms), None), //ya
                PlanetShader::Gargantua => gargantua_shader(&fragment, uniforms), //ya
                PlanetShader::Wormhole => wormhole_shader(&fragment, uniforms), //ya
            };
            
    
            framebuffer.set_current_color(shaded_color.to_hex());
            framebuffer.point(x, y, fragment.depth);

            if let Some(emission) = emission_color {
                framebuffer.set_emission_color(x, y, emission.to_hex());
            }
            //Aplicar el post-procesamiento de emisión
            // framebuffer.apply_emission();

        }
    }
    
    // Segunda pasada: Combinar el buffer de emisión en el buffer principal
    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let emission_color = framebuffer.emission_buffer[y * framebuffer.width + x];
            let current_color = framebuffer.buffer[y * framebuffer.width + x];

            // Mezclamos los colores con el buffer de emisión usando un modo de mezcla (screen blending)
            if emission_color != 0 {
                framebuffer.buffer[y * framebuffer.width + x] = blend_screen(current_color, emission_color);
            }
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

    //Inicializar nave para explorar el espacio
    let spaceship_obj = Obj::load("src/assets/spaceship.obj").expect("Failed to load spaceship.obj");

    // Inicializar la textura
    init_texture("src/assets/textures/water.png").expect("Failed to initialize texture");

    // Inicializar el mapa normal
    init_normal_map("src/assets/textures/water.png").expect("Failed to load normal map");

    // Inicializar el Skybox después de cargar otros recursos
    // let skybox = Skybox::new("src/assets/textures/stars.jpg");
    let skybox = Skybox::new(100); // Genera 500 estrellas

    // Almacenar los arrays de vértices en variables
    let sphere_vertices = sphere_obj.get_vertex_array();
    let ufo_vertices = ufo_obj.get_vertex_array();
    let eye_vertices = eye_obj.get_vertex_array();
    let spaceship_vertices = spaceship_obj.get_vertex_array();

    let mut current_model = CurrentModel::Sphere; // Empezamos con el modelo esfera

    // Inicializa la cámara en función del modelo actual
    let mut camera = initialize_camera(&current_model);

    // Crear la lista de modelos con las posiciones en espiral
    let mut models = vec![
    Model {
        vertex_array: &eye_vertices,
        shader: PlanetShader::Wormhole,
        position: Vec3::new(0.0, 0.0, 0.0), // Centro
        scale: 2.0,
        rotation: Vec3::new(0.0, 0.0, 0.0), // Inicializar rotación
        rotation_speed: Vec3::new(0.0, 0.2, 0.0), // Rotación lenta en el eje Y
        collision_radius: 3.0,
    },
    Model {
        vertex_array: &sphere_vertices,
        shader: PlanetShader::Rocky,
        position: generate_spiral_position(1, 5.0, 1.0),
        scale: 1.5,
        rotation: Vec3::new(0.0, 0.0, 0.0), // Inicializar rotación
        rotation_speed: Vec3::new(0.0, 0.4, 0.0), // Rotación lenta en el eje Y
        collision_radius: 3.5,
    },
    Model {
        vertex_array: &sphere_vertices,
        shader: PlanetShader::Oceanic,
        position: generate_spiral_position(2, 5.0, 1.0),
        scale: 1.0,
        rotation: Vec3::new(0.0, 0.0, 0.0), // Inicializar rotación
        rotation_speed: Vec3::new(0.0, 0.5, 0.0), // Rotación lenta en el eje Y
        collision_radius: 2.5,
    },
    Model {
        vertex_array: &sphere_vertices,
        shader: PlanetShader::Earth,
        position: generate_spiral_position(3, 5.0, 1.0),
        scale: 1.5,
        rotation: Vec3::new(0.0, 0.0, 0.0), // Inicializar rotación
        rotation_speed: Vec3::new(0.0, 0.8, 0.0), // Rotación lenta en el eje Y
        collision_radius: 3.5,
    },
    Model {
        vertex_array: &sphere_vertices,
        shader: PlanetShader::Frozen,
        position: generate_spiral_position(4, 5.0, 1.0),
        scale: 1.5,
        rotation: Vec3::new(0.0, 0.0, 0.0), // Inicializar rotación
        rotation_speed: Vec3::new(0.0, 0.13, 0.0), // Rotación lenta en el eje Y
        collision_radius: 3.5,
    },
    Model {
        vertex_array: &sphere_vertices,
        shader: PlanetShader::Gaseous,
        position: generate_spiral_position(5, 5.0, 1.0),
        scale: 2.0,
        rotation: Vec3::new(0.0, 0.0, 0.0), // Inicializar rotación
        rotation_speed: Vec3::new(0.0, 0.21, 0.0), // Rotación lenta en el eje Y
        collision_radius: 4.0,
    },
    Model {
        vertex_array: &ufo_vertices,
        shader: PlanetShader::Ufo,
        position: generate_spiral_position(6, 5.0, 1.0),
        scale: 0.005,
        rotation: Vec3::new(0.0, 0.0, 0.0), // Inicializar rotación
        rotation_speed: Vec3::new(0.0, 0.8, 0.0), // Rotación lenta en el eje Y
        collision_radius: 0.05,
    },
    Model {
        vertex_array: &eye_vertices,
        shader: PlanetShader::Gargantua,
        position: generate_spiral_position(7, 5.0, 1.0),
        scale: 2.0,
        rotation: Vec3::new(0.0, 0.0, 0.0), // Inicializar rotación
        rotation_speed: Vec3::new(0.0, 0.0, 0.0), // Rotación lenta en el eje Y
        collision_radius: 4.0,
    },
    Model {
        vertex_array: &spaceship_vertices,
        shader: PlanetShader::Ufo,
        position: camera.eye + camera.get_forward_vector() * 4.0,
        scale: 0.02,
        rotation: Vec3::new(0.0, 0.0, 0.0), // Inicializar rotación
        rotation_speed: Vec3::new(0.0, 0.0, 0.0), // Rotación lenta en el eje Y
        collision_radius: 8.0,
    },
    ];

    let mut time_counter = 0;
    let mut current_planet_shader = PlanetShader::Rocky;


    // Inicializar el nivel de emision
    let mut emission_intensity = 1.0;

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }
    
        let (rest_models, spaceship_model) = models.split_last_mut().unwrap();
        handle_input(&window, &mut camera, rest_models, spaceship_model);
        
        handle_key_input(&window, &mut camera, &mut models);
    
        framebuffer.clear();
    
        // Actualizar la posición de la nave para que esté siempre frente a la cámara
        for model in &mut models {
            // Si es un UFO, seguimos controlando su posición y rotación usando la cámara
            if let PlanetShader::Ufo = model.shader {
                model.position = camera.eye + camera.get_forward_vector() * 2.5;
                model.rotation = Vec3::new(camera.pitch, camera.yaw, 0.0);
            } else {
                // Para otros modelos, actualizamos la rotación usando `rotation_speed`
                model.rotation.x += model.rotation_speed.x * time_counter as f32 * 0.001;
                model.rotation.y += model.rotation_speed.y * time_counter as f32 * 0.001;
                model.rotation.z += model.rotation_speed.z * time_counter as f32 * 0.001;
            }
        }
        
        
    
        // Crear uniforms antes de renderizar
        let view_matrix = camera.get_view_matrix();
        let projection_matrix = create_projection_matrix(window_width as f32, window_height as f32);
        let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);
    
        time_counter += 1;
    
        let uniforms = Uniforms {
            view_matrix,
            projection_matrix,
            viewport_matrix,
            model_matrix: Mat4::identity(),
            transformation_matrix: Mat4::identity(),
            normal_matrix: Mat3::identity(),
            time: time_counter,
            noise: &noise,
            emission_intensity,
        };
    
        // Renderizar el skybox primero
        skybox.render(&mut framebuffer, &uniforms, camera.eye);

        render_orbits(&mut framebuffer, &models, &view_matrix, &projection_matrix);

    
        // Iterar sobre la lista de modelos y renderizar cada uno
        for model in &models {
            // Crear la matriz de modelo para este modelo
            let model_matrix = create_model_matrix_with_rotation(model.position, model.scale, model.rotation);
            let transformation_matrix = uniforms.projection_matrix * uniforms.view_matrix * model_matrix;
            let normal_matrix = model_matrix.fixed_resize::<3, 3>(0.0).try_inverse().unwrap().transpose();
            
    
            let model_uniforms = Uniforms {
                model_matrix,
                view_matrix: uniforms.view_matrix,
                projection_matrix: uniforms.projection_matrix,
                viewport_matrix: uniforms.viewport_matrix,
                transformation_matrix,
                normal_matrix,
                time: uniforms.time,
                noise: uniforms.noise,
                emission_intensity: uniforms.emission_intensity,
            };
    
            render(&mut framebuffer, &model_uniforms, model.vertex_array, &model.shader);
        }
    
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();
    
        std::thread::sleep(frame_delay);
    }

}

// Función para manejar la selección de shaders y modelos de planetas.
fn handle_key_input(window: &Window, camera: &mut Camera, models: &mut Vec<Model>) {
    if window.is_key_down(Key::Key1) {
        warp_to_planet(camera, &models[1]); // Rocky
    }
    if window.is_key_down(Key::Key2) {
        warp_to_planet(camera, &models[2]); // Gaseous
    }
    if window.is_key_down(Key::Key3) {
        warp_to_planet(camera, &models[3]); // Frozen
    }
    if window.is_key_down(Key::Key4) {
        warp_to_planet(camera, &models[4]); // Earth
    }
    if window.is_key_down(Key::Key5) {
        warp_to_planet(camera, &models[5]); // Oceanic
    }
    if window.is_key_down(Key::Key6) {
        warp_to_planet(camera, &models[6]); // UFO
    }
    if window.is_key_down(Key::Key7) {
        warp_to_planet(camera, &models[7]); // Gargantua
    }
    if window.is_key_down(Key::Key8) {
        warp_to_planet(camera, &models[0]); // Wormhole (al principio de la lista)
    }
}

fn handle_input(window: &Window, camera: &mut Camera, spaceship_model: &mut Model, models: &mut [Model]) {
    let orbit_speed = PI / 50.0;
    let zoom_speed = 0.5;
    let mut new_position = spaceship_model.position;

    // Cambio a Bird Eye View al presionar la tecla B
    // Cambio a Bird Eye View al presionar la tecla B
    if window.is_key_down(Key::B) {
        camera.toggle_bird_eye_view();
    }
    


    // Movimiento con flechas para orbitar la cámara
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

    // Zoom con teclas W y S
    if window.is_key_down(Key::W) {
        camera.zoom(-zoom_speed);
    }
    if window.is_key_down(Key::S) {
        camera.zoom(zoom_speed);
    }

    // Verificar colisiones antes de actualizar la posición
    let mut collision_detected = false;
    for model in models {
        if model.shader != PlanetShader::Ufo && check_collision(&spaceship_model, &model) {
            collision_detected = true;
            break;
        }
    }

    // Manejar la posición según colisión
    if !collision_detected {
        spaceship_model.position = new_position;
    } else {
        let max_retroceso = 5.0;
        let backward_vector = -camera.get_forward_vector() * 2.5;
        let nueva_posicion = spaceship_model.position + backward_vector;

        if nalgebra_glm::distance(&spaceship_model.position, &nueva_posicion) < max_retroceso {
            spaceship_model.position = nueva_posicion;
        }
    }

    // Actualizar el centro de la cámara para que apunte siempre hacia la nave si no está en Bird Eye View
    if !camera.is_bird_eye_view {
        camera.center = spaceship_model.position;
    }

    // Calcular la rotación de la nave basada en la orientación de la cámara
    let forward_vector = camera.get_forward_vector();
    let pitch = forward_vector.y.atan2((forward_vector.x.powi(2) + forward_vector.z.powi(2)).sqrt());
    let yaw = forward_vector.z.atan2(forward_vector.x);

    spaceship_model.rotation = Vec3::new(pitch, yaw, 0.0);
}


