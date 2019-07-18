extern crate piston_window;
extern crate opengl_graphics;
extern crate glutin_window;
extern crate rand;

use piston_window::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use std::thread;

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
        counter: 0.0,
        mouse_pos: [0.0, 0.0]
    };
    while let Some(e) = window.next() {
        if let Some(ref r_args) = e.render_args() {
            app.render(r_args, &mut gl);
        }

        if let Some(ref u_args) = e.update_args() {
            app.update(u_args);
        }

        if let Some(ref press_args) = e.press_args() {
            use piston_window::Button::Mouse;
            // use piston_window::MouseButton;
            if *press_args == Mouse(MouseButton::Left) {
                if app.mouse_click() {
                    window.set_should_close(true);
                }
            }

            if *press_args == Mouse(MouseButton::Right) {
                if app.right_click() {
                    window.set_should_close(true);
                }
            }
        }
        if let Some(ref mouse_args) = e.mouse_cursor_args() {
            // println!("mouse_moved to {}, {}", mouse_args[0], mouse_args[1]);
            app.mouse_move(mouse_args);
        }
    }
}
