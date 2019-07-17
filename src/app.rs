use piston_window::*;
use opengl_graphics::GlGraphics;
use crate::board::Board;

pub struct App {
    pub window_bg_color: [f32; 4],
    pub board: Board,
    pub counter: f64,
    pub mouse_pos: [f64; 2],
}

impl App {
    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        let ref c = Context::new_abs(args.window_size[0] as f64, args.window_size[1] as f64);

        let w_bg_col = self.window_bg_color;
        gl.draw(args.viewport(), |_, gl| {
            clear(w_bg_col, gl);
            self.board.render(c, gl)
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.counter += args.dt;
    }

    pub fn mouse_move(&mut self, args: &[f64; 2]) {
        self.mouse_pos = *args;
    }

    pub fn mouse_click(&mut self) {
        self.board.mouse_click(&self.mouse_pos);
    }
}