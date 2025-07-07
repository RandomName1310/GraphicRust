use macroquad::prelude::*;

struct Player {
    rect: Rect,
    upper_hitbox: Rect,
    lower_hitbox: Rect,
    left_hitbox: Rect,
    right_hitbox: Rect,
    vel_x: f32,
    vel_y: f32,
    speed: f32,
    jump_height: f32,
    is_falling: bool,
    can_walk_left: bool,
    can_walk_right: bool,
}

struct World {
    platforms: Vec<Rect>,
}

impl Player {
    fn new() -> Self {
        Self {
            rect: Rect::new(100.0, 100.0, 32.0, 32.0),
            upper_hitbox: Rect::new(100.0, 100.0, 32.0, 32.0),
            lower_hitbox: Rect::new(100.0, 100.0, 32.0, 32.0),
            left_hitbox: Rect::new(100.0, 100.0, 32.0, 32.0),
            right_hitbox: Rect::new(100.0, 100.0, 32.0, 32.0),
            vel_x: 0.0,
            vel_y: 0.0,
            speed: 4.0,
            jump_height: 6.0,
            is_falling: false,
            can_walk_left: false,
            can_walk_right: false,
        }
    }

    fn update(&mut self, platforms: &[Rect]) {
        let vel_x = &mut self.vel_x;
        let vel_y = &mut self.vel_y;

        const GRAVITY: f32 = -0.15;
        const AIRFRICTION: f32 = -0.2;

        // check if falling
        self.is_falling = true;
        self.can_walk_left = true;
        self.can_walk_right = true;
        for platform in platforms{
            // check upper hitbox
            if self.upper_hitbox.overlaps(platform){
                *vel_y = -*vel_y * 0.8;
            }
            // check lower hitbox
            else if self.lower_hitbox.overlaps(platform){
                self.is_falling = false;
            }
            // check left hitbox
            else if self.left_hitbox.overlaps(platform){
                self.can_walk_left = false;
                *vel_x = -*vel_x;
            }
            // check right hitbox
            else if self.right_hitbox.overlaps(platform){
                self.can_walk_right = false;
                *vel_x = -*vel_x;
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
                KeyCode::D => if self.can_walk_right{*vel_x = self.speed},
                KeyCode::A => if self.can_walk_left{*vel_x = -self.speed},
                KeyCode::W => if !self.is_falling{*vel_y = self.jump_height},
                _ => {} 
            }
        }
        
        // apply changes to player rect
        self.rect.x += *vel_x;
        self.rect.y -= *vel_y;

        // recalculate hitboxes 
        self.upper_hitbox = Rect::new(
            self.rect.x + 2.0,
            self.rect.y - 3.0,
            self.rect.w - 5.0,
            self.rect.h / 5.0,
        );
        self.lower_hitbox = Rect::new(
            self.rect.x + 2.0,
            self.rect.y + self.rect.h + 1.0,
            self.rect.w - 5.0,
            self.rect.h / 5.0,
        );
        self.left_hitbox = Rect::new(
            self.rect.x - self.rect.w / 5.0 + 1.0, 
            self.rect.y + 2.0,         
            self.rect.w / 5.0,
            self.rect.h - 4.0,
        );

        self.right_hitbox = Rect::new(
            self.rect.x + self.rect.w - 1.0,
            self.rect.y + 2.0,
            self.rect.w / 5.0,
            self.rect.h - 4.0,
        );

        // dash logic
        let dash_x: f32 = 10.0;
        let dash_y: f32 = 10.0;
        let smooth: f32 = 10.0;

        let mouse_pos: Vec2 = mouse_position().into();
        let dash_vec: Vec2 = Vec2::new(((self.rect.x - mouse_pos.x)/smooth).clamp(-dash_x, dash_x), ((self.rect.y - mouse_pos.y)/smooth).clamp(-dash_y, dash_y));
        draw_text(&format!("Dash: {}, {}", dash_vec.x, dash_vec.y), 20.0, 80.0, 20.0, WHITE);

        if is_mouse_button_pressed(MouseButton::Left){
            *vel_x = -dash_vec.x;
            *vel_y = dash_vec.y;
        }

        // debug
        draw_text(&format!("X: {}", vel_x), 20.0, 20.0, 20.0, WHITE);
        draw_text(&format!("Y: {}", vel_y), 20.0, 40.0, 20.0, WHITE);
        draw_text(&format!("Is Falling: {}", self.is_falling), 20.0, 60.0, 20.0, WHITE);
    }

    fn draw(&self) {
        // draw player
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE);
        //draw_rectangle(self.upper_hitbox.x, self.upper_hitbox.y, self.upper_hitbox.w, self.upper_hitbox.h, RED);
        //draw_rectangle(self.lower_hitbox.x, self.lower_hitbox.y, self.lower_hitbox.w, self.lower_hitbox.h, PURPLE);
        //draw_rectangle(self.left_hitbox.x, self.left_hitbox.y, self.left_hitbox.w, self.left_hitbox.h, PINK);
        //draw_rectangle(self.right_hitbox.x, self.right_hitbox.y, self.right_hitbox.w, self.right_hitbox.h, PINK);

    }
}

impl World {
    fn new() -> Self {
        Self {
            platforms: vec![
                Rect::new(200.0, 450.0, 100.0, 20.0),
                Rect::new(400.0, 350.0, 100.0, 20.0),
                Rect::new(-200.0, 550.0, 1000.0, 100.0),
                Rect::new(0.0, 0.0, 100.0, 1000.0),
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