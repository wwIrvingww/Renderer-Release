// framebuffer.rs

// Framebuffer para gestionar el buffer de píxeles
pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub zbuffer: Vec<f32>,
    pub emission_buffer: Vec<u32>, // Nuevo buffer para el color de emisión
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            zbuffer: vec![f32::INFINITY; width * height],
            emission_buffer: vec![0; width * height], // Inicialización del buffer de emisión
            background_color: 0x000000,
            current_color: 0xFFFFFF,
        }
    }

    // Función para limpiar el framebuffer
    pub fn clear(&mut self) {
        self.buffer.fill(self.background_color);
        self.zbuffer.fill(f32::INFINITY);
        self.emission_buffer.fill(0);  // Limpiar el buffer de emisión
    }

    pub fn point(&mut self, x: usize, y: usize, depth: f32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            if self.zbuffer[index] > depth {
                self.buffer[index] = self.current_color;
                self.zbuffer[index] = depth;
            }
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    // Establece el color de emisión en el buffer de emisión
    pub fn set_emission_color(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.emission_buffer[index] = color;
        }
    }

    pub fn point_emission(&mut self, x: usize, y: usize, depth: f32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            if self.zbuffer[index] > depth {
                self.emission_buffer[index] = self.current_color;
                self.zbuffer[index] = depth;
            }
        }
    }

    pub fn apply_emission(&mut self) {
        let blur_radius = 2; // Reducir el radio para mejorar rendimiento
        let emission_intensity = 0.5; // Factor de intensidad para suavizar el efecto
        let mut temp_emission_buffer = self.emission_buffer.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let mut blended_color = 0u32;
                let mut count = 0;

                // Aplicar desenfoque simple en un área reducida
                for dy in -(blur_radius as i32)..=(blur_radius as i32) {
                    for dx in -(blur_radius as i32)..=(blur_radius as i32) {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        if nx >= 0 && ny >= 0 && nx < self.width as i32 && ny < self.height as i32 {
                            let index = ny as usize * self.width + nx as usize;
                            blended_color = blended_color.saturating_add(self.emission_buffer[index]);
                            count += 1;
                        }
                    }
                }

                if count > 0 {
                    blended_color /= count; // Promediar el color de emisión
                }

                // Mezclar con el buffer principal usando intensidad ajustada
                let index = y * self.width + x;
                let emission_color = (blended_color as f32 * emission_intensity) as u32;
                self.buffer[index] = self.combine_colors(self.buffer[index], emission_color);
            }
        }

        // Limpiar el buffer de emisión para el próximo frame
        self.emission_buffer.fill(0);
    }

    fn combine_colors(&self, base_color: u32, emission_color: u32) -> u32 {
        // Mezcla aditiva con saturación para evitar sobresaturar el color
        let base_r = (base_color >> 16) & 0xFF;
        let base_g = (base_color >> 8) & 0xFF;
        let base_b = base_color & 0xFF;

        let emission_r = (emission_color >> 16) & 0xFF;
        let emission_g = (emission_color >> 8) & 0xFF;
        let emission_b = emission_color & 0xFF;

        let r = (base_r + emission_r).min(255);
        let g = (base_g + emission_g).min(255);
        let b = (base_b + emission_b).min(255);

        (r << 16) | (g << 8) | b
    }

    /// Dibuja una línea entre dos puntos (x0, y0) y (x1, y1) usando el algoritmo de Bresenham.
    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: u32) {
        let mut x0 = x0 as i32;
        let mut y0 = y0 as i32;
        let x1 = x1 as i32;
        let y1 = y1 as i32;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        while x0 != x1 || y0 != y1 {
            if x0 >= 0 && y0 >= 0 && x0 < self.width as i32 && y0 < self.height as i32 {
                self.buffer[(y0 as usize) * self.width + (x0 as usize)] = color;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x0 += sx;
            }
            if e2 < dx {
                err += dx;
                y0 += sy;
            }
        }
    }
}
    

