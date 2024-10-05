use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    // Constructor que recibe valores RGB
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    // Constructor que recibe un valor HEX
    pub fn from_hex(hex: &str) -> Result<Color, &'static str> {
        if hex.len() != 6 {
            return Err("Hex color must be 6 characters long.");
        }

        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid hex value")?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid hex value")?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid hex value")?;

        Ok(Color { r, g, b })
    }

    // Método para crear un color negro
    pub fn black() -> Color {
        Color { r: 0, g: 0, b: 0 }
    }

    // Clamping de valores RGB entre 0 y 255
    fn clamp(value: i32) -> u8 {
        if value < 0 {
            0
        } else if value > 255 {
            255
        } else {
            value as u8
        }
    }

    // Sumar dos colores sin sobrepasar el valor de 255
    pub fn add(&self, other: &Color) -> Color {
        Color {
            r: Color::clamp(self.r as i32 + other.r as i32),
            g: Color::clamp(self.g as i32 + other.g as i32),
            b: Color::clamp(self.b as i32 + other.b as i32),
        }
    }

    // Multiplicar un color por un número
    pub fn multiply(&self, scalar: f32) -> Color {
        Color {
            r: Color::clamp((self.r as f32 * scalar) as i32),
            g: Color::clamp((self.g as f32 * scalar) as i32),
            b: Color::clamp((self.b as f32 * scalar) as i32),
        }
    }
}

// Implementar el trait Display para la estructura Color
use std::fmt;
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color(r: {}, g: {}, b: {})", self.r, self.g, self.b)
    }
}

// Implementación del operador Mul para Color
impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, scalar: f32) -> Color {
        Color {
            r: Color::clamp((self.r as f32 * scalar) as i32),
            g: Color::clamp((self.g as f32 * scalar) as i32),
            b: Color::clamp((self.b as f32 * scalar) as i32),
        }
    }
}

// Implementación del operador Add para Color
impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: Color::clamp(self.r as i32 + other.r as i32),
            g: Color::clamp(self.g as i32 + other.g as i32),
            b: Color::clamp(self.b as i32 + other.b as i32),
        }
    }
}