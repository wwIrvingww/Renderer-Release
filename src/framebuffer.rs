use crate::color::Color;
use minifb::{Window, WindowOptions, Key};

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>, // El buffer se representa como un vector de u32 para los colores
    current_color: u32,    // Agrega un campo para almacenar el color actual
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height], // Inicializar el buffer con color negro
            current_color: 0,                // Color inicial
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