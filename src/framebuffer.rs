use crate::color::Color;
use crate::vertex::Vertex;
use crate::fragment::Fragment;
use crate::shader::vertex_shader;
use crate::uniforms::Uniforms; // Asegúrate de importar Uniforms desde uniforms.rs
use minifb::{Window, WindowOptions, Key};

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
    } // <- Cierre de la función point

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

// Solo la etapa de Fragment Processing con el Vertex Shader
pub fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    let mut fragments: Vec<Fragment> = Vec::new();

    // Vertex Shader Stage: Aplicar transformaciones a los vértices
    let mut transformed_vertices = Vec::new();
    for vertex in vertex_array {
        let transformed_vertex = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed_vertex);
    }

    // Fragment Processing Stage: dibujar los fragmentos en el framebuffer
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        framebuffer.set_current_color(fragment.color);
        framebuffer.point(x as isize, y as isize);
    }
}
