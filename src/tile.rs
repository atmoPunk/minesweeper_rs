use piston_window::*;
use opengl_graphics::GlGraphics;

#[derive(Debug, Default, Clone, Copy)]
pub struct Tile {
    pub is_mine: bool,
    pub pos_x: i32,
    pub pos_y: i32,
    pub height: f64,
    pub width: f64,
    pub pos_x_real: f64,
    pub pos_y_real: f64,
    pub is_open: bool,
}

impl Tile {
    pub fn render(&self, c: &Context, gl: &mut GlGraphics) {
        let color: [f32; 4];
        if !self.is_open {
            color = [0.5, 0.5, 1.0, 1.0]; 
        }
        else if self.is_mine {
            // println!("Im a mine, pos: {}, {}", self.pos_x, self.pos_y);
            color = [1.0, 0.5, 0.5, 1.0];
        } else {
            color = [0.5, 1.0, 0.5, 1.0];
        }
        let position: [f64; 4];

        position = [self.pos_x_real, self.pos_y_real, self.height, self.width];
        Rectangle::new(color).draw(position, &DrawState::default(), c.transform, gl);
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }
}