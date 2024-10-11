use crate::color::Color;
use crate::vertex::Vertex;
use crate::fragment::Fragment;
use crate::shader::vertex_shader;
use crate::uniforms::Uniforms;
use crate::line::line;

use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::{Vec2, Vec3};


// Framebuffer para gestionar el buffer de píxeles
pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
            current_color: 0,
        }
    }

    // Método para limpiar el framebuffer con un color de fondo
    pub fn clear(&mut self, color: Color) {
        let color_u32 = (255 << 24) | ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32);
        for pixel in self.buffer.iter_mut() {
            *pixel = color_u32;
        }
    }

    // Método para dibujar un punto en el framebuffer
    pub fn point(&mut self, x: isize, y: isize) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            let index = (y as usize) * self.width + (x as usize);
            self.buffer[index] = self.current_color;
        }
    }

    // Método para establecer el color actual
    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = (255 << 24) | ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32);
    }

    // Método para renderizar la ventana utilizando minifb
    pub fn render_window(&self) {
        let mut window = Window::new(
            "Framebuffer Example",
            self.width,
            self.height,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        // Mientras la ventana esté abierta y no se presione la tecla ESC
        while window.is_open() && !window.is_key_down(Key::Escape) {
            window.update_with_buffer(&self.buffer, self.width, self.height).unwrap();
        }
    }
}

// Cálculo del Bounding Box que contiene el triángulo
fn calculate_bounding_box(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> (i32, i32, i32, i32) {
    let min_x = v1.x.min(v2.x).min(v3.x).floor() as i32;
    let min_y = v1.y.min(v2.y).min(v3.y).floor() as i32;
    let max_x = v1.x.max(v2.x).max(v3.x).ceil() as i32;
    let max_y = v1.y.max(v2.y).max(v3.y).ceil() as i32;
    (min_x, min_y, max_x, max_y)
}

// Coordenadas baricéntricas para un punto en el triángulo
fn barycentric_coordinates(p: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3) -> (f32, f32, f32) {
    let v0 = b - a;  // Vector AB
    let v1 = c - a;  // Vector AC
    let v2 = p - a;  // Vector AP

    let d00 = v0.dot(&v0);
    let d01 = v0.dot(&v1);
    let d11 = v1.dot(&v1);
    let d20 = v2.dot(&v0);
    let d21 = v2.dot(&v1);

    let denom = d00 * d11 - d01 * d01;
    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = 1.0 - v - w;

    (u, v, w)
}

// Rasterización de triángulos usando Bounding Box y las coordenadas baricéntricas
pub fn primitive_assembly_rasterization(vertex_array: &[Vertex]) -> Vec<Fragment> {
    let mut fragments: Vec<Fragment> = Vec::new();

    // Recorrer el vertex_array en grupos de 3 (triángulos)
    for triangle_vertices in vertex_array.chunks(3) {
        if triangle_vertices.len() == 3 {
            let v0 = &triangle_vertices[0];
            let v1 = &triangle_vertices[1];
            let v2 = &triangle_vertices[2];

            // Calcular el Bounding Box del triángulo
            let (min_x, min_y, max_x, max_y) = calculate_bounding_box(
                &v0.transformed_position,
                &v1.transformed_position,
                &v2.transformed_position,
            );

            // Restringimos la rasterización al área dentro del Bounding Box
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    let p = Vec3::new(x as f32, y as f32, 0.0);

                    // Obtener coordenadas baricéntricas
                    let (u, v, w) = barycentric_coordinates(&p, &v0.transformed_position, &v1.transformed_position, &v2.transformed_position);

                    // Verificar si el punto está dentro del triángulo
                    if u >= 0.0 && v >= 0.0 && w >= 0.0 {
                        // Interpolar color usando las coordenadas baricéntricas
                        let r = (u * v0.color.r as f32 + v * v1.color.r as f32 + w * v2.color.r as f32) as u8;
                        let g = (u * v0.color.g as f32 + v * v1.color.g as f32 + w * v2.color.g as f32) as u8;
                        let b = (u * v0.color.b as f32 + v * v1.color.b as f32 + w * v2.color.b as f32) as u8;

                        // Crear un fragmento interpolado
                        let fragment = Fragment {
                            position: Vec2::new(x as f32, y as f32),
                            color: Color { r, g, b },
                            depth: u * v0.transformed_position.z + v * v1.transformed_position.z + w * v2.transformed_position.z,
                        };

                        fragments.push(fragment);
                    }
                }
            }
        }
    }

    fragments
}

// Solo la etapa de Fragment Processing con el Vertex Shader y Rasterización

pub fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], indices: &[u32]) {
    // Vertex Shader Stage: Aplicar transformaciones a los vértices
    let transformed_vertices: Vec<Vertex> = vertex_array
        .iter()
        .map(|vertex| vertex_shader(vertex, uniforms))
        .collect();

    // Primero rasterizar los triángulos (relleno)
    let fragments = primitive_assembly_rasterization(&transformed_vertices);

    // Dibujar los fragmentos en el framebuffer (triángulos)
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        framebuffer.set_current_color(fragment.color);
        framebuffer.point(x as isize, y as isize);
    }

    // Ahora dibujar el wireframe (líneas)
    for chunk in indices.chunks(3) {
        let v0 = &transformed_vertices[chunk[0] as usize];
        let v1 = &transformed_vertices[chunk[1] as usize];
        let v2 = &transformed_vertices[chunk[2] as usize];

        // Cambiar el color para las líneas del wireframe
        framebuffer.set_current_color(Color::new(0, 0, 0));  // Color negro para el wireframe

        // Dibuja las líneas entre los vértices del triángulo
        let fragments_v0_v1 = line(v0, v1);
        let fragments_v1_v2 = line(v1, v2);
        let fragments_v2_v0 = line(v2, v0);

        // Dibujar los fragmentos de cada línea en el framebuffer (wireframe)
        for fragment in fragments_v0_v1.iter().chain(fragments_v1_v2.iter()).chain(fragments_v2_v0.iter()) {
            let x = fragment.position.x as usize;
            let y = fragment.position.y as usize;

            framebuffer.set_current_color(fragment.color);
            framebuffer.point(x as isize, y as isize);
        }
    }
}

