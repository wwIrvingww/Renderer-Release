use crate::color::Color;
use crate::vertex::Vertex;
use crate::fragment::Fragment;
use crate::shader::vertex_shader;
use crate::triangle::triangle;
use crate::uniforms::Uniforms;
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

// Rasterización de triángulos usando Bounding Box y la función triangle
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
                    // Verificar si el punto (x, y) está dentro del triángulo
                    let p = Vec2::new(x as f32, y as f32);

                    // Verificar si el punto está dentro del triángulo
                    if is_point_inside_triangle(&v0.transformed_position, &v1.transformed_position, &v2.transformed_position, &p) {
                        // Llamar a la función triangle para rasterizar el triángulo
                        fragments.extend(triangle(v0, v1, v2));
                    }
                }
            }
        }
    }

    fragments
}

// Verificar si un punto está dentro del triángulo usando el algoritmo baricéntrico
fn is_point_inside_triangle(v0: &Vec3, v1: &Vec3, v2: &Vec3, p: &Vec2) -> bool {
    let p0 = Vec2::new(v0.x, v0.y);
    let p1 = Vec2::new(v1.x, v1.y);
    let p2 = Vec2::new(v2.x, v2.y);

    let area = edge_function(&p0, &p1, &p2);
    let w0 = edge_function(&p1, &p2, p);
    let w1 = edge_function(&p2, &p0, p);
    let w2 = edge_function(&p0, &p1, p);

    w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 && (w0 + w1 + w2).abs() <= area.abs()
}

// Calcula el área de un triángulo para el algoritmo baricéntrico
fn edge_function(v0: &Vec2, v1: &Vec2, p: &Vec2) -> f32 {
    (p.x - v0.x) * (v1.y - v0.y) - (p.y - v0.y) * (v1.x - v0.x)
}

// Solo la etapa de Fragment Processing con el Vertex Shader y Primitive Assembly
pub fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    let mut fragments: Vec<Fragment> = Vec::new();

    // Vertex Shader Stage: Aplicar transformaciones a los vértices
    let mut transformed_vertices = Vec::new();
    for vertex in vertex_array {
        let transformed_vertex = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed_vertex);
    }

    // Primitive Assembly y Rasterización
    fragments = primitive_assembly_rasterization(&transformed_vertices);

    // Fragment Processing Stage: dibujar los fragmentos en el framebuffer
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        framebuffer.set_current_color(fragment.color);
        framebuffer.point(x as isize, y as isize);
    }
}
