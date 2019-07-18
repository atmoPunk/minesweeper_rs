use piston_window::*;
use opengl_graphics::GlGraphics;
use graphics::Image;
use std::path::Path;

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
    pub is_marked: bool,
}

impl Tile {
    pub fn render(&self, c: &Context, gl: &mut GlGraphics, mine_count: i32, text_vec: &[opengl_graphics::Texture]) {
        let mut tile_color: [f32; 4] = [0.7, 0.7, 0.7, 1.0];
        if !self.is_open {
            tile_color = [0.5, 0.5, 1.0, 1.0]; 
        }
        else if self.is_mine {
            // println!("Im a mine, pos: {}, {}", self.pos_x, self.pos_y);
            tile_color = [1.0, 0.5, 0.5, 1.0];
        }
        let position: [f64; 4];
        position = [self.pos_x_real, self.pos_y_real, self.height, self.width];
        let image = Image::new().rect([self.pos_x_real, self.pos_y_real, self.height, self.width]);
        
        Rectangle::new(tile_color).draw(position, &DrawState::default(), c.transform, gl);
        if mine_count != 0 && self.is_open && !self.is_mine {
            image.draw(&text_vec[mine_count as usize], &DrawState::default(), c.transform, gl);
        }

        if self.is_marked {
            image.draw(&text_vec[9], &DrawState::default(), c.transform, gl);
        }
        // Text::new_color(real_color, 12).draw(mine_count.to_string(), ,&DrawState::default(), c.transform, gl);
    }

    pub fn open(&mut self) {
        self.is_marked = false;
        self.is_open = true;
    }

    pub fn mark(&mut self) {
        if !self.is_open {
            self.is_marked = !self.is_marked;
        }
    }
}