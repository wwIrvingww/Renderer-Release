// fragment_shader.rs
use crate::fragment::Fragment;

pub fn fragment_shader(fragment: &Fragment) -> Fragment {
    // Aquí puedes modificar el color del fragmento, por ejemplo
    // aplicando una simple reducción de brillo basada en la profundidad.

    let brightness_factor = 1.0 - (fragment.depth * 0.5); // Ejemplo de ajuste por profundidad
    let brightness_factor = brightness_factor.clamp(0.0, 1.0);

    let new_color = crate::color::Color {
        r: (fragment.color.r as f32 * brightness_factor) as u8,
        g: (fragment.color.g as f32 * brightness_factor) as u8,
        b: (fragment.color.b as f32 * brightness_factor) as u8,
    };

    Fragment {
        position: fragment.position,
        color: new_color,
        depth: fragment.depth,
    }
}
