use crate::linear_math::{Vec3, sub, dot, add, dot_scalar, length};
use crate::scene::{Sphere, Light};

pub fn canvas_to_viewport(point: Vec3, viewport: &Vec3, canvas_width: f64, canvas_height: f64) -> Vec3 {
  Vec3::new(
    point.x * viewport.x / canvas_width,
    point.y * viewport.y / canvas_height,
    1.0
  )
}

pub fn intersect_ray(origin: &Vec3, direction: &Vec3, object: &Sphere) -> (f64, f64) {
  // Linear equation prep.
  // TODO: implements multiple objects forms
  let r = object.radious;
  let co = sub(&origin, &object.center);
  let a = dot(&direction, &direction);
  let b = 2.0 * dot(&co, &direction);
  let c = dot(&co, &co) - (r * r);
  let discriminant = b * b - 4.0 * a * c;

  if discriminant < 0.0 {
    return (f64::INFINITY, f64::INFINITY);
  }

  let square_disc= f64::sqrt(discriminant);
  let t1 = (-b + square_disc) / (2.0 * a);
  let t2 = (-b - square_disc) / (2.0 * a);
  (t1, t2)
}

pub fn closest_intersection(objects: &Vec<Sphere>, origin: &Vec3, direction: &Vec3, min: f64, max: f64) -> (Option<Sphere>, f64) {
  let mut closest_t = f64::INFINITY;
  // TODO: implements generic forms
  let mut closest_object: Option<Sphere> = None;

  for object in objects {
    let (t1, t2) = intersect_ray(&origin, &direction, &object);
    if t1 >= min && t1 <= max && t1 < closest_t {
      closest_t = t1;
      closest_object = Some(object.clone());
    }
    if t2 >= min && t2 <= max && t2 < closest_t {
      closest_t = t2;
      closest_object = Some(object.clone());
    }
  }
  (closest_object, closest_t)
}

pub fn reflect_ray(r: &Vec3, n: &Vec3) -> Vec3 {
  let scalar_n_r = dot_scalar(&n, dot(&n, &r));
  sub(
    &dot_scalar(
      &scalar_n_r,
      2.0
    ),
    &r
  )
}

pub fn compute_light(objects: &Vec<Sphere>, lights: &Vec<Light>, p: &Vec3, n: &Vec3, v: &Vec3, s: f64) -> f64 {
  let mut i = 0.0;
  let mut max = 1.0;

  for light in lights {
    if light.surface_type == "AMBIENT" {
      i += light.intensity;
    } else {
      let mut l = Vec3::default();
      let mut r = Vec3::default();

      if light.surface_type == "POINT" {
        l = sub(&light.position, &p);
        max = 1.0;
      } else {
        l = light.direction.clone();
        max = f64::INFINITY;
      }

      let ( shadow, _ct ) = closest_intersection(
        &objects, 
        &p, 
        &l, 
        0.001, 
        max
      );

      if shadow.is_none() {
        let n_dot = dot(&n, &l);
        if n_dot > 0.0 {
          i += (n_dot * light.intensity) / (length(&n) * length(&l));
        }

        if s != -1.0 {
          r = reflect_ray(&l, &n);
          let r_v = dot(&r, &v);
          if r_v > 0.0 {
            i += light.intensity * f64::powf(
              r_v / (length(&r) * length(&v)),
              s
            );
          }
        }
      }
    }
  }
  i
}

