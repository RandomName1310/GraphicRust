use macroquad::prelude::*;

struct Player {
    rect: Rect,
    vel_x: f32,
    vel_y: f32,
    speed: f32,
    jump_height: f32,
    is_falling: bool,
}

struct World {
    platforms: Vec<Rect>,
}

impl Player {
    fn new() -> Self {
        Self {
            rect: Rect::new(100.0, 100.0, 32.0, 32.0),
            vel_x: 0.0,
            vel_y: 0.0,
            speed: 4.0,
            jump_height: 6.0,
            is_falling: false,
        }
    }

    fn update(&mut self, platforms: &[Rect]) {
        let vel_x = &mut self.vel_x;
        let vel_y = &mut self.vel_y;

        const GRAVITY: f32 = -0.15;
        const AIRFRICTION: f32 = -0.2;

        // draw line
        let line_begin: Vec2 =  vec2(self.rect.x + self.rect.w / 2.0, self.rect.y + self.rect.h / 2.0);
        let line_end: Vec2 = mouse_position().into();

        draw_line(line_begin.x, line_begin.y, line_end.x, line_end.y, 1.0, RED);

        // check if falling
        self.is_falling = true;
        for platform in platforms{
            if self.rect.overlaps(platform){
                self.is_falling = false;
            }
        }

        // apply accelerations
        if *vel_x > 0.0 && *vel_x < 1.0 {*vel_x = 0.0}
        if *vel_x >= 0.0{*vel_x += AIRFRICTION}
        if *vel_x < 0.0{*vel_x -= AIRFRICTION}

        if self.is_falling{*vel_y += GRAVITY} else{*vel_y = 0.0};

        // movement logic
        let keys_down = get_keys_down();

        for key in keys_down {
            match key {
                KeyCode::D => *vel_x = self.speed,
                KeyCode::A => *vel_x = -self.speed,
                KeyCode::W => if !self.is_falling{*vel_y = self.jump_height},
                _ => {} 
            }
        }
        
        // apply changes to player rect
        self.rect.x += *vel_x;
        self.rect.y -= *vel_y;
        // debug
        draw_text(&format!("X: {}", vel_x), 20.0, 20.0, 20.0, WHITE);
        draw_text(&format!("Y: {}", vel_y), 20.0, 40.0, 20.0, WHITE);
        draw_text(&format!("Is Falling: {}", self.is_falling), 20.0, 60.0, 20.0, WHITE);
    }

    fn draw(&self) {
        // draw player
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE);
    }
}

impl World {
    fn new() -> Self {
        Self {
            platforms: vec![
                Rect::new(200.0, 450.0, 100.0, 20.0),
                Rect::new(400.0, 350.0, 100.0, 20.0),
                Rect::new(-200.0, 550.0, 1000.0, 100.0),
            ],
        }
    }

    fn update(&mut self) {
    }

    fn draw(&self) {
        for platform in &self.platforms {
            draw_rectangle(platform.x, platform.y, platform.w, platform.h, GREEN);
        }
    }
}

#[macroquad::main("Jogo 2D Simples")]
async fn main() {
    let mut player = Player::new();
    let world = World::new();

    loop {
        clear_background(DARKGRAY);

        // handle world
        world.draw();

        // handle player
        player.update(&world.platforms);
        player.draw();

        next_frame().await;
    }
}