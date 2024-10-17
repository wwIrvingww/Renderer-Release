use nalgebra_glm::{Vec3, Vec4, Mat3};
use crate::vertex::Vertex;
use crate::Uniforms;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
  // Transformar la posición del vértice usando las matrices de modelo, vista y proyección
  let position = Vec4::new(
      vertex.position.x,
      vertex.position.y,
      vertex.position.z,
      1.0
  );
  
  // Transformación combinada (proyección * vista * modelo)
  let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

  // Realizar la división por w para obtener las coordenadas NDC
  let w = transformed.w;
  let ndc_position = Vec4::new(
      transformed.x / w,
      transformed.y / w,
      transformed.z / w,
      1.0
  );

  // Aplicar la matriz de viewport para llevar las coordenadas NDC a la ventana
  let screen_position = uniforms.viewport_matrix * ndc_position;

  // Transformar la normal utilizando solo la parte rotacional de la matriz de modelo
  let model_matrix_3x3 = Mat3::new(
      uniforms.model_matrix[0], uniforms.model_matrix[1], uniforms.model_matrix[2],
      uniforms.model_matrix[4], uniforms.model_matrix[5], uniforms.model_matrix[6],
      uniforms.model_matrix[8], uniforms.model_matrix[9], uniforms.model_matrix[10]
  );
  let transformed_normal = (model_matrix_3x3 * vertex.normal).normalize();

  // Crear un nuevo vértice con los atributos transformados
  Vertex {
      position: vertex.position,
      normal: vertex.normal,
      tex_coords: vertex.tex_coords,
      color: vertex.color,
      transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
      transformed_normal,
  }
}