pub fn trace_ray(objects: &Vec<Sphere>, lights: &Vec<Light>, origin: &Vec3, direction: &Vec3, min: f64, max: f64, recursion_depth: f64) -> Vec3 {
  let (closest_object, closest_t) = closest_intersection(&objects, &origin, &direction, min, max);
  
  if closest_object.is_none() {
    return Vec3::new(0.0, 0.0, 0.0);
  }
  // lights pre-requisites
  let closest = closest_object.unwrap();
  let p = add(&origin, &dot_scalar(&direction, closest_t));
  let n = sub(&p, &closest.center);
  let n_delta = dot_scalar(&n, 1.0 / length(&n));

  // calculates light
  let light = compute_light(
    &objects, 
    &lights, 
    &p, 
    &n_delta, 
    &dot_scalar(&direction, -1.0), 
    closest.specular
  );
  let local_color = dot_scalar(&closest.color, light);

  // reflections
  // there is no reflective surface
  if recursion_depth <= 0.0 || closest.reflective <= 0.0 {
    return local_color;
  }

  // calculates reflection
  let r = reflect_ray(
    &dot_scalar(
      &direction, 
      -1.0
    ),
    &n_delta
  );

  let reflected_color = trace_ray(
    &objects, 
    &lights, 
    &p, 
    &r, 
    0.001, 
    f64::INFINITY, 
    recursion_depth - 1.0
  );
  let local_color_r = dot_scalar(&local_color, 1.0 - closest.reflective);
  add(&local_color_r, &dot_scalar(&reflected_color, closest.reflective))
}

#[cfg(test)]
mod test {
  use crate::{linear_math::Vec3, scene::{Sphere, Light}};
  use std::f64::INFINITY;
  use super::{
    canvas_to_viewport,
    intersect_ray,
    closest_intersection,
    reflect_ray,
    compute_light, trace_ray
  };

  #[test]
  fn return_canvas_transformation() {
    let viewport = Vec3::new(1.0, 1.0, 1.0);
    let canvas = canvas_to_viewport(
      Vec3::new(0.0, 0.0, 0.0),
      &viewport,
      10.0,
      10.0
    );
    assert_eq!(canvas.x, 0.0);
    assert_eq!(canvas.y, 0.0);
    assert_eq!(canvas.z, 1.0);
  }

  #[test]
  fn return_canvas_transformation_with_no_zeros_values() {
    let viewport = Vec3::new(2.0, 2.0, 2.0);
    let canvas = canvas_to_viewport(
      Vec3::new(1.0, 1.0, 1.0),
      &viewport,
      10.0,
      10.0
    );
    assert_eq!(canvas.x, 0.2);
    assert_eq!(canvas.y, 0.2);
    assert_eq!(canvas.z, 1.0);
  }

  #[test]
  fn solves_intersection() {
    let origin = Vec3::new(0.0,0.0,0.0);
    let direction = Vec3::new(-0.5,-0.5,1.0);
    let object = Sphere::new(
      Vec3::new(
        0.0, 
        -5001.0, 
        0.0
      ),
      Vec3::new(
        255.0, 
        255.0, 
        0.0
      ),
      5000.0,
      1000.0,
      0.5
    );
    let (t1, t2) = intersect_ray(&origin, &direction, &object);
    assert_eq!(t1, 3331.9989989986984);
    assert_eq!(t2, 2.001001001302029);
  }

  #[test]
  fn solves_with_infinite_values () {
    let origin = Vec3::new(0.0,0.0,0.0);
    let direction = Vec3::new(-0.5,-0.5,1.0);
    let object = Sphere::new(
      Vec3::new(
        0.0, 
        -5001.0, 
        0.0
      ),
      Vec3::new(
        255.0, 
        255.0, 
        0.0
      ),
      500.0,
      1000.0,
      0.5
    );
    let (t1, _t2) = intersect_ray(&origin, &direction, &object);
    assert_eq!(t1, INFINITY);
  }

  #[test]
  fn calculates_closest_intersection() {
    let origin = Vec3::new(0.0,0.0,0.0);
    let direction = Vec3::new(-0.5,-0.5,1.0);
    let objects = vec![
      Sphere::new(
        Vec3::new(
          0.0, 
          -5001.0, 
          0.0
        ),
        Vec3::new(
          255.0, 
          255.0, 
          0.0
        ),
        5000.0,
        1000.0,
        0.5
      )
    ];
    let (closest_object, closest_t) = closest_intersection(
      &objects,
      &origin,
      &direction, 
      1.0, 
      1000.0
    );
    let object = closest_object.unwrap();
    assert_eq!(object.center.x, 0.0);
    assert_eq!(object.center.y, -5001.0);
    assert_eq!(object.center.z, 0.0);
    assert_eq!(closest_t, 2.001001001302029);
  }

