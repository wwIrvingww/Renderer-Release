// obj.rs
use tobj;
use nalgebra_glm::{Vec2, Vec3};
use crate::vertex::Vertex;
use crate::color::Color;

pub struct Obj {
    vertices: Vec<Vec3>,
    normals: Vec<Vec3>,
    texcoords: Vec<Vec2>,
    indices: Vec<u32>,
}

impl Obj {
    pub fn load(filename: &str) -> Result<Self, tobj::LoadError> {
        let (models, _) = tobj::load_obj(filename, &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        })?;

        let mesh = &models[0].mesh;

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
        let mut vertex_array = Vec::new();

        for &index in &self.indices {
            let pos = self.vertices[index as usize];
            let normal = self.normals.get(index as usize).cloned().unwrap_or(Vec3::new(0.0, 0.0, 0.0));
            let texcoord = self.texcoords.get(index as usize).cloned().unwrap_or(Vec2::new(0.0, 0.0));

            // Crear un vértice utilizando la posición, normal y coordenadas de textura
            let vertex = Vertex {
                position: pos,
                normal,
                tex_coords: texcoord,
                color: Color::black(), // Color negro por defecto, puedes ajustarlo si es necesario
                transformed_position: pos,
                transformed_normal: normal,
            };

            vertex_array.push(vertex);
        }

        vertex_array
    }
}
