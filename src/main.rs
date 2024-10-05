mod framebuffer;
mod color;
mod vertex;
mod fragment;
mod line;
mod triangle;
mod obj;

use framebuffer::Framebuffer;
use color::Color;
use vertex::Vertex;
use nalgebra_glm::Vec2;
use nalgebra_glm::Vec3;
use obj::Obj;

pub struct Uniforms {
    
}

fn main() {
    // Tamaño del framebuffer (ventana)
    let width = 800;
    let height = 600;

    // Crear una instancia del framebuffer
    let mut framebuffer = Framebuffer::new(width, height);

    // Establecer un color de fondo (por ejemplo, negro)
    let background_color = Color::new(0, 0, 20);
    framebuffer.clear(background_color);

    // Cargar el archivo OBJ
    let obj_model = Obj::load("assets/cube.obj").expect("Error cargando el archivo OBJ");

    // Obtener el array de vértices
    let vertices = obj_model.get_vertex_array();

    // Definir un factor de escala para asegurar que los vértices estén dentro de la ventana
    let scale_factor = 100.0;
    let offset_x = width as f32 / 2.0;
    let offset_y = height as f32 / 2.0;

    // Dibujar los vértices obtenidos del archivo OBJ
    for vertex in vertices {
        let x = vertex.position.x * scale_factor + offset_x;
        let y = vertex.position.y * scale_factor + offset_y;

        // Nos aseguramos de que los vértices están dentro de los límites de la ventana antes de dibujarlos
        if x >= 0.0 && x < width as f32 && y >= 0.0 && y < height as f32 {
            framebuffer.set_current_color(vertex.color);  // Usamos el color del vértice
            framebuffer.point(x as isize, y as isize);  // Dibujamos el vértice
        }
    }

    // Renderizar la ventana con el contenido del framebuffer
    framebuffer.render_window();
}

