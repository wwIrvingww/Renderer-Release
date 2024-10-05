use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::line::line;

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new(); // Corregido el nombre de la variable

    // Dibujar los bordes del triángulo
    fragments.extend(line(v1, v2));
    fragments.extend(line(v2, v3));
    fragments.extend(line(v3, v1));

    fragments // Retorna el vector de fragmentos
}
