use piston_window::*;
use rand::random;
use opengl_graphics::GlGraphics;
use crate::tile::Tile;

pub struct Board {
    tiles: Vec::<Tile>,
    mine_count: i32,
    board_height: i32,
    board_width: i32,
}

impl Board {
    pub fn new(board_height: i32, board_width: i32) -> Self {
        let mut tiles: Vec<Tile> = vec![Default::default(); (board_height * board_width) as usize];
        for x in 0 .. board_width {
            for y in 0 .. board_height {
                tiles[(y * board_width + x) as usize].pos_x = x;
                tiles[(y * board_width + x) as usize].pos_y = y;
            }
        }
        let mine_count = board_height * board_width / 10;
        let mut current_mine_count = 0;
        while current_mine_count < mine_count {
            let mine_x = (random::<u32>() % board_width as u32) as i32;
            let mine_y = (random::<u32>() % board_height as u32) as i32;

            let mut tile = &mut tiles[(mine_y * board_width + mine_x) as usize];
            if tile.is_mine == false {
                tile.is_mine = true;
                current_mine_count += 1;
            }
        }
        Board { tiles, mine_count, board_height, board_width }
    }

    fn get_tile(&self, x: i32, y: i32) -> Option<&Tile> {
        self.tiles.get((y * self.board_width + x) as usize)
    }

    fn get_tile_mut(&mut self, x: i32, y: i32) -> Option<&mut Tile> {
        self.tiles.get_mut((y * self.board_width + x) as usize)
    }

    pub fn render(&self, c: &Context, gl: &mut GlGraphics) {
        const PADDING: f64 = 5.0;
        const LINE: f64 = 2.0;
        let bh_pixels = 480.0 - 2.0 * PADDING;
        let bw_pixels = 480.0 - 2.0 * PADDING;
        let color: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
        let position: [f64; 4] = [PADDING, PADDING, bh_pixels, bw_pixels];
        Rectangle::new(color).draw(position, &DrawState::default(), c.transform, gl);
        let tile_height = (bh_pixels - LINE) / self.board_height as f64;
        let tile_width = (bw_pixels - LINE)/ self.board_width as f64;
        for tile in self.tiles.iter() {
            tile.render(c, gl, tile_height, tile_width);
        }
    }

    fn neighbor_mine_count(&self, x: i32, y: i32) -> i32 {
        let mut counter = 0;
        for dx in -1 .. 1 {
            for dy in -1 .. 1 {
                match self.get_tile(x + dx, y + dy) {
                    Some(tile) => {
                        counter += tile.is_mine as i32;
                    },
                    None => ()
                }
            } 
        }
        counter
    }
}