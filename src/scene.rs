use crate::linear_math::Vec3;

#[derive(Default, Clone)]
pub struct Sphere {
  pub center: Vec3,
  pub color: Vec3,
  pub radious: f64,
  pub specular: f64,
  pub reflective: f64
}

impl Sphere {
  pub fn new(center: Vec3, color: Vec3, radious: f64, specular: f64, reflective: f64) -> Self {
    Self { center, color, radious, specular, reflective }
  }
}

// TODO: surface_type validation?
// valid types:
// - AMBIENT
// - POINT
// - DIRECTIONAL
pub struct Light {
  pub position: Vec3,
  pub direction: Vec3,
  pub intensity: f64,
  pub surface_type: String
}

impl Light {
  pub fn new(position: Vec3, direction: Vec3, intensity: f64, surface_type: String) -> Self {
    Self { position, direction, intensity, surface_type }
  }
}
