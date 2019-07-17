use piston_window::*;
use rand::random;
use opengl_graphics::GlGraphics;
use crate::tile::Tile;

pub struct Board {
    tiles: Vec::<Tile>,
    mine_count: i32,
    height: i32,
    width: i32,
    height_real: f64,
    width_real: f64,
}

const PADDING: f64 = 5.0;
const LINE: f64 = 2.0;

impl Board {
    pub fn new(height: i32, width: i32) -> Self {
        let height_real = 480.0 - 2.0 * PADDING;
        let width_real = 480.0 - 2.0 * PADDING;
        let tile_height = (height_real - LINE) / height as f64;
        let tile_width = (width_real - LINE)/ width as f64;
        let mut tiles: Vec<Tile> = vec![Default::default(); (height * width) as usize];
        for x in 0 .. width {
            for y in 0 .. height {
                let tile = tiles.get_mut((y * width + x) as usize).unwrap();
                tile.pos_x = x;
                tile.pos_y = y;
                tile.height = tile_height - LINE;
                tile.width = tile_width - LINE;
                tile.pos_x_real = PADDING + x as f64 * tile_width + LINE;
                tile.pos_y_real = PADDING + y as f64 * tile_height + LINE;
                // tiles[(y * board_width + x) as usize].pos_x = x;
                // tiles[(y * board_width + x) as usize].pos_y = y;
                // tiles[(y * board_width + x) as usize].tile_height = tile_height;
                // tiles[(y * board_width + x) as usize].tile_width = tile_width;
        //         tiles[(y * board_width + x) as usize].pos_x_real = PADDING + self.pos_x as f64 * tile_width + LINE;
        // let pos_real_y = PADDING + self.pos_y as f64 * tile_height + LINE;

            }
        }
        let mine_count = height * width / 10;
        let mut current_mine_count = 0;
        while current_mine_count < mine_count {
            let mine_x = (random::<u32>() % width as u32) as i32;
            let mine_y = (random::<u32>() % height as u32) as i32;

            let mut tile = &mut tiles[(mine_y * width + mine_x) as usize];
            if tile.is_mine == false {
                tile.is_mine = true;
                current_mine_count += 1;
            }
        }
        Board { tiles, mine_count, height, width, height_real, width_real }
    }

    fn get_tile(&self, x: i32, y: i32) -> Option<&Tile> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            None
        } else {
            self.tiles.get((y * self.width + x) as usize)
        }
    }

    fn get_tile_mut(&mut self, x: i32, y: i32) -> Option<&mut Tile> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            None
        } else {
            self.tiles.get_mut((y * self.width + x) as usize)
        }
    }

    fn get_tile_real_mut(&mut self, x: f64, y: f64) -> Option<&mut Tile> {
        for tile in self.tiles.iter_mut() {
            if x >= tile.pos_x_real && x <= tile.pos_x_real + tile.width {
                if y >= tile.pos_y_real && y <= tile.pos_y_real + tile.height {
                    return Some(tile);
                }
            }
        }
        None
    }

    pub fn render(&self, c: &Context, gl: &mut GlGraphics) {

        let color: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
        let position: [f64; 4] = [PADDING, PADDING, self.height_real, self.width_real];
        Rectangle::new(color).draw(position, &DrawState::default(), c.transform, gl);


        
        for tile in self.tiles.iter() {
            tile.render(c, gl);
        }
    }

    fn neighbor_mine_count(&self, x: i32, y: i32) -> i32 {
        let mut counter = 0;
        for dx in -1 .. 2 {
            for dy in -1 .. 2 {
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

    pub fn mouse_click(&mut self, mouse_pos: &[f64; 2]) {
        let mouse_x = mouse_pos[0];
        let mouse_y = mouse_pos[1];
        let px;
        let py;
        let mut on = false;
        match self.get_tile_real_mut(mouse_x, mouse_y) {
            Some(tile) => {
                // tile.open();
                px = tile.pos_x;
                py = tile.pos_y;
                on = true;
            },
            None => {
                px = -1;
                py = -1;
            }
        }
        if on {
            self.open_neighbors(px, py);
        }
    }

    pub fn open_neighbors(&mut self, x: i32, y: i32) {
        self.get_tile_mut(x, y).unwrap().open();
        let mine_c = self.neighbor_mine_count(x, y);
        println!("tile: {}, {}, mc: {}", x, y, mine_c);
        if mine_c == 0 {
            for dx in -1 .. 2 {
                for dy in -1 .. 2 {
                    println!("{}, {}", dx, dy);
                    match self.get_tile(x + dx, y + dy) {
                        
                        Some(tile) => {
                            println!("trying: {}, {}", x+dx, y+dy);

                            if !tile.is_open {
                                println!("opening");
                                self.open_neighbors(x + dx, y + dy)
                            }
                        },
                        None => {
                        }
                    }
                }
            }
        }
    }
}