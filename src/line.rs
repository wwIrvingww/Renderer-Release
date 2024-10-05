use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::color::Color;

pub fn line(a: &Vertex, b: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    let x0 = a.position.x as i32;
    let y0 = a.position.y as i32;
    let x1 = b.position.x as i32;
    let y1 = b.position.y as i32;

    // Diferencias y direcciones
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx - dy;

    let mut x = x0;
    let mut y = y0;

    let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

    while x != x1 || y != y1 {
        // Interpolar el color y la profundidad entre a y b
        let t = (((x - x0).pow(2) + (y - y0).pow(2)) as f32).sqrt() / distance;
        let color = interpolate_color(&a.color, &b.color, t);
        let depth = interpolate_f32(a.position.z, b.position.z, t);

        // Crear el fragmento
        fragments.push(Fragment {
            position: nalgebra_glm::Vec2::new(x as f32, y as f32),
            color,
            depth,
        });

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }

    // Asegurarse de incluir el último punto (b)
    fragments.push(Fragment {
        position: nalgebra_glm::Vec2::new(x1 as f32, y1 as f32),
        color: b.color,
        depth: b.position.z,
    });

    fragments
}

// Función para interpolar los colores entre dos vértices
fn interpolate_color(c1: &Color, c2: &Color, t: f32) -> Color {
    Color {
        r: ((1.0 - t) * c1.r as f32 + t * c2.r as f32) as u8,
        g: ((1.0 - t) * c1.g as f32 + t * c2.g as f32) as u8,
        b: ((1.0 - t) * c1.b as f32 + t * c2.b as f32) as u8,
    }
}

// Función para interpolar valores de profundidad (u otros valores flotantes)
fn interpolate_f32(a: f32, b: f32, t: f32) -> f32 {
    (1.0 - t) * a + t * b
}
