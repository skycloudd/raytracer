use crate::colour::{rgb, Colour};
use crate::hit::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::{
    ASPECT_RATIO, DEFOCUS_ANGLE, FOCUS_DISTANCE, LOOK_AT, LOOK_FROM, MAX_DEPTH, SAMPLES_PER_PIXEL,
    VERTICAL_FOV, WIDTH,
};
use glam::Vec3;
use image::{Rgb, RgbImage};

pub struct Camera {
    image: RgbImage,
    width: u32,
    height: u32,
    origin: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_00: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let origin = LOOK_FROM;

        let width = WIDTH;
        let height = (width as f32 / ASPECT_RATIO) as u32;

        let image = RgbImage::new(width, height);

        let real_aspect_ratio = width as f32 / height as f32;

        let theta = VERTICAL_FOV.to_radians();
        let half_height = (theta / 2.0).tan();

        let viewport_height = 2.0 * half_height * FOCUS_DISTANCE;
        let viewport_width = viewport_height * real_aspect_ratio;

        let v_up = Vec3::new(0.0, 1.0, 0.0);

        let w = (LOOK_FROM - LOOK_AT).normalize();
        let u = v_up.cross(w).normalize();
        let v = w.cross(u);

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        let pixel_delta_u = viewport_u / (width as f32);
        let pixel_delta_v = viewport_v / (height as f32);

        let viewport_top_left = origin - (FOCUS_DISTANCE * w) - (viewport_u + viewport_v) / 2.0;

        let pixel_00 = viewport_top_left + (pixel_delta_u + pixel_delta_v) / 2.0;

        let defocus_radius = FOCUS_DISTANCE * (DEFOCUS_ANGLE / 2.0).to_radians().tan();

        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            image,
            width,
            height,
            origin,
            pixel_delta_u,
            pixel_delta_v,
            pixel_00,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&mut self, world: &impl Hittable) {
        for y in 0..self.height {
            eprint!("Rendering line {}/{}\r", y + 1, self.height);

            for x in 0..self.width {
                let mut colour = Colour::new(0.0, 0.0, 0.0);

                for n in 0..SAMPLES_PER_PIXEL {
                    let ray = self.get_ray(x, y, n);

                    colour += ray_colour(&ray, world, MAX_DEPTH);
                }

                self.write_pixel(x, y, colour, SAMPLES_PER_PIXEL);
            }
        }

        self.image.save("image.png").unwrap();
    }

    fn get_ray(&self, x: u32, y: u32, n: u32) -> Ray {
        let pixel_center =
            self.pixel_00 + (self.pixel_delta_u * x as f32) + (self.pixel_delta_v * y as f32);
        let pixel_sample = pixel_center + self.sample_pixel(n);

        let origin = if DEFOCUS_ANGLE > 0.0 {
            let defocus_disk = random_in_unit_disk();

            self.origin
                + (self.defocus_disk_u * defocus_disk.x)
                + (self.defocus_disk_v * defocus_disk.y)
        } else {
            self.origin
        };

        let direction = pixel_sample - origin;

        Ray::new(origin, direction)
    }

    fn write_pixel(&mut self, x: u32, y: u32, colour: Colour, samples: u32) {
        let r = (colour.x / samples as f32).sqrt();
        let g = (colour.y / samples as f32).sqrt();
        let b = (colour.z / samples as f32).sqrt();

        let intensity = 0.0..1.0;

        let r = (intensity.clamp(r) * 255.0) as u8;
        let g = (intensity.clamp(g) * 255.0) as u8;
        let b = (intensity.clamp(b) * 255.0) as u8;

        self.image.put_pixel(x, y, Rgb([r, g, b]));
    }

    fn sample_pixel(&self, nth: u32) -> Vec3 {
        let sqrt_samples = (SAMPLES_PER_PIXEL as f32).sqrt() as u32;

        let nth = nth % SAMPLES_PER_PIXEL;

        let x = ((nth % sqrt_samples) as f32 / sqrt_samples as f32) - 0.5;
        let y = ((nth / sqrt_samples) as f32 / sqrt_samples as f32) - 0.5;

        x * self.pixel_delta_u + y * self.pixel_delta_v
    }
}

fn ray_colour(ray: &Ray, hittable: &impl Hittable, depth: u16) -> Colour {
    if depth == 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }

    let rec = hittable.hit(ray, 0.001..f32::INFINITY);

    if let Some(rec) = rec {
        let scatter = rec.material().scatter(ray, &rec);

        if let Some((attenuation, scattered)) = scatter {
            attenuation * ray_colour(&scattered, hittable, depth - 1)
        } else {
            Colour::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = ray.direction().normalize();

        let t = 0.5 * (unit_direction.y + 1.0);

        (1.0 - t) * rgb(255, 255, 255) + t * rgb(127, 178, 255)
    }
}

fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            -0.5 + rand::random::<f32>(),
            -0.5 + rand::random::<f32>(),
            0.0,
        ) * 2.0;

        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
