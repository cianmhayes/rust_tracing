//#![warn(missing_docs)]

//! # Raytracing in Rust
//!
//! This project is an implementation of the raytracing in one weekend, written in rust.

use image::RgbImage;
use std::error::Error;

mod hittable;
mod interval;
mod ray;
mod vec3;
mod camera;

use interval::Interval;
use hittable::{Hittable, Sphere};
use vec3::Vec3;
use camera::Camera;

fn minimal_make_image() -> RgbImage {
    let cam = Camera::new(400, 16.0/9.0, 100, 50);
    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        }),
    ];
    cam.render(&world)
}

fn main() -> Result<(), Box<dyn Error>> {
    minimal_make_image().save("test5.jpg")?;
    Ok(())
}
