extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate sdl2_window;

use std::path::Path;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use opengl_graphics::*;
use sdl2_window::Sdl2Window;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Sdl2Window =
        WindowSettings::new(
            "opengl_graphics: image_test",
            [700, 700]
        )
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let rust_logo = Texture::from_path(&Path::new("./assets/rust.png")).unwrap();
    let mut gl = GlGraphics::new(opengl);
    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                let transform = c.transform.trans(0.0, 0.0);

                clear([1.0; 4], g);

                image(&rust_logo, transform, g);
            });
        }
    }
}
