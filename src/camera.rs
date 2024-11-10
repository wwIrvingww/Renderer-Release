use nalgebra_glm::{Vec3, rotate_vec3, look_at };
use std::f32::consts::PI;

pub struct Camera {
  pub eye: Vec3,
  pub center: Vec3,
  pub up: Vec3,
  pub yaw: f32,
  pub pitch: f32,
  pub has_changed: bool,
}


impl Camera {
  pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
    Camera {
      eye,
      center,
      up,
      yaw: 0.0,   // Inicializa yaw a 0.0
      pitch: 0.0, // Inicializa pitch a 0.0
      has_changed: true,
    }
  }

  pub fn get_view_matrix(&self) -> nalgebra_glm::Mat4 {
    look_at(&self.eye, &self.center, &self.up)
  }

  pub fn basis_change(&self, vector: &Vec3) -> Vec3 {
    let forward = (self.center - self.eye).normalize();
    let right = forward.cross(&self.up).normalize();
    let up = right.cross(&forward).normalize();

    let rotated = 
    vector.x * right +
    vector.y * up +
    - vector.z * forward;

    rotated.normalize()
  }

  pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
    let radius_vector = self.eye - self.center;
    let radius = radius_vector.magnitude();

    self.yaw = (radius_vector.z.atan2(radius_vector.x) + delta_yaw) % (2.0 * PI);

    let radius_xz = (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
    self.pitch = (-radius_vector.y).atan2(radius_xz) + delta_pitch;
    self.pitch = self.pitch.clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

    let new_eye = self.center + Vec3::new(
      radius * self.yaw.cos() * self.pitch.cos(),
      -radius * self.pitch.sin(),
      radius * self.yaw.sin() * self.pitch.cos()
    );

    self.eye = new_eye;
    self.has_changed = true;
  }


  pub fn zoom(&mut self, delta: f32) {
    let direction = (self.center - self.eye).normalize();
    self.eye += direction * delta;
    self.has_changed = true;
  }

  pub fn move_center(&mut self, direction: Vec3) {
    let radius_vector = self.center - self.eye;
    let radius = radius_vector.magnitude();

    let angle_x = direction.x * 0.05; // Adjust this factor to control rotation speed
    let angle_y = direction.y * 0.05;

    let rotated = rotate_vec3(&radius_vector, angle_x, &Vec3::new(0.0, 1.0, 0.0));

    let right = rotated.cross(&self.up).normalize();
    let final_rotated = rotate_vec3(&rotated, angle_y, &right);

    self.center = self.eye + final_rotated.normalize() * radius;
    self.has_changed = true;
  }
  
  pub fn get_forward_vector(&self) -> Vec3 {
    (self.center - self.eye).normalize()
  }
}
