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
mod numeric_utilities;
mod ray;
mod vec3;

use camera::CameraBuilder;
use hittable::{Hittable, Sphere};
use interval::Interval;
use material::{Dielectric, Lambertian, Metal};
use vec3::Vec3;

use rand::distributions::Standard;
use rand::prelude::*;

fn get_rand() -> f32 {
    rand::thread_rng().sample(Standard)
}

fn make_image() -> RgbImage {
    let cam = CameraBuilder::new(400, 16.0 / 9.0)
        .look_from(Vec3::new(13.0, 2.0, 3.0))
        .build();

    let mut world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {
            // ground
            center: Vec3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Lambertian::new(Vec3::new(0.5, 0.5, 0.5)),
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: Dielectric::new(1.5),
        }),
        Box::new(Sphere {
            center: Vec3::new(-4.0, 1.0, 0.0),
            radius: 1.0,
            material: Lambertian::new(Vec3::new(0.4, 0.2, 0.1)),
        }),
        Box::new(Sphere {
            center: Vec3::new(4.0, 1.0, 0.0),
            radius: 1.0,
            material: Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0),
        }),
    ];
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                a as f32 + 0.9 * get_rand(),
                0.2,
                b as f32 + 0.9 * get_rand(),
            );

            if (&center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let choose_mat = get_rand();
                if choose_mat < 0.8 {
                    let albedo = Vec3::random() * Vec3::random();
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Lambertian::new(albedo),
                    }));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_in_range(0.5, 1.0);
                    let fuzz = get_rand();
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Metal::new(albedo, fuzz),
                    }));
                } else {
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Dielectric::new(1.5),
                    }));
                }
            }
        }
    }
    cam.render(&world)
}

fn main() -> Result<(), Box<dyn Error>> {
    make_image().save("test2.jpg")?;
    Ok(())
}
