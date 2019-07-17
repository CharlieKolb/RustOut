//! Game board logic.

/*

ToDo:

Switch from rect-rect intersection to vector rect intersection for collision detection

*/

use crate::vec2;
use vec2::Vec2;

use std::f64;

#[derive(PartialEq)]
pub enum Direction {
    Idle,
    Left,
    Right,
}

pub enum CollisionType {
    Player,
    Wall,
    Movable,
}

#[derive(Debug)]
pub struct Rectangle {
    pub position: Vec2,
    pub dimension: Vec2,
}

impl Rectangle {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self {
            position: Vec2::new(x, y),
            dimension: Vec2::new(w, h),
        }
    }

    pub fn intersects(&self, o: &Rectangle) -> bool {
        let Vec2 { x, y } = self.position;
        let Vec2 { x: w, y: h } = self.dimension;

        let Vec2 { x: ox, y: oy } = o.position;
        let Vec2 { x: ow, y: oh } = o.dimension;

        if x + w < ox || x > ox + ow || y + h < oy || y > oy + oh {
            return false;
        }

        return true;
    }

    // Return intersecting segments and the intersecting points
    pub fn get_intersecting_segments(&self, start: &Vec2, end: &Vec2) -> Vec<((Vec2, Vec2), Vec2)> {
        vec![
            (
                self.position.clone(),
                self.position + Vec2::zero().set_x(self.dimension.x),
            ),
            (
                self.position.clone(),
                self.position + Vec2::zero().set_y(self.dimension.y),
            ),
            (
                self.position + Vec2::zero().set_x(self.dimension.x),
                self.position + self.dimension,
            ),
            (
                self.position + Vec2::zero().set_y(self.dimension.y),
                self.position + self.dimension,
            ),
        ]
        .into_iter()
        .filter_map(|(p1, p2)| {
            let (_, point_opt) = vec2::segment_segment_distance(&p1, &p2, start, end);
            match point_opt {
                Some(point) => Some(((p1, p2), point)),
                None => None,
            }
        })
        .collect()
    }
}

pub struct Body {
    pub prev_position: Vec2,
    pub hitbox: Rectangle,
    pub velocity: Vec2,
}

impl Body {
    pub fn new(hitbox: Rectangle, velocity: Vec2) -> Self {
        Self {
            prev_position: hitbox.position.clone(),
            hitbox,
            velocity,
        }
    }

    pub fn apply_velocity(&mut self, delta: f64) {
        self.prev_position = self.hitbox.position.clone();
        self.hitbox.position += self.velocity * delta;
    }
}

trait GameObject {
    fn get_prev_position(&self) -> &Vec2;
    fn get_body(&mut self) -> &mut Body;
    fn update(&mut self, delta: f64);

    fn collision_type(&self) -> CollisionType;
    fn on_collision(&mut self, other: &mut GameObject);
}

pub struct Player {
    pub body: Body,
    pub direction: Direction,
}

pub struct Ball {
    pub body: Body,
}

// ToDo: Duplicate code; how to propegate the intersection as option? Probably use with fold to find the closest one instead?
#[allow(dead_code)]
fn get_shortest_distance_segment(
    point: &Vec2,
    segments: Vec<(Vec2, Vec2)>,
) -> Option<(Vec2, Vec2)> {
    let mut shortest_distance = f64::INFINITY;
    let mut result = None;
    for (p1, p2) in segments {
        let d = vec2::point_segment_distance(&point, &p1, &p2);
        if d < shortest_distance {
            shortest_distance = d;
            result = Some((p1, p2));
        }
    }

    result
}

// Returns None if there are no segments in the list
fn get_shortest_distance_segment_with_intersection(
    point: &Vec2,
    segments_with_intersection: Vec<((Vec2, Vec2), Vec2)>, // Vec of (segment, intersection_point)
) -> Option<((Vec2, Vec2), Vec2)> {
    let mut shortest_distance = f64::INFINITY;
    let mut result = None;
    for ((p1, p2), intersect) in segments_with_intersection {
        let d = vec2::point_segment_distance(&point, &p1, &p2);
        if d < shortest_distance {
            shortest_distance = d;
            result = Some(((p1, p2), intersect));
        }
    }

    result
}

fn get_first_colliding_wall(body: &Body, wall: &Rectangle) -> Option<((Vec2, Vec2), Vec2)> {
    let width_vec = Vec2::zero().set_x(body.hitbox.dimension.x);
    let height_vec = Vec2::zero().set_y(body.hitbox.dimension.y);

    let mut segments = wall.get_intersecting_segments(&body.prev_position, &body.hitbox.position);
    segments.append(&mut wall.get_intersecting_segments(
        &(body.prev_position + width_vec),
        &(body.hitbox.position + width_vec),
    ));
    segments.append(&mut wall.get_intersecting_segments(
        &(body.prev_position + height_vec),
        &(body.hitbox.position + height_vec),
    ));
    segments.append(&mut wall.get_intersecting_segments(
        &(body.prev_position + body.hitbox.dimension),
        &(body.hitbox.position + body.hitbox.dimension),
    ));
    get_shortest_distance_segment_with_intersection(&body.prev_position, segments)
}

impl GameObject for Player {
    fn get_prev_position(&self) -> &Vec2 {
        &self.body.prev_position
    }

    fn get_body(&mut self) -> &mut Body {
        return &mut self.body;
    }

