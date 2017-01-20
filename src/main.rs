extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston_window;
extern crate image;

use piston::input::*;
use piston::window::WindowSettings;
use piston_window::PistonWindow;
use opengl_graphics::*;
use image::RgbaImage;
use image::Rgba;
use image::Pixel;
use image::GenericImage;
use image::DynamicImage;
use std::path::Path;
use std::collections::HashMap;
use graphics::types::Color;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Pattern {
    pixels: Vec<Rgba<u8>>,
}

impl Pattern {
    fn from_area(image: &DynamicImage, x: u32, y: u32, N: u8) -> Self {
        let N = N as u32;
        let mut pixels = Vec::with_capacity((N*N) as usize);
        let min_x = x - N / 2;
        let min_y = y - N / 2;

        for i in 0..N  {
            for j in 0..N {
                pixels.push( image.get_pixel(min_x + j, min_y + i ).to_rgba() );
            }
        }

        Pattern {
            pixels: pixels,
        }
    }
}

struct GenerationContext {
    patterns: HashMap<Pattern, u16>,
    sortedPatterns: Vec<(u16, Pattern)>,
}

impl GenerationContext {
    fn from_path<P>(path: P, N: u8) -> Self where P:AsRef<Path>{
        let img = image::open(path).unwrap();

        let (w, h) = img.dimensions();

        let mut patterns : HashMap<Pattern, u16> = HashMap::new();

        for y in 1..(h-1) {
            for x in 1..(w-1) {
                let pattern = Pattern::from_area(&img, x, y, N);

                *patterns.entry(pattern).or_insert(0) += 1;
            }
        }
        
        let mut g = GenerationContext {
            sortedPatterns: vec![],
            patterns: patterns,
        };

        g.sortedPatterns = g.patterns.iter().map(|(p, c)| (*c, (*p).clone())).collect();
        g.sortedPatterns.sort_by(|&(a, _), &(b, _)| b.cmp(&a) ); 

        for (c, _) in g.sortedPatterns.clone() {
            println!("{:?}", c);
        }

        g
    }
}

fn draw_state(buffer: &mut RgbaImage) {

}

fn main() {
    const N: u8 = 3;
    let padding = 20;
    let input_w = 32;
    let result_w = 640;
    let result_h = 640;
    let window_w = result_w + padding * 3 + input_w;
    let window_h = result_h + padding * 2;
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow =
        WindowSettings::new(
            "Streamingpattern",
            [window_w, window_h]
        )
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut buffer = RgbaImage::from_pixel(result_w, result_h, Rgba::from_channels(255, 0, 0, 255));
    let mut texture_desc = TextureSettings::new();
    texture_desc.set_filter(Filter::Nearest);

    let mut result_tex = Texture::from_image(&buffer, &texture_desc);

    let input = "./assets/knot.png";
    let input_tex = Texture::from_path(input).unwrap();

    let context = GenerationContext::from_path(input, N);

    let mut gl = GlGraphics::new(opengl);
    while let Some(e) = window.next() {
        use graphics::*;

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                clear([0.0; 4], g);

                draw_state(&mut buffer);

                result_tex.update(&buffer);

                image(&result_tex, c.transform.trans((padding * 2 + input_w) as f64, padding as f64), g);

                let (w, h) = input_tex.get_size();
                image(
                    &input_tex,
                    c.transform.trans(padding as f64, padding as f64).scale(input_w as f64 / w as f64, input_w as f64 / h as f64),
                    g
                );
            });
        }
    }
}
