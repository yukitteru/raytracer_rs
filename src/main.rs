use std::env;
use std::fs::File;
use image::ColorType;
use image::png::PNGEncoder;
use palette::Pixel;
use palette::Srgb;

fn write_image(filename: &str, pixels: &[u8], bounds:(usize, usize)) -> Result<(), std::io::Error> {
    let out = File::create(filename).expect("Unable to open file");
    let encoder = PNGEncoder::new(out);
    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::RGB(8))?;
    Ok(())
}

fn render(pixels: &mut [u8], bounds: (usize, usize)) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1 * 3);

    for y in 0..bounds.1 {
        eprintln!("scan lines remaining {}", bounds.1 - y); //scanlines dbg
        for x in 0..bounds.0 {
            let color = Srgb::new(
                (x as f32 / (bounds.0 as f32 - 1.0)) as f32,
                (y as f32 / (bounds.1 as f32 - 1.0)) as f32,
                0.25,
            );
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

