use std::fmt::{ Result, Formatter, Display };
use std::ops::{ Add, Mul, Sub };

#[derive(Default)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T: Display> Display for Vec3<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "[{},{},{}]", self.x, self.y, self.z)
    }
}

impl<T> Vec3<T> {
  pub fn new(x: T, y: T, z: T) -> Self {
      Self { x, y, z }
  }
}

pub fn add<T>(a: Vec3<T>, b: Vec3<T>) -> Vec3<T> where 
  T: Copy + 
  Add<T, Output = T> 
{
  Vec3::new(
      a.x + b.x,
      a.y + b.y,
      a.z + b.z
  )
}

pub fn add_scalar<T>(a: Vec3<T>, s: T) -> Vec3<T> where 
  T: Copy + Add<T, Output = T> 
{
  Vec3::new(
      a.x + s,
      a.y + s,
      a.z + s
  )
}

pub fn sub<T>(a: Vec3<T>, b: Vec3<T>) -> Vec3<T> where 
  T: Copy +  Sub<T, Output = T> 
{
  Vec3::new(
      a.x - b.x,
      a.y - b.y,
      a.z - b.z
  )
}

pub fn sub_scalar<T>(a: Vec3<T>, s: T) -> Vec3<T> where 
  T: Copy + Sub<T, Output = T> 
{
  Vec3::new(
      a.x - s,
      a.y - s,
      a.z - s
  )
}

pub fn dot<T>(a: Vec3<T>, b: Vec3<T>) -> T where
  T: Copy + Add<T, Output = T> + Mul<T, Output = T>
{
  (a.x * b.x) + (a.y * b.y) + (a.z * b.z)
}

pub fn dot_scalar<T>(a: Vec3<T>, s: T) -> Vec3<T> where
  T: Copy + Add<T, Output = T> + Mul<T, Output = T>
{
  Vec3::new(
    a.x * s,
    a.y * s,
    a.z * s
  )
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
    dot_scalar
  };

  #[test]
  fn validate_default_values() {
      let vector: Vec3<f32> = Vec3::default();
      assert_eq!(vector.x, 0.0);
      assert_eq!(vector.y, 0.0);
      assert_eq!(vector.z, 0.0);
  }

  #[test]
  fn validate_fmt() {
      let vector: Vec3<f32> = Vec3::default();
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
      let c = add(a, b);
      assert_eq!(c.x, 4.0);
      assert_eq!(c.y, 4.0);
      assert_eq!(c.z, 4.0);
  }

  #[test]
  fn add_scalar_function_validation() {
      let a = Vec3::new(2.0,2.0, 2.0);
      let b = 2.0;
      let c = add_scalar(a, b);
      assert_eq!(c.x, 4.0);
      assert_eq!(c.y, 4.0);
      assert_eq!(c.z, 4.0);
  }

  #[test]
  fn sub_function_validation() {
    let a = Vec3::new(2.0,2.0, 2.0);
    let b = Vec3::new(3.0,3.0, 3.0);
    let c = sub(a, b);
    assert_eq!(c.x, -1.0);
    assert_eq!(c.y, -1.0);
    assert_eq!(c.z, -1.0);
  }

  #[test]
  fn sub_scalar_function_validation() {
    let a = Vec3::new(2.0,2.0, 2.0);
    let b = 3.0;
    let c = sub_scalar(a, b);
    assert_eq!(c.x, -1.0);
    assert_eq!(c.y, -1.0);
    assert_eq!(c.z, -1.0);
  }

  #[test]
  fn dot_function_validation() {
    let a = Vec3::new(2.0,2.0, 2.0);
    let b = Vec3::new(2.0,2.0, 2.0);
    let c = dot(a, b);
    assert_eq!(c, 12.0);
  }

  #[test]
  fn dot_scalar_function_validation() {
    let a = Vec3::new(2.0,2.0, 2.0);
    let b = 3.0;
    let c = dot_scalar(a, b);
    assert_eq!(c.x, 6.0);
    assert_eq!(c.y, 6.0);
    assert_eq!(c.z, 6.0);
  }
}