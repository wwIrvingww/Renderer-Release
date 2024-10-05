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

    // Imprimir la cantidad de vértices y caras para verificar
    println!("Vertices: {:?}", obj_model.vertices);
    println!("Indices: {:?}", obj_model.indices);

    // Obtener el array de vértices
    let vertices = obj_model.get_vertex_array();

    // Dibujar los vértices obtenidos del archivo OBJ
    for vertex in vertices {
        framebuffer.set_current_color(vertex.color);  // Usamos el color del vértice
        framebuffer.point(vertex.position.x as isize, vertex.position.y as isize);  // Dibujamos el vértice
    }

    // Renderizar la ventana con el contenido del framebuffer
    framebuffer.render_window();
}

