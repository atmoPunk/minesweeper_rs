use piston_window::*;
use rand::random;
use opengl_graphics::GlGraphics;


pub struct Board {
    tiles: Vec::<bool>,
    mine_count: i32,
    board_height: i32,
    board_width: i32,
}

impl Board {
    pub fn new(board_height: i32, board_width: i32) -> Self {
        let mut tiles = vec![false; (board_height * board_width) as usize];
        let mine_count = board_height * board_width / 10;
        let mut current_mine_count = 0;
        while current_mine_count < mine_count {
            let mine_x = (random::<u32>() % board_width as u32) as i32;
            let mine_y = (random::<u32>() % board_height as u32) as i32;
            let t = tiles[(mine_y * board_width + mine_x) as usize];
            if t == false {
                tiles[(mine_y * board_width + mine_x) as usize] = true;
                current_mine_count += 1;
            }
        }
        Board { tiles, mine_count, board_height, board_width }
    }

    fn get_tile(&self, x: i32, y: i32) -> &bool {
        &self.tiles[(y * self.board_width + x) as usize]
    }

    fn get_mut_tile(&mut self, x: i32, y: i32) -> &mut bool {
        &mut self.tiles[(y * self.board_width + x) as usize] 
    }

    pub fn render_board(&self, c: &Context, gl: &mut GlGraphics) {
        const PADDING: f64 = 5.0;
        const LINE: f64 = 1.0;
        let bh_pixels = 640.0 - 2.0 * PADDING;
        let bw_pixels = 640.0 - 2.0 * PADDING;
        let color: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
        let position: [f64; 4] = [PADDING, PADDING, bh_pixels, bw_pixels];
        Rectangle::new(color).draw(position, &DrawState::default(), c.transform, gl);
    }
}