use image::png::PNGEncoder;
use image::ColorType;
use palette::Pixel;
use palette::Srgb;
use rand::Rng;
use std::env;
use std::f64::MAX;
use std::fs::File;
use ray_tracer_rs::HitRecord;
use ray_tracer_rs::Hittable;

use ray_tracer_rs::{Camera, Point3D, Ray, Sphere};

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let out = File::create(filename).expect("Unable to open file");
    let encoder = PNGEncoder::new(out);
    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::RGB(8))?;
    Ok(())
}

fn ray_color(ray: &Ray, world: &Vec<Sphere>) -> Srgb {
    let hit = hit_world(world, ray, 0.001, std::f64::MAX);
    return match hit {
        Some(hit_record) => {
            // let n = (ray.at(hit_record.t) - Point3D::new(0.0, 0.0, -1.0)).unit_vector();
            let n = hit_record.normal;
            return Srgb::new(
                0.5 * n.x() as f32 + 0.5,
                0.5 * n.y() as f32 + 0.5,
                0.5 * n.z() as f32 + 0.5,
            );
        }
        None => {
            let t: f32 = 0.5 * (ray.direction.unit_vector().y() as f32 + 1.0);
            Srgb::new(
                (1.0 - t) * 1.0 + t * 0.5,
                (1.0 - t) * 1.0 + t * 0.7,
                (1.0 - t) * 1.0 + t * 1.0,
            )
        }
    };
}

#[test]
fn test_ray_color() {
    let p = Point3D::new(0.0, 0.0, 0.0);
    let q = Point3D::new(1.0, 0.0, 0.0);
    let r = Ray::new(p, q);
    let w = Vec::new();
    assert_eq!(ray_color(&r, &w), Srgb::new(0.75, 0.85, 1.0));
}

fn render(pixels: &mut [u8], bounds: (usize, usize)) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1 * 3);

    let samples_per_pixel = 8;

    let camera = Camera::new(
        Point3D::new(0.0, 0.0, 0.0),
        2.0,
        (800 / 600) as f64 * 2.5,
        1.0,
    );
    let mut world: Vec<Sphere> = Vec::new();
    world.push(Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.5));
    world.push(Sphere::new(Point3D::new(0.0, -100.5, -1.0), 100.0));
    let mut rng = rand::thread_rng();

    for y in 0..bounds.1 {
        eprint!(".");
        for x in 0..bounds.0 {
            let mut pixel_colors: Vec<f32> = vec![0.0; 3];
            for _s in 0..samples_per_pixel {
                let u = (x as f64 + rng.gen::<f64>()) / (bounds.0 as f64 - 1.0);
                let v = (bounds.1 as f64 - (y as f64 + rng.gen::<f64>())) / (bounds.1 as f64 - 1.0);
                let r = camera.get_ray(u, v);
                let c = ray_color(&r, &world);
                pixel_colors[0] += c.red;
                pixel_colors[1] += c.green;
                pixel_colors[2] += c.blue;
            }
            let color = Srgb::new(
                pixel_colors[0] / samples_per_pixel as f32,
                pixel_colors[1] / samples_per_pixel as f32,
                pixel_colors[2] / samples_per_pixel as f32,
            );

            let i = y * bounds.0 + x;
            let pixel: [u8; 3] = color.into_format().into_raw();
            pixels[i * 3] = pixel[0];
            pixels[i * 3 + 1] = pixel[1];
            pixels[i * 3 + 2] = pixel[2];
        }
    }
}

fn hit_world(world: &Vec<Sphere>, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let mut closest_so_far = t_max;
    let mut hit_record = None;
    for sphere in world {
        if let Some(hit) = sphere.hit(r, t_min, closest_so_far) {
            closest_so_far = hit.t;
            hit_record = Some(hit);
        }
    }
    return hit_record;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let image_width = 800;
    let image_height = 600;
    let mut pixels = vec![0; image_width * image_height * 3];

    println!("raytracer {}x{}", image_width, image_height);
    if args.len() != 2 {
        println!("Usage: {} <output_file>", args[0]);
        return;
    }
    render(&mut pixels, (image_width, image_height));
    write_image(&args[1], &pixels, (image_width, image_height)).expect("error writing image");
}
