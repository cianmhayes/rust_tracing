use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Scattering {
    pub scattered: Ray,
    pub attentuation: Vec3,
}

pub trait Material {
    fn scatter(&self, r: &Ray, normal: &Vec3, hit_point: &Vec3) -> Option<Scattering>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo:Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, normal: &Vec3, hit_point: &Vec3) -> Option<Scattering> {
        let mut direction = normal + &Vec3::random_unit();
        if direction.is_near_zero() {
            direction = normal.clone();
        }
        Some(Scattering {
            scattered: Ray::new(hit_point.clone(), direction),
            attentuation: self.albedo.clone(),
        })
    }
}

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo:Vec3) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, normal: &Vec3, hit_point: &Vec3) -> Option<Scattering> {
        let reflected = r.direction.unit_vector().reflect(normal);
        Some(Scattering {
            scattered: Ray::new(hit_point.clone(), reflected),
            attentuation: self.albedo.clone(),
        })
    }
}
