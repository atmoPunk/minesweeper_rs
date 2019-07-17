extern crate piston_window;
extern crate opengl_graphics;
extern crate glutin_window;
extern crate rand;

use piston_window::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

mod app;
mod board;
mod tile;

fn main() {


    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow<GlutinWindow> = WindowSettings::new("Minesweeper", [480, 480]).exit_on_esc(true).build().unwrap();
    let mut gl = GlGraphics::new(opengl);
    let mut app = app::App {
        window_bg_color: [0.9, 0.9, 0.9, 1.0],
        board: board::Board::new(10, 10),
        counter: 0.0
    };
    while let Some(e) = window.next() {
        if let Some(ref r_args) = e.render_args() {
            app.render(r_args, &mut gl);
        }

        if let Some(ref u_args) = e.update_args() {
            app.update(u_args);
        }
    }
}
