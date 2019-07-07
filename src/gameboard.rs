//! Game board logic.

#[derive(PartialEq)]
pub enum Direction {
    Idle,
    Left,
    Right,
}

trait GameObject {
    fn update(&mut self, delta: f64);
}

pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rectangle {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self { x, y, w, h }
    }

    pub fn intersects(&mut self, o: &Rectangle) -> bool {
        if self.x + self.w < o.x
            || self.x > o.x + o.w
            || self.y + self.h < o.y
            || self.y > o.y + o.h
        {
            return false;
        }

        return true;
    }
}

pub struct Vec2 {
    pub dx: f64,
    pub dy: f64,
}

impl Vec2 {
    pub fn new(dx: f64, dy: f64) -> Self {
        Self { dx, dy }
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(f64::powf(self.dx, 2.0) + f64::powf(self.dy, 2.0))
    }

    pub fn set_length(mut self, new_length: f64) -> Self {
        let len = self.length();
        self.dx = self.dx / len * new_length;
        self.dy = self.dy / len * new_length;
        self
        
    }
}

pub struct Body {
    pub hitbox: Rectangle,
    pub velocity: Vec2,
}

impl Body {
    pub fn new(hitbox: Rectangle, velocity: Vec2) -> Self {
        Self { hitbox, velocity }
    }

    pub fn apply_velocity(&mut self, delta: f64) {
        self.hitbox.x += self.velocity.dx * delta;
        self.hitbox.y += self.velocity.dy * delta;
    }
}

pub struct Player {
    pub body: Body,
    pub direction: Direction,
}

impl GameObject for Player {
    fn update(&mut self, delta: f64) {
        match self.direction {
            Direction::Left => {
                self.body.velocity.dx = -f64::abs(self.body.velocity.dx);
                self.body.apply_velocity(delta);
            }
            Direction::Right => {
                self.body.velocity.dx = f64::abs(self.body.velocity.dx);
                self.body.apply_velocity(delta);
            }
            Direction::Idle => (),
        }
    }
}

pub struct Ball {
    pub body: Body,
}

impl GameObject for Ball {
    fn update(&mut self, delta: f64) {
        self.body.apply_velocity(delta)
    }
}

/// Stores game board information.
pub struct Gameboard {
    pub player: Player,
    pub ball: Ball,
    pub size: f64,
}

impl Gameboard {
    /// Creates a new game board.
    pub fn new(size: f64) -> Self {
        Self {
            player: Player {
                body: Body {
                    hitbox: Rectangle::new(250.0, 350.0, 100.0, 15.0),
                    velocity: Vec2::new(100.0, 0.0),
                },
                direction: Direction::Idle,
            },
            ball: Ball {
                body: Body {
                    hitbox: Rectangle::new(295.0, 250.0, 10.0, 10.0),
                    velocity: Vec2::new(20.0, 150.0),
                },
            },
            size,
        }
    }

    pub fn update(&mut self, delta: f64) {
        self.player.update(delta);
        self.ball.update(delta);

        let ball_body = &mut self.ball.body;

        if ball_body.hitbox.intersects(&self.player.body.hitbox) {
            let half_width = self.player.body.hitbox.w / 2.0;

            // -1 for left edge, 0 for middle, 1 for right edge
            let scaled = (f64::abs(ball_body.hitbox.x + ball_body.hitbox.w/2.0 - self.player.body.hitbox.x) / half_width) - 1.0;

            ball_body.velocity = Vec2::new(scaled, -1.0).set_length(ball_body.velocity.length());
        }
        else if ball_body.hitbox.y <= 0.0 {
            ball_body.velocity.dy *= -1.025;
        }
        else if  ball_body.hitbox.x < 0.0 || ball_body.hitbox.x + ball_body.hitbox.w > self.size {
            ball_body.velocity.dx *= -1.025;
        }
    }
}
