use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,  // Ahora los campos son públicos
    pub g: u8,  // Ahora los campos son públicos
    pub b: u8,  // Ahora los campos son públicos
}

impl Color {
    // Constructor to initialize the color using r, g, b values as u8
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    // Default black color
    pub fn black() -> Self {
        Color { r: 0, g: 0, b: 0 }
    }

    // New constructor to initialize the color using r, g, b values as f32 (0.0 to 1.0)
    pub fn from_float(r: f32, g: f32, b: f32) -> Self {
        Color {
            r: (r.clamp(0.0, 1.0) * 255.0) as u8,
            g: (g.clamp(0.0, 1.0) * 255.0) as u8,
            b: (b.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }

    // Function to create a color from a hex value
    pub fn from_hex(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Color { r, g, b }
    }

    // Linear interpolation between two colors
    pub fn lerp(&self, other: &Color, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Color {
            r: (self.r as f32 + (other.r as f32 - self.r as f32) * t).round() as u8,
            g: (self.g as f32 + (other.g as f32 - self.g as f32) * t).round() as u8,
            b: (self.b as f32 + (other.b as f32 - self.b as f32) * t).round() as u8,
        }
    }

    // Function to return the color as a hex value
    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    // Check if the color is black
    pub fn is_black(&self) -> bool {
        self.r == 0 && self.g == 0 && self.b == 0
    }

    // Normal blend mode: Use the blend color if it's not black
    pub fn blend_normal(&self, blend: &Color) -> Color {
        if blend.is_black() { *self } else { *blend }
    }

    // Multiply blend mode
    pub fn blend_multiply(&self, blend: &Color) -> Color {
        Color::new(
            ((self.r as f32 * blend.r as f32) / 255.0) as u8,
            ((self.g as f32 * blend.g as f32) / 255.0) as u8,
            ((self.b as f32 * blend.b as f32) / 255.0) as u8,
        )
    }

    // Add blend mode
    pub fn blend_add(&self, blend: &Color) -> Color {
        Color::new(
            (self.r as u16 + blend.r as u16).min(255) as u8,
            (self.g as u16 + blend.g as u16).min(255) as u8,
            (self.b as u16 + blend.b as u16).min(255) as u8,
        )
    }

    // Subtract blend mode
    pub fn blend_subtract(&self, blend: &Color) -> Color {
        if blend.is_black() {
            *self
        } else {
            Color::new(
                self.r.saturating_sub(blend.r),
                self.g.saturating_sub(blend.g),
                self.b.saturating_sub(blend.b),
            )
        }
    }

    // Screen blend mode
    pub fn blend_screen(&self, blend: &Color) -> Color {
        Color::new(
            (255 - ((255 - self.r) as u16 * (255 - blend.r) as u16) / 255) as u8,
            (255 - ((255 - self.g) as u16 * (255 - blend.g) as u16) / 255) as u8,
            (255 - ((255 - self.b) as u16 * (255 - blend.b) as u16) / 255) as u8,
        )
    }

    // Overlay blend mode
    pub fn blend_overlay(&self, blend: &Color) -> Color {
        Color::new(
            if self.r < 128 {
                (2 * self.r as u16 * blend.r as u16 / 255) as u8
            } else {
                (255 - 2 * (255 - self.r as u16) * (255 - blend.r as u16) / 255) as u8
            },
            if self.g < 128 {
                (2 * self.g as u16 * blend.g as u16 / 255) as u8
            } else {
                (255 - 2 * (255 - self.g as u16) * (255 - blend.g as u16) / 255) as u8
            },
            if self.b < 128 {
                (2 * self.b as u16 * blend.b as u16 / 255) as u8
            } else {
                (255 - 2 * (255 - self.b as u16) * (255 - blend.b as u16) / 255) as u8
            }
        )
    }
}

// Implement addition for Color
use std::ops::Add;

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r.saturating_add(other.r),
            g: self.g.saturating_add(other.g),
            b: self.b.saturating_add(other.b),
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

// Implement multiplication by a constant for Color
use std::ops::Mul;

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, scalar: f32) -> Color {
        Color {
            r: (self.r as f32 * scalar).clamp(0.0, 255.0) as u8,
            g: (self.g as f32 * scalar).clamp(0.0, 255.0) as u8,
            b: (self.b as f32 * scalar).clamp(0.0, 255.0) as u8,
        }
    }
}

// Implement display formatting for Color
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color(r: {}, g: {}, b: {})", self.r, self.g, self.b)
    }
}
