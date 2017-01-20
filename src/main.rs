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
use std::path::Path;
use graphics::types::Color;

#[derive(Debug)]
struct Pattern {
    count: usize,
    pixels: Vec<[u8; 4]>,
}

struct GenerationContext {

}

impl GenerationContext {
    fn from_path<P>(path: P) -> Self where P:AsRef<Path>{
        let path = path.as_ref();
        GenerationContext {}
    }
}

fn draw_state(buffer: &mut RgbaImage) {

}

fn main() {
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

    let context = GenerationContext::from_path(input);

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
