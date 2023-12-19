use crate::interval::Interval;
use crate::material::{Material, Scattering};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub is_front_face: bool,
    pub scattered: Option<Scattering>,
}

impl HitRecord {
    pub fn new(
        r: &Ray,
        point: Vec3,
        normal: Vec3,
        t: f32,
        scattered: Option<Scattering>,
    ) -> HitRecord {
        let is_front_face = r.direction.dot(&normal) < 0.0;
        HitRecord {
            point,
            normal: if is_front_face {
                normal
            } else {
                &normal * -1.0
            },
            t,
            is_front_face,
            scattered,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, interval: &Interval<f32>) -> Option<HitRecord>;
}

pub struct Sphere<T> {
    pub center: Vec3,
    pub radius: f32,
    pub material: T,
}

impl<T> Hittable for Sphere<T>
where
    T: Material,
{
    fn hit(&self, r: &Ray, interval: &Interval<f32>) -> Option<HitRecord> {
        let oc = &r.origin - &self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Find the nearest root that lies in the acceptable range.
        let sqrt_d = discriminant.sqrt();
        let mut root = (-half_b - sqrt_d) / a;
        if !interval.surrounds(&root) {
            root = (-half_b + sqrt_d) / a;
            if !interval.surrounds(&root) {
                return None;
            }
        }

        let hit_point = r.at(root);
        let normal = (&hit_point - &self.center) / self.radius;

        let scattered = self.material.scatter(r, &normal, &hit_point);
        Some(HitRecord::new(r, hit_point, normal, root, scattered))
    }
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, r: &Ray, interval: &Interval<f32>) -> Option<HitRecord> {
        let mut closest_so_far = interval.max;
        let mut record: Option<HitRecord> = None;
        for hittable in self.iter() {
            if let Some(hr) = hittable.hit(r, &Interval::new(interval.min, closest_so_far)) {
                if hr.t < closest_so_far {
                    closest_so_far = hr.t;
                    record = Some(hr);
                }
            }
        }
        record
    }
}
