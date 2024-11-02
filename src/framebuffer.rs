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

    // pub fn clear(&mut self) {
    //     for pixel in self.buffer.iter_mut() {
    //         *pixel = self.background_color;
    //     }
    //     for depth in self.zbuffer.iter_mut() {
    //         *depth = f32::INFINITY;
    //     }
    //     for emission in self.emission_buffer.iter_mut() { // Limpiar el buffer de emisión
    //         *emission = 0;
    //     }
    // }

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

     // Función para aplicar el post-procesamiento de emisión
     pub fn apply_emission(&mut self) {
        let blur_radius = 5; // Radio de desenfoque
        let mut temp_emission_buffer = self.emission_buffer.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let mut blended_color = 0u32;
                let mut count = 0;

                // Aplicar desenfoque en un área de radio `blur_radius` alrededor de cada píxel emisivo
                for dy in -(blur_radius as i32)..=(blur_radius as i32) {
                    for dx in -(blur_radius as i32)..=(blur_radius as i32) {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        if nx >= 0 && ny >= 0 && nx < self.width as i32 && ny < self.height as i32 {
                            let index = ny as usize * self.width + nx as usize;
                            blended_color += self.emission_buffer[index];
                            count += 1;
                        }
                    }
                }

                if count > 0 {
                    blended_color /= count; // Promediar el color de emisión
                }

                // Mezclar el color de emisión en el buffer principal
                let index = y * self.width + x;
                self.buffer[index] = self.combine_colors(self.buffer[index], blended_color);
            }
        }
    }

    fn combine_colors(&self, base_color: u32, emission_color: u32) -> u32 {
        // Combinar base y emisión para aplicar el efecto de luz
        // Aquí puedes ajustar cómo se mezclan, tal vez utilizando el modo "additive blend"
        base_color.saturating_add(emission_color)
    }
}
    

