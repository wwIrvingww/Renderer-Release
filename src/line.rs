use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::color::Color;
use nalgebra_glm::{Vec3, Vec2}; // Importamos Vec3 para las normales

pub fn line(a: &Vertex, b: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    let start = a.transformed_position;
    let end = b.transformed_position;

    let mut x0 = start.x as i32;
    let mut y0 = start.y as i32;
    let x1 = end.x as i32;
    let y1 = end.y as i32;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = if dx > dy { dx / 2 } else { -dy / 2 };

    loop {
        let t = if dx > dy {
            (x0 - start.x as i32) as f32 / (x1 - start.x as i32) as f32
        } else {
            (y0 - start.y as i32) as f32 / (y1 - start.y as i32) as f32
        };

        let z = start.z + (end.z - start.z) * t;
        let tex_coords = a.tex_coords + (b.tex_coords - a.tex_coords) * t;

        // Normal y color predeterminados
        let default_normal = Vec3::new(0.0, 0.0, 1.0);
        let intensity = 1.0;

        fragments.push(Fragment::new(
            Vec2::new(x0 as f32, y0 as f32),        // position
            Color::new(255, 255, 255),             // color
            z,                                     // depth
            default_normal,                        // normal
            intensity,                             // intensity
            Vec3::new(x0 as f32, y0 as f32, z),    // vertex_position (en 3D)
            tex_coords,                            // tex_coords
        ));

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = err;
        if e2 > -dx {
            err -= dy;
            x0 += sx;
        }
        if e2 < dy {
            err += dx;
            y0 += sy;
        }
    }

    fragments
}
