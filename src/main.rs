mod framebuffer;
mod color;
mod vertex;
mod fragment;
mod line;
mod triangle;

use framebuffer::Framebuffer;
use color::Color;
use vertex::Vertex;
use nalgebra_glm::Vec2;
use nalgebra_glm::Vec3;
use triangle::triangle;

fn main() {
    // Tamaño del framebuffer (ventana)
    let width = 800;
    let height = 600;

    // Crear una instancia del framebuffer
    let mut framebuffer = Framebuffer::new(width, height);

    // Establecer un color de fondo (por ejemplo, negro)
    let background_color = Color::new(0, 0, 20);
    framebuffer.clear(background_color);

    // Definir tres vértices para dibujar un triángulo
    let vertex_a = Vertex::new_with_color(Vec3::new(100.0, 100.0, 0.0), Color::new(255, 0, 0)); // Rojo
    let vertex_b = Vertex::new_with_color(Vec3::new(400.0, 300.0, 0.0), Color::new(0, 255, 0)); // Verde
    let vertex_c = Vertex::new_with_color(Vec3::new(200.0, 500.0, 0.0), Color::new(0, 0, 255)); // Azul

    // Dibujar el triángulo entre los vértices
    let fragments = triangle(&vertex_a, &vertex_b, &vertex_c);

    // Dibujar cada fragmento en el framebuffer
    for fragment in fragments {
        framebuffer.set_current_color(fragment.color);
        framebuffer.point(fragment.position.x as isize, fragment.position.y as isize);
    }

    // Renderizar la ventana con el contenido del framebuffer
    framebuffer.render_window();
}
