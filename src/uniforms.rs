use nalgebra_glm::Mat4;

pub struct Uniforms {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
}

impl Uniforms {
    pub fn new(model_matrix: Mat4, view_matrix: Mat4, projection_matrix: Mat4) -> Self {
        Uniforms {
            model_matrix,
            view_matrix,
            projection_matrix,
        }
    }
}
