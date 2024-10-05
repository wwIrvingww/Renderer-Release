mod framebuffer;
mod color;

use framebuffer::Framebuffer;
use color::Color;

fn main() {
    // Tamaño del framebuffer (ventana)
    let width = 800;
    let height = 600;

    // Crear una instancia del framebuffer
    let mut framebuffer = Framebuffer::new(width, height);

    // Establecer un color de fondo (por ejemplo, negro)
    let background_color = Color::new(0, 0, 20);
    framebuffer.clear(background_color);

    // Establecer el color actual para dibujar (por ejemplo, rojo)
    let draw_color = Color::new(255, 0, 0);
    framebuffer.set_current_color(draw_color);

    // Dibujar algunos puntos en el framebuffer
    for x in 100..200 {
        for y in 100..200 {
            framebuffer.point(x, y); // Dibujar un cuadrado rojo
        }
    }

    // Renderizar la ventana con el contenido del framebuffer
    framebuffer.render_window();
}
