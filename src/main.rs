use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use std::env;
use palette::Pixel;
use palette::Srgb;

use ray_tracer_rs::{Point3D, Ray, Camera};


fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let out = File::create(filename).expect("Unable to open file");
    let encoder = PNGEncoder::new(out);
    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::RGB(8))?;
    Ok(())
}

#[test]
fn test_ray_color() {
    let p = Point3D::new(0.0, 0.0, 0.0);
    let q = Point3D::new(1.0, 0.0, 0.0);
    let r = Ray::new(p, q);
    assert_eq!(ray_color(&r), Srgb::new(0.75, 0.85, 1.0));
}

fn ray_color(ray: &Ray) -> Srgb {
    let t: f32 = 0.5 * (ray.direction.unit_vector().y() as f32 + 1.0);
    return Srgb::new((1.0 - t) * 1.0 + t * 0.5, (1.0 - t) * 1.0 + t * 0.7, (1.0 - t) * 1.0 + t * 1.0);
}

fn render(pixels: &mut [u8], bounds: (usize, usize)) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1 * 3);
    let camera = Camera::new(Point3D::new(0.0, 0.0, 0.0), 2.0, (800 / 600) as f64 * 2.0, 1.0);

    for y in 0..bounds.1 {
        eprintln!("scan lines remaining {}", bounds.1 - y); //scanlines dbg
        for x in 0..bounds.0 {
            let u = (x as f64) / (bounds.0 as f64 - 1.0);
            let v = (bounds.1 as f64 - y as f64) / (bounds.1 as f64 - 1.0);
            let r = Ray::new(camera.origin, camera.lower_left_corner + camera.horizontal * u + camera.vertical * v - camera.origin);
            let color = ray_color(&r);

            let i = y * bounds.0 + x;
            let pixel: [u8; 3] = color.into_format().into_raw();
            pixels[i * 3] = pixel[0];
            pixels[i * 3 + 1] = pixel[1];
            pixels[i * 3 + 2] = pixel[2];
        }
    }
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

