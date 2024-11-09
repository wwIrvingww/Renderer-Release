use nalgebra_glm::{Vec2, Vec3};
use crate::color::Color;

#[derive(Clone, Debug)]
pub struct Fragment {
    pub position: Vec2,
    pub color: Color,
    pub depth: f32,
    pub normal: Vec3,
    pub intensity: f32,
    pub vertex_position: Vec3,
    pub tex_coords: Vec2, // Añadido: Coordenadas de textura (UV)
}

impl Fragment {
    pub fn new(
        position: Vec2,
        color: Color,
        depth: f32,
        normal: Vec3,
        intensity: f32,
        vertex_position: Vec3,
        tex_coords: Vec2,
    ) -> Self {
        Fragment {
            position,
            color,
            depth,
            normal,
            intensity,
            vertex_position,
            tex_coords,
        }
    }
}
