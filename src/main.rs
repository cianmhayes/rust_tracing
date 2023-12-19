//#![warn(missing_docs)]

//! # Raytracing in Rust
//!
//! This project is an implementation of the raytracing in one weekend, written in rust.

use image::RgbImage;
use std::error::Error;

mod camera;
mod hittable;
mod interval;
mod material;
mod ray;
mod vec3;

use camera::Camera;
use hittable::{Hittable, Sphere};
use interval::Interval;
use material::{Lambertian, Metal};
use vec3::Vec3;

fn minimal_make_image() -> RgbImage {
    let cam = Camera::new(400, 16.0 / 9.0, 100, 50);
    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
            material: Lambertian::new(Vec3::new(0.8, 0.8, 0.0)),
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
            material: Lambertian::new(Vec3::new(0.7, 0.3, 0.3)),
        }),
        Box::new(Sphere {
            center: Vec3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: Metal::new(Vec3::new(0.8, 0.8, 0.8)),
        }),
        Box::new(Sphere {
            center: Vec3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: Metal::new(Vec3::new(0.8, 0.6, 0.2)),
        }),
    ];
    cam.render(&world)
}

fn main() -> Result<(), Box<dyn Error>> {
    minimal_make_image().save("test7.jpg")?;
    Ok(())
}