    fn update(&mut self, delta: f64) {
        match self.direction {
            Direction::Left => {
                self.body.velocity.x = -f64::abs(self.body.velocity.x);
                self.body.apply_velocity(delta);
            }
            Direction::Right => {
                self.body.velocity.x = f64::abs(self.body.velocity.x);
                self.body.apply_velocity(delta);
            }
            Direction::Idle => (),
        }
    }

    fn collision_type(&self) -> CollisionType {
        CollisionType::Wall
    }

    fn on_collision(&mut self, other: &mut GameObject) {
        match other.collision_type() {
            CollisionType::Wall => {
                let colliding_wall_opt =
                    get_first_colliding_wall(&self.body, &other.get_body().hitbox);

                if let Some((_, point)) = colliding_wall_opt
                {
                    match self.direction {
                        Direction::Left => {
                            self.body.prev_position = self.body.hitbox.position;
                            self.body.hitbox.position = self.body.hitbox.position.set_x(point.x);
                        }
                        Direction::Right => {
                            self.body.prev_position = self.body.hitbox.position;
                            self.body.hitbox.position = self
                                .body
                                .hitbox
                                .position
                                .set_x(point.x - self.body.hitbox.dimension.x);
                        }
                        _ => (),
                    }
                }
            }
            CollisionType::Movable => {
                // Bounce back objects hitting the player at set angles
                let ref mut other_body = other.get_body();

                let half_width = self.body.hitbox.dimension.x / 2.0;

                // -1 for left edge, 0 for middle, 1 for right edge
                let scaled = f64::abs(
                    other_body.hitbox.position.x + other_body.hitbox.dimension.x / 2.0
                        - self.body.hitbox.position.x,
                ) / half_width
                    - 1.0;

                other_body.velocity =
                    Vec2::new(scaled, -1.0).set_length(other_body.velocity.length());
            }
            _ => (),
        }
    }
}

impl GameObject for Ball {
    fn get_prev_position(&self) -> &Vec2 {
        &self.body.prev_position
    }

    fn get_body(&mut self) -> &mut Body {
        return &mut self.body;
    }

    fn update(&mut self, delta: f64) {
        self.body.apply_velocity(delta)
    }

    fn collision_type(&self) -> CollisionType {
        CollisionType::Movable
    }

    fn on_collision(&mut self, other: &mut GameObject) {
        match other.collision_type() {
            CollisionType::Wall | CollisionType::Movable => {
                let colliding_wall_opt =
                    get_first_colliding_wall(&self.body, &other.get_body().hitbox);

                if let Some(((w1, w2), _)) = colliding_wall_opt {
                    let wall_vec = w2 - w1;
                    let closer_normal = wall_vec.get_closer_normal(&self.body.prev_position);
                    self.body.velocity = self.body.velocity.reflect_on(&closer_normal);
                }
            }
            CollisionType::Player => {
                // Player handles collision
            }
        }
    }
}

pub struct Wall {
    pub body: Body,
}

impl GameObject for Wall {
    fn get_prev_position(&self) -> &Vec2 {
        &self.body.prev_position
    }

    fn get_body(&mut self) -> &mut Body {
        return &mut self.body;
    }

    fn update(&mut self, _: f64) {}

    fn collision_type(&self) -> CollisionType {
        CollisionType::Wall
    }

    fn on_collision(&mut self, _: &mut GameObject) {}
}

/// Stores game board information.
pub struct Gameboard {
    pub player: Player,
    pub ball: Ball,
    pub walls: [Wall; 3],
    pub size: f64,
}

impl Gameboard {
    /// Creates a new game board.
    pub fn new(size: f64) -> Self {
        Self {
            player: Player {
                body: Body::new(
                    Rectangle::new(150.0, 350.0, 100.0, 15.0),
                    Vec2::new(250.0, 0.0),
                ),
                direction: Direction::Idle,
            },
            ball: Ball {
                body: Body::new(
                    Rectangle::new(165.0, 250.0, 10.0, 10.0),
                    Vec2::new(0.0, 300.0),
                ),
            },
            walls: [
                Wall {
                    body: Body::new(Rectangle::new(-10., -10., 10., size + 10.), Vec2::zero()),
                },
                Wall {
                    body: Body::new(Rectangle::new(0., -10., size, 10.), Vec2::zero()),
                },
                Wall {
                    body: Body::new(Rectangle::new(size, -10., 10., size + 10.), Vec2::zero()),
                },
            ],
            size,
        }
    }

    pub fn update(&mut self, delta: f64) {
        self.player.update(delta);
        self.ball.update(delta);

        if self.player.body.hitbox.intersects(&self.ball.body.hitbox) {
            self.player.on_collision(&mut self.ball);
        }

        for wall in &mut self.walls {
            if self.ball.body.hitbox.intersects(&wall.body.hitbox) {
                self.ball.on_collision(wall)
            }

            if self.player.body.hitbox.intersects(&wall.body.hitbox) {
                self.player.on_collision(wall)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_shortest_distance_segment() {
        let point: Vec2 = Vec2::zero();

        let segments = vec![
            (Vec2::new(5., 5.), Vec2::new(4., 4.)),
            (Vec2::new(5., 5.), Vec2::new(6., 6.)),
        ];
        let result = get_shortest_distance_segment(&point, segments);
        assert_eq!(result, Some((Vec2::new(5., 5.), Vec2::new(4., 4.))));
    }
}
