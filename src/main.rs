#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

mod components;
use components::{Player, Ball, Scores, constants::*};
use minifb::{Key, Window, WindowOptions};

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    
    let mut window = Window::new(
        "PONG",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // cap fps
    // window.limit_update_rate(Some(std::time::Duration::from_millis(100)));

    // game setup
    let mut scores = Scores(0, 0);
    let mut player1 = Player::new(2, 12);
    let mut player2 = Player::new(GRID_WIDTH - 2 - 1, 12);
    let mut ball = Ball::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {

        let timer = std::time::Instant::now();

        // clear buffer
        for i in buffer.iter_mut() {
            *i = BACKGROUND;
        }

        // game logic here
        player1.update(&mut buffer, window.is_key_down(Key::W), window.is_key_down(Key::S));
        player2.update(&mut buffer, window.is_key_down(Key::Up), window.is_key_down(Key::Down));
        ball.update(&mut buffer, &mut scores);

        // check player - ball collision
        if ball.x == player1.x + 1 && ball.y >= player1.y && ball.y <= player1.y + PLAYER_HEIGHT {
            ball.bounce();
        }
        else if ball.x == player2.x - 1 && ball.y >= player2.y && ball.y <= player2.y + PLAYER_HEIGHT {
            ball.bounce();
        }

        // display fps
        let elapsed = timer.elapsed().as_millis() as f32;
        let fps = 1.0 / elapsed * 1000.0;
        println!("FPS: {}", fps);

        // update window
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
