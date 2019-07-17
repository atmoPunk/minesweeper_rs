use piston_window::*;
use opengl_graphics::GlGraphics;

#[derive(Debug, Default, Clone, Copy)]
pub struct Tile {
    pub is_mine: bool,
    pub pos_x: i32,
    pub pos_y: i32,
}

impl Tile {
    pub fn render(&self, c: &Context, gl: &mut GlGraphics, tile_height: f64, tile_width: f64) {
        const PADDING: f64 = 5.0;
        const LINE: f64 = 2.0;
        let color: [f32; 4];
        if self.is_mine {
            println!("Im a mine, pos: {}, {}", self.pos_x, self.pos_y);
            color = [1.0, 0.5, 0.5, 1.0];
        } else {
            color = [0.5, 1.0, 0.5, 1.0];
        }
        let position: [f64; 4];
        let pos_real_x = PADDING + self.pos_x as f64 * tile_width + LINE;
        let pos_real_y = PADDING + self.pos_y as f64 * tile_height + LINE;
        position = [pos_real_x, pos_real_y, tile_height - LINE, tile_width - LINE];
        Rectangle::new(color).draw(position, &DrawState::default(), c.transform, gl);
    }
}