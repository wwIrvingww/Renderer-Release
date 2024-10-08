use crate::color::Color;
use crate::vertex::Vertex;
use crate::fragment::Fragment;
use crate::shader::vertex_shader;
use crate::uniforms::Uniforms;
use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::Vec2;

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

// Calcula el área del triángulo utilizando el producto cruzado
fn edge_function(v0: &Vec2, v1: &Vec2, p: &Vec2) -> f32 {
    (p.x - v0.x) * (v1.y - v0.y) - (p.y - v0.y) * (v1.x - v0.x)
}

// Rasterización de triángulos (Primitive Assembly y Rasterización)
pub fn primitive_assembly_rasterization(vertex_array: &[Vertex], framebuffer: &mut Framebuffer) -> Vec<Fragment> {
    let mut fragments: Vec<Fragment> = Vec::new();

    // Recorrer el vertex_array en grupos de 3 (triángulos)
    for triangle in vertex_array.chunks(3) {
        if triangle.len() == 3 {
            // Extraer los vértices del triángulo
            let v0 = &triangle[0];
            let v1 = &triangle[1];
            let v2 = &triangle[2];

            // Convertir las posiciones a 2D
            let p0 = Vec2::new(v0.transformed_position.x, v0.transformed_position.y);
            let p1 = Vec2::new(v1.transformed_position.x, v1.transformed_position.y);
            let p2 = Vec2::new(v2.transformed_position.x, v2.transformed_position.y);

            // Calcular los límites del triángulo para rasterización
            let min_x = p0.x.min(p1.x).min(p2.x).max(0.0) as isize;
            let max_x = p0.x.max(p1.x).max(p2.x).min(framebuffer.width as f32) as isize;
            let min_y = p0.y.min(p1.y).min(p2.y).max(0.0) as isize;
            let max_y = p0.y.max(p1.y).max(p2.y).min(framebuffer.height as f32) as isize;

            // Calcular el área total del triángulo
            let area = edge_function(&p0, &p1, &p2);

            // Recorrer todos los píxeles dentro del bounding box del triángulo
            for y in min_y..max_y {
                for x in min_x..max_x {
                    let p = Vec2::new(x as f32, y as f32);

                    // Calcular las coordenadas baricéntricas
                    let w0 = edge_function(&p1, &p2, &p);
                    let w1 = edge_function(&p2, &p0, &p);
                    let w2 = edge_function(&p0, &p1, &p);

                    // Si el punto está dentro del triángulo
                    if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                        // Interpolar el color de los vértices usando coordenadas baricéntricas
                        let total_area = area.abs();
                        let w0_norm = w0 / total_area;
                        let w1_norm = w1 / total_area;
                        let w2_norm = w2 / total_area;

                        // Interpolar color usando las coordenadas baricéntricas
                        let r = (w0_norm * v0.color.r as f32 + w1_norm * v1.color.r as f32 + w2_norm * v2.color.r as f32) as u8;
                        let g = (w0_norm * v0.color.g as f32 + w1_norm * v1.color.g as f32 + w2_norm * v2.color.g as f32) as u8;
                        let b = (w0_norm * v0.color.b as f32 + w1_norm * v1.color.b as f32 + w2_norm * v2.color.b as f32) as u8;

                        // Crear un fragmento interpolado
                        let fragment = Fragment {
                            position: p,
                            color: Color { r, g, b },
                            depth: w0_norm * v0.transformed_position.z + w1_norm * v1.transformed_position.z + w2_norm * v2.transformed_position.z,
                        };

                        // Guardar el fragmento
                        fragments.push(fragment);
                    }
                }
            }
        }
    }

    fragments
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
    fragments = primitive_assembly_rasterization(&transformed_vertices, framebuffer);

    // Fragment Processing Stage: dibujar los fragmentos en el framebuffer
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        framebuffer.set_current_color(fragment.color);
        framebuffer.point(x as isize, y as isize);
    }
}
