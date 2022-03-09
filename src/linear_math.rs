use std::fmt::{ Result, Formatter, Display };

#[derive(Clone, Default)]
pub struct Vec3 {
  pub x: f64,
  pub y: f64,
  pub z: f64
}

impl Display for Vec3 {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "[{},{},{}]", self.x, self.y, self.z)
  }
}

impl Vec3 {
  pub fn new(x: f64, y: f64, z: f64) -> Self {
    Self { x, y, z }
  }
}

pub fn add(a: &Vec3, b: &Vec3) -> Vec3 {
  Vec3::new(
    a.x + b.x,
    a.y + b.y,
    a.z + b.z
  )
}

pub fn add_scalar(a: &Vec3, s: f64) -> Vec3 {
  Vec3::new(
    a.x + s,
    a.y + s,
    a.z + s
  )
}

pub fn sub(a: &Vec3, b: &Vec3) -> Vec3 {
  Vec3::new(
    a.x - b.x,
    a.y - b.y,
    a.z - b.z
  )
}

pub fn sub_scalar(a: &Vec3, s: f64) -> Vec3 {
  Vec3::new(
    a.x - s,
    a.y - s,
    a.z - s
  )
}

pub fn dot(a: &Vec3, b: &Vec3) -> f64 {
  (a.x * b.x) + (a.y * b.y) + (a.z * b.z)
}

pub fn dot_scalar(a: &Vec3, s: f64) -> Vec3 {
  Vec3::new(
    a.x * s,
    a.y * s,
    a.z * s
  )
}

pub fn length(a: &Vec3) -> f64 {
  f64::sqrt(dot(&a, &a))
}

#[cfg(test)]
mod tests {
  use super::{
    Vec3,
    add,
    add_scalar,
    sub,
    sub_scalar,
    dot,
    dot_scalar,
    length
  };

  #[test]
  fn validate_default_values() {
      let vector: Vec3 = Vec3::default();
      assert_eq!(vector.x, 0.0);
      assert_eq!(vector.y, 0.0);
      assert_eq!(vector.z, 0.0);
  }

  #[test]
  fn validate_fmt() {
      let vector: Vec3 = Vec3::default();
      assert_eq!(vector.to_string(), "[0,0,0]");
  }

  #[test]
  fn validate_custom_values() {
      let vector = Vec3::new(2.0, 2.0, 2.0);
      assert_eq!(vector.x, 2.0);
      assert_eq!(vector.y, 2.0);
      assert_eq!(vector.z, 2.0);
  }

  #[test]
  fn add_function_validation() {
      let a = Vec3::new(2.0,2.0, 2.0);
      let b = Vec3::new(2.0,2.0, 2.0);
      let c = add(&a, &b);
      assert_eq!(c.x, 4.0);
      assert_eq!(c.y, 4.0);
      assert_eq!(c.z, 4.0);
  }

  #[test]
  fn add_scalar_function_validation() {
      let a = Vec3::new(2.0,2.0, 2.0);
      let b = 2.0;
      let c = add_scalar(&a, b);
      assert_eq!(c.x, 4.0);
      assert_eq!(c.y, 4.0);
      assert_eq!(c.z, 4.0);
  }

  #[test]
  fn sub_function_validation() {
    let a = Vec3::new(2.0,2.0, 2.0);
    let b = Vec3::new(3.0,3.0, 3.0);
    let c = sub(&a, &b);
    assert_eq!(c.x, -1.0);
    assert_eq!(c.y, -1.0);
    assert_eq!(c.z, -1.0);
  }

  #[test]
  fn sub_scalar_function_validation() {
    let a = Vec3::new(2.0,2.0, 2.0);
    let b = 3.0;
    let c = sub_scalar(&a, b);
    assert_eq!(c.x, -1.0);
    assert_eq!(c.y, -1.0);
    assert_eq!(c.z, -1.0);
  }

  #[test]
  fn dot_function_validation() {
    let a = Vec3::new(2.0,2.0, 2.0);
    let b = Vec3::new(2.0,2.0, 2.0);
    let c = dot(&a, &b);
    assert_eq!(c, 12.0);
  }

  #[test]
  fn dot_scalar_function_validation() {
    let a = Vec3::new(2.0,2.0, 2.0);
    let b = 3.0;
    let c = dot_scalar(&a, b);
    assert_eq!(c.x, 6.0);
    assert_eq!(c.y, 6.0);
    assert_eq!(c.z, 6.0);
  }

  #[test]
  fn calculates_length_of_vector() {
    let a = Vec3::new(1.0,1.0, 1.0);
    let length = length(&a);
    assert_eq!(length, f64::sqrt(3.0));
  }
}