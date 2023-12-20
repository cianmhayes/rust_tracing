use crate::hittable::Impact;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::distributions::Standard;
use rand::prelude::*;

pub struct Scattering {
    pub scattered: Ray,
    pub attenuation: Vec3,
}

pub trait Material {
    fn scatter(&self, r: &Ray, impact: &Impact) -> Option<Scattering>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, impact: &Impact) -> Option<Scattering> {
        let mut direction = &impact.normal + &Vec3::random_unit();
        if direction.is_near_zero() {
            direction = impact.normal.clone();
        }
        Some(Scattering {
            scattered: Ray::new(impact.point.clone(), direction),
            attenuation: self.albedo.clone(),
        })
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, impact: &Impact) -> Option<Scattering> {
        let mut reflected = r.direction.unit_vector().reflect(&impact.normal);
        reflected += Vec3::random_unit() * self.fuzz;
        if reflected.dot(&impact.normal) > 0.0 {
            Some(Scattering {
                scattered: Ray::new(impact.point.clone(), reflected),
                attenuation: self.albedo.clone(),
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refractive_index: f32,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        Dielectric { refractive_index }
    }
}

impl Dielectric {
    fn refelectance(cos_theta: f32, refraction_ratio: f32) -> f32 {
        let mut r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
    }

    fn get_rand() -> f32 {
        rand::thread_rng().sample(Standard)
    }

    fn get_random_reflection(cos_theta: f32, refraction_ratio: f32) -> bool {
        let reflectance = Self::refelectance(cos_theta, refraction_ratio) - 1.0;
        let rand = Self::get_rand();
        reflectance > rand
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, impact: &Impact) -> Option<Scattering> {
        let refraction_ratio = if impact.is_front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let cos_theta = 1.0f32.min(impact.normal.dot(&r.direction.unit_vector()));
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let should_reflect= cannot_refract || ( impact.is_front_face && Self::get_random_reflection(cos_theta, refraction_ratio));
        let direction = if cannot_refract {
            r.direction.reflect(&impact.normal)
        } else {
            r.direction
                .unit_vector()
                .refract(&impact.normal, refraction_ratio)
        };

        Some(Scattering {
            scattered: Ray::new(impact.point.clone(), direction),
            attenuation: Vec3::new(1.0, 1.0, 1.0),
        })
    }
}
