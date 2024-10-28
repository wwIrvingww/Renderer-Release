use crate::color::Color;
use crate::fragment::Fragment;
use crate::Uniforms;
use crate::shader::{depth_based_fragment_shader, noise_based_fragment_shader, moving_clouds_shader};

/// Primer shader de planeta: simula un planeta rocoso con textura granular
pub fn rocky_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Genera un color basado en el ruido para simular la textura de un planeta rocoso
    let base_color = Color::new(139, 69, 19); // Color marrón como base para el planeta rocoso
    let noise_color = noise_based_fragment_shader(fragment, uniforms); // Usa el shader de ruido existente

    // Interpolación entre el color base y el ruido para generar textura
    let rocky_color = base_color.lerp(&noise_color, 0.5);

    // Ajuste de profundidad para simular sombras y relieve en el planeta
    depth_based_fragment_shader(fragment, rocky_color)
}

/// Segundo shader de planeta: simula un planeta gaseoso con nubes en movimiento
pub fn gaseous_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Color base para el planeta, más azulado para un aspecto gaseoso
    let base_color = Color::new(0, 0, 139); // Azul oscuro como fondo del planeta
    let cloud_color = moving_clouds_shader(fragment, uniforms); // Usa el shader de nubes en movimiento

    // Mezcla entre el color base y las nubes para un efecto gaseoso
    let blended_color = base_color.lerp(&cloud_color, 0.7);

    // Aplicar un ajuste de profundidad para mejorar el aspecto de las nubes y el fondo gaseoso
    depth_based_fragment_shader(fragment, blended_color)
}
