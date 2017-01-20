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

fn draw_state(buffer: &mut RgbaImage) {

}

fn main() {
    let window_w = 640;
    let window_h = 480;
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow =
        WindowSettings::new(
            "opengl_graphics: image_test",
            [window_w, window_h]
        )
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut buffer = RgbaImage::from_pixel(window_w, window_h, Rgba::from_channels(255, 0, 0, 255));
    let mut texture_desc = TextureSettings::new();
    texture_desc.set_filter(Filter::Nearest);

    let mut rust_logo = Texture::from_image(&buffer, &texture_desc);
    let mut gl = GlGraphics::new(opengl);
    
    while let Some(e) = window.next() {
        use graphics::*;

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                clear([0.0; 4], g);

                draw_state(&mut buffer);

                rust_logo.update(&buffer);

                image(&rust_logo, c.transform, g);
            });
        }
    }
}
