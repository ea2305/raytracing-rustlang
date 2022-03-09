pub fn transform_coord(x: u32, y: u32, cw: i16, ch: i16) -> (i16, i16) {
  if x >= cw as u32 || y >= ch as u32 {
    panic!("x,y are outside the valid image coordinates");
  }
  (
      (x as i16) - (cw / 2),
      -(y as i16) + (ch / 2)
  )
}

#[cfg(test)]
mod test {
  use super::{transform_coord};

  // functions returns a valid result
  #[test]
  fn transform_coordinates_with_valid_values() {
    let (x, y) = transform_coord(0, 0, 10, 10);
    assert_eq!(x, -5);
    assert_eq!(y, 5);
  }

  #[test]
  fn transform_coordinatess_with_valid_values() {
    let (x, y) = transform_coord(0, 0, 10, 10);
    assert_eq!(x, -5);
    assert_eq!(y, 5);
  }
  // function returns error
  // x < 0 and y < 0 are covered by the type
  // x >= cw
  // y >= cw
  #[test]
  #[should_panic]
  fn transform_coordinates_panic_when_x_outside_limits() {
    let (x, y) = transform_coord(11, 0, 10, 10);
    assert_eq!(y, -5);
    assert_eq!(x, 6);
  }
  
  #[test]
  #[should_panic]
  fn transform_coordinates_panic_when_y_outside_limits() {
    let (x, y) = transform_coord(0, 11, 10, 10);
    assert_eq!(y, -5);
    assert_eq!(x, 6);
  }
}