use crate::numeric_utilities;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::Hittable;
use crate::Interval;
use image::{Rgb, RgbImage};

pub struct Camera {
    image_width: u32,
    image_height: u32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_angle: f32,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    pixel_samples: u32,
    max_ray_depth: u32,
}

pub struct CameraBuilder {
    image_width: u32,
    aspect_ratio: f32,
    vert_fov: f32,
    defocus_angle: f32,
    focal_dist: f32,
    look_from: Vec3,
    look_at: Vec3,
    v_up: Vec3,
    pixel_samples: u32,
    max_ray_depth: u32,
}

impl CameraBuilder {
    pub fn new(image_width: u32, aspect_ratio: f32) -> Self {
        CameraBuilder {
            image_width,
            aspect_ratio,
            vert_fov: 20.0,
            defocus_angle: 0.6,
            focal_dist: 10.0,
            look_from: Vec3::new(0.0, 0.0, 0.0),
            look_at: Vec3::new(0.0, 0.0, -1.0),
            v_up: Vec3::new(0.0, 1.0, 0.0),
            pixel_samples: 500,
            max_ray_depth: 50,
        }
    }

    pub fn image_width(mut self, image_width: u32) -> Self {
        self.image_width = image_width;
        self
    }

    pub fn aspect_ratio(mut self, aspect_ratio: f32) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn vert_fov(mut self, vert_fov: f32) -> Self {
        self.vert_fov = vert_fov;
        self
    }

    pub fn defocus_angle(mut self, defocus_angle: f32) -> Self {
        self.defocus_angle = defocus_angle;
        self
    }

    pub fn focal_dist(mut self, focal_dist: f32) -> Self {
        self.focal_dist = focal_dist;
        self
    }

    pub fn look_from(mut self, look_from: Vec3) -> Self {
        self.look_from = look_from;
        self
    }

    pub fn look_at(mut self, look_at: Vec3) -> Self {
        self.look_at = look_at;
        self
    }

    pub fn v_up(mut self, v_up: Vec3) -> Self {
        self.v_up = v_up;
        self
    }

    pub fn pixel_samples(mut self, pixel_samples: u32) -> Self {
        self.pixel_samples = pixel_samples;
        self
    }

    pub fn max_ray_depth(mut self, max_ray_depth: u32) -> Self {
        self.max_ray_depth = max_ray_depth;
        self
    }

    pub fn build(self) -> Camera {
        Camera::new(
            self.image_width,
            self.aspect_ratio,
            self.vert_fov,
            self.defocus_angle,
            self.focal_dist,
            self.look_from,
            self.look_at,
            self.v_up,
            self.pixel_samples,
            self.max_ray_depth,
        )
    }
}

impl Camera {
    pub fn new(
        image_width: u32,
        aspect_ratio: f32,
        vert_fov: f32,
        defocus_angle: f32,
        focal_dist: f32,
        look_from: Vec3,
        look_at: Vec3,
        v_up: Vec3,
        pixel_samples: u32,
        max_ray_depth: u32,
    ) -> Self {
        let image_height = 1.max((image_width as f32 / aspect_ratio) as u32);

        let theta = vert_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0f32 * focal_dist * h;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        let w = (&look_from - &look_at).unit_vector();
        let u = v_up.cross(&w).unit_vector();
        let v = w.cross(&u);
        let center = look_from;

        let viewport_u = &u * viewport_width;
        let viewport_v = &v * (viewport_height * -1.0);

        let pixel_delta_u = &viewport_u / image_width as f32;
        let pixel_delta_v = &viewport_v / image_height as f32;

        let viewport_upper_left =
            &center - (w * focal_dist) - &viewport_u / 2.0f32 - &viewport_v / 2.0f32;
        let pixel00_loc = &viewport_upper_left + ((&pixel_delta_u + &pixel_delta_v) * 0.5f32);

        let defocus_radius = focal_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            pixel_samples,
            max_ray_depth,
        }
    }

    pub fn render(&self, world: &Vec<Box<dyn Hittable>>) -> RgbImage {
        let mut image = RgbImage::new(self.image_width, self.image_height);
        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let pixel = self.render_point(world, x, y);
                image.put_pixel(x, y, pixel)
            }
        }
        image
    }

    pub fn render_point(&self, world: &Vec<Box<dyn Hittable>>, x: u32, y: u32) -> Rgb<u8> {
        let hit_interval = Interval::<f32>::new(0.001, f32::INFINITY);
        let clamp_interval = Interval::<f32>::new(0.0, 0.999);
        let mut running_colour = Vec3::new(0.0, 0.0, 0.0);
        for _ in 0..self.pixel_samples {
            let r = self.get_ray(x, y);
            let colour = Self::get_ray_color(&r, world, &hit_interval, self.max_ray_depth);
            running_colour += colour / self.pixel_samples as f32;
        }
        Rgb([
            (clamp_interval.clamp(&numeric_utilities::linear_to_gamma(running_colour.x)) * 256f32)
                as u8,
            (clamp_interval.clamp(&numeric_utilities::linear_to_gamma(running_colour.y)) * 256f32)
                as u8,
            (clamp_interval.clamp(&numeric_utilities::linear_to_gamma(running_colour.z)) * 256f32)
                as u8,
        ])
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let mut pixel_center =
            &self.pixel00_loc + &(&self.pixel_delta_u * x as f32 + &self.pixel_delta_v * y as f32);
        pixel_center += &self.pixel_delta_u * (-0.5 + numeric_utilities::get_rand_float());
        pixel_center += &self.pixel_delta_v * (-0.5 + numeric_utilities::get_rand_float());

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center.clone()
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = &pixel_center - &ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn get_ray_color(
        r: &Ray,
        world: &Vec<Box<dyn Hittable>>,
        hit_interval: &Interval<f32>,
        remaining_ray_depth: u32,
    ) -> Vec3 {
        if remaining_ray_depth == 0 {
            Vec3::new(0.0, 0.0, 0.0)
        } else if let Some(hr) = world.hit(r, hit_interval) {
            if let Some(scattering) = hr.scattered {
                scattering.attenuation
                    * Self::get_ray_color(
                        &scattering.scattered,
                        world,
                        hit_interval,
                        remaining_ray_depth - 1,
                    )
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }
        } else {
            let unit_direction = r.direction.unit_vector();
            let a = 0.5 * (unit_direction.y + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
        }
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let r = Vec3::random_in_unit_disk();
        &self.center + (&self.defocus_disk_u * r.x) + (&self.defocus_disk_v * r.y)
    }
}
