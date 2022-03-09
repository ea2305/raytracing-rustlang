// imports image usefull tools
extern crate raytracing;

use image::{ImageBuffer, Rgb};
use raytracing::linear_math::Vec3;
use raytracing::scene::{Light, Sphere};
use raytracing::tracer::canvas_to_viewport;
use raytracing::util::transform_coord;
use raytracing::tracer::trace_ray;

fn main() {
    // ====================================================
    const RECURSION_DEPTH: f64 = 3.0;
    // Perspective configuration
    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let viewport: Vec3 = Vec3::new(1.0, 1.0, 1.0);
    // ====================================================
    // Canvas configuration
    let canvas_width: i16 = 600;
    let canvas_height: i16 = 600;
    // Create a new ImgBuf with
    // width: canvas_width and height: canvas_height
    let mut imgbuf = ImageBuffer::new(canvas_width as u32, canvas_height as u32);
    // ====================================================
    // Scene objects
    let objects: Vec<Sphere> = vec![
        Sphere::new(
            Vec3::new(0.0, -1.0, 3.0),
            Vec3::new(255.0, 0.0, 0.0),
            1.0,
            500.0,
            0.2,
        ),
        Sphere::new(
            Vec3::new(2.0, 0.0, 4.0),
            Vec3::new(0.0, 0.0, 255.0),
            1.0,
            500.0,
            0.3,
        ),
        Sphere::new(
            Vec3::new(-2.0, 0.0, 4.0),
            Vec3::new(0.0, 255.0, 0.0),
            1.0,
            10.0,
            0.4,
        ),
        Sphere::new(
            Vec3::new(0.0, -5001.0, 0.0),
            Vec3::new(255.0, 255.0, 0.0),
            5000.0,
            1000.0,
            0.5,
        ),
    ];
    let lights: Vec<Light> = vec![
        Light::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            0.2,
            String::from("AMBIENT"),
        ),
        Light::new(
            Vec3::new(2.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            0.6,
            String::from("POINT"),
        ),
        Light::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 4.0, 4.0),
            0.2,
            String::from("DIRECTIONAL"),
        ),
    ];
    // ====================================================

    // Iterate over the coordinates and pixels of the image
    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
			let (x, y) = transform_coord(_x, _y, canvas_width, canvas_height);
			let point = Vec3::new(x as f64, y as f64, 0.0);
			// generates direction relative to canvas
			let direction = canvas_to_viewport(
				point, 
				&viewport, 
				canvas_width as f64, 
				canvas_height as f64
			);
			let color = trace_ray(
				&objects, 
				&lights, 
				&origin, 
				&direction, 
				1.0, 
				1000.0, 
				RECURSION_DEPTH
			);

			// mutate the pixel reference
			*pixel = Rgb([color.x as u8, color.y as u8, color.z as u8]);
    }

    // Save image
    imgbuf.save("output.png").unwrap();
}
