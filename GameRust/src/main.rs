use macroquad::prelude::*;

struct Player {
    rect: Rect,
    speed: f32,
}

impl Player {
    fn new() -> Self {
        Self {
            rect: Rect::new(100.0, 100.0, 32.0, 32.0),
            speed: 5.0,
        }
    }

    fn update(&mut self) {
        let mut move_x = 0.0;
        let mut move_y = 0.0;

        let keys_down = get_keys_down();

        for key in keys_down {
            match key {
                KeyCode::D => move_x += self.speed,
                KeyCode::A => move_x -= self.speed,
                KeyCode::S => move_y += self.speed,
                KeyCode::W => move_y -= self.speed,
                _ => {} 
            }
        }

        self.rect.x += move_x;
        self.rect.y += move_y;
    }

    fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE);
    }
}

#[macroquad::main("Jogo 2D Simples")]
async fn main() {
    let mut player = Player::new();

    loop {
        clear_background(DARKGRAY);

        player.update();
        player.draw();

        // Mostra FPS no canto
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 20.0, WHITE);

        next_frame().await;
    }
}