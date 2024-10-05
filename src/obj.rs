// obj.rs
use tobj;
use nalgebra_glm::{Vec2, Vec3};
use crate::vertex::Vertex;

pub struct Obj {
    pub vertices: Vec<Vec3>,  // Hacer público
    pub normals: Vec<Vec3>,   // Hacer público
    pub texcoords: Vec<Vec2>, // Hacer público
    pub indices: Vec<u32>,    // Hacer público
}


impl Obj {
    pub fn load(filename: &str) -> Result<Self, tobj::LoadError> {
        let (models, _) = tobj::load_obj(filename, &tobj::LoadOptions {
            single_index: true,
            triangulate: true, // Asegura que las caras cuadradas se conviertan en triángulos
            ..Default::default()
        })?;

        // Verifica que haya al menos un modelo cargado
        if models.is_empty() {
            println!("no hay modelo")
        }

        let mesh = &models[0].mesh;

        println!("Number of vertices: {}", mesh.positions.len() / 3);
        println!("Number of indices: {}", mesh.indices.len());

        let vertices: Vec<Vec3> = mesh.positions.chunks(3)
            .map(|v| Vec3::new(v[0], v[1], v[2]))
            .collect();

        let normals: Vec<Vec3> = mesh.normals.chunks(3)
            .map(|n| Vec3::new(n[0], n[1], n[2]))
            .collect();

        let texcoords: Vec<Vec2> = mesh.texcoords.chunks(2)
            .map(|t| Vec2::new(t[0], t[1]))
            .collect();

        let indices = mesh.indices.clone();

        Ok(Obj {
            vertices,
            normals,
            texcoords,
            indices,
        })
    }

    pub fn get_vertex_array(&self) -> Vec<Vertex> {
        let mut vertices = Vec::new();

        // Recorremos los índices
        for &index in &self.indices {
            // Obtenemos la posición del vértice
            let position = self.vertices[index as usize];

            // Obtenemos la normal o usamos una normal por defecto si no existe
            let normal = self.normals.get(index as usize)
                .cloned()
                .unwrap_or(Vec3::new(0.0, 1.0, 0.0)); // Normal por defecto

            // Obtenemos las coordenadas de textura o usamos (0.0, 0.0) si no existe
            let tex_coords = self.texcoords.get(index as usize)
                .cloned()
                .unwrap_or(Vec2::new(0.0, 0.0)); // Coordenadas de textura por defecto

            // Crear el vértice y añadirlo al array
            vertices.push(Vertex::new(position, normal, tex_coords));
        }

        vertices
    }

}