  #[test]
  fn calculates_reflect_ray() {
    let a = Vec3::new(1.0, 1.0, 1.0);
    let b = Vec3::new(1.0, 1.0, 1.0);
    let r = reflect_ray(&a, &b);
    assert_eq!(r.x, 5.0);
    assert_eq!(r.y, 5.0);
    assert_eq!(r.z, 5.0);
  }

  #[test]
  fn starts_light_calculation() {
    // TODO: maybe should be a global object
    let objects: Vec<Sphere> = vec![
        Sphere::new(
            Vec3::new(0.0, -1.0, 3.0),
            Vec3::new(255.0, 0.0, 0.0),
            1.0,
            500.0,
            0.2
        ),
        Sphere::new(
            Vec3::new(2.0, 0.0, 4.0),
            Vec3::new(0.0, 0.0, 255.0),
            1.0,
            500.0,
            0.3
        ),
        Sphere::new(
            Vec3::new(-2.0, 0.0, 4.0),
            Vec3::new(0.0, 255.0, 0.0),
            1.0,
            10.0,
            0.4
        ),
        Sphere::new(
            Vec3::new(0.0, -5001.0, 0.0),
            Vec3::new(255.0, 255.0, 0.0),
            5000.0,
            1000.0,
            0.5
        ),
    ];
    let lights: Vec<Light> = vec![
      Light::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        0.2,
        String::from("AMBIENT")
      ),
      Light::new(
        Vec3::new(2.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        0.6,
        String::from("POINT")
      ),
      Light::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 4.0, 4.0),
        0.2,
        String::from("DIRECTIONAL")
      )
    ];
    let r = compute_light(
      &objects,
      &lights,
      &Vec3::new(-1.0005005006510146, -1.0005005006510146, 2.001001001302029),
      &Vec3::new(-0.00020010010013020294, 0.9999998998998699, 0.0004002002002604059),
      &Vec3::new(0.5, 0.5, -1.0),
      1000.0
    );

    assert_eq!(r, 0.6301454612898025);
  }

  #[test]
  fn gets_color_with_light_and_shadows() {
    let origin = Vec3::new(0.0,0.0,0.0);
    let direction = Vec3::new(-0.5,-0.5,1.0);
    let objects: Vec<Sphere> = vec![
        Sphere::new(
            Vec3::new(0.0, -1.0, 3.0),
            Vec3::new(255.0, 0.0, 0.0),
            1.0,
            500.0,
            0.2
        ),
        Sphere::new(
            Vec3::new(2.0, 0.0, 4.0),
            Vec3::new(0.0, 0.0, 255.0),
            1.0,
            500.0,
            0.3
        ),
        Sphere::new(
            Vec3::new(-2.0, 0.0, 4.0),
            Vec3::new(0.0, 255.0, 0.0),
            1.0,
            10.0,
            0.4
        ),
        Sphere::new(
            Vec3::new(0.0, -5001.0, 0.0),
            Vec3::new(255.0, 255.0, 0.0),
            5000.0,
            1000.0,
            0.5
        ),
    ];
    let lights: Vec<Light> = vec![
      Light::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        0.2,
        String::from("AMBIENT")
      ),
      Light::new(
        Vec3::new(2.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        0.6,
        String::from("POINT")
      ),
      Light::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 4.0, 4.0),
        0.2,
        String::from("DIRECTIONAL")
      )
    ];
    let color = trace_ray(
      &objects,
      &lights,
      &origin,
      &direction,
      1.0,
      1000.0,
      3.0
    );
    assert_eq!(color.x, 96.38257683539419);
    assert_eq!(color.y, 144.8400555916369);
    assert_eq!(color.z, 0.0);
  }
}