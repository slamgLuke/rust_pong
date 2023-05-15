use rand::Rng;

pub mod constants {
    pub const FOREGROUND: u32 = 0xFFFFFF;
    pub const BACKGROUND: u32 = 0x000000;

    pub const PLAYER_HEIGHT: usize = 4;

    // grid size
    pub const GRID_HEIGHT: usize = 30;
    pub const GRID_WIDTH: usize = 42;
    pub const UPSCALE: usize = 20;

    // screen size
    pub const WIDTH: usize = GRID_WIDTH * UPSCALE;
    pub const HEIGHT: usize = GRID_HEIGHT * UPSCALE;
}

use constants::*;

pub struct Scores(pub usize, pub usize);

pub struct Ball {
    pub x: usize,
    pub y: usize,
    vx: bool,
    vy: bool,
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            x: GRID_WIDTH / 2,
            y: GRID_HEIGHT / 2,
            // rand velocity (either 1 or -1)
            vx: rand::thread_rng().gen::<bool>(),
            vy: rand::thread_rng().gen::<bool>(),
        }
    }

    fn destroy(&mut self) {
        self.x = GRID_WIDTH / 2;
        self.y = GRID_HEIGHT / 2;
        self.vx = rand::thread_rng().gen::<bool>();
        self.vy = rand::thread_rng().gen::<bool>();
    }

    pub fn update(&mut self, buffer: &mut Vec<u32>, scores: &mut Scores) {

        // check for scores
        if self.x <= 0 {
            self.destroy();

            scores.1 += 1;
            return;
        }
        else if self.x >= GRID_WIDTH - 1 {
            self.destroy();

            scores.0 += 1;
            return;
        }

        // bottom - top collision
        if self.y <= 0 || self.y >= GRID_HEIGHT - 1 {
            self.vy = !self.vy;
        }
        
        // update position
        if self.vx {
            self.x += 1;
        }
        else {
            self.x -= 1;
        }
        
        if self.vy {
            self.y += 1;
        }
        else {
            self.y -= 1;
        }

        // draw ball
        draw_pixel(buffer, self.x, self.y);
    }

    pub fn bounce(&mut self) {
        self.vx = !self.vx;
    }

}


pub struct Player {
    pub x: usize,
    pub y: usize,
}

impl Player {
    pub fn new(x: usize, y: usize) -> Player {
        Player {
            x,
            y,
        }
    }

    pub fn update(&mut self, buffer: &mut Vec<u32> ,up: bool, down: bool) {
        // both up and down pressed
        if up && down {
            for i in 0..PLAYER_HEIGHT {
                draw_pixel(buffer, self.x, self.y + i);
            }
        }
        // up pressed
        else if up && self.y > 0 {
            self.y -= 1;
        }
        // down pressed
        else if down && self.y < GRID_HEIGHT - PLAYER_HEIGHT {
            self.y += 1;
        }

        // draw player
        for i in 0..PLAYER_HEIGHT {
            draw_pixel(buffer, self.x, self.y + i);
        }
    }
}

fn draw_pixel(buffer: &mut Vec<u32>, x: usize, y: usize) {
    for (i, p) in buffer.iter_mut().enumerate() {
        let ix = i % WIDTH;
        let iy = i / WIDTH;

        if ix >= x * UPSCALE && ix < (x + 1) * UPSCALE && iy >= y * UPSCALE && iy < (y + 1) * UPSCALE {
            *p = FOREGROUND;
        }
    }
}
