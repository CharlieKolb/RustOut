//! Game board logic.

/*

ToDo:

Switch from rect-rect intersection to vector rect intersection for collision detection

*/

use crate::math;
use math::{ Vec2, Segment };

use std::f64;

#[derive(PartialEq)]
pub enum Direction {
    Idle,
    Left,
    Right,
}

pub enum CollisionType {
    Wall,
    Movable,
    Block,
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

    fn make_segments(&self) -> [Segment; 4] {
        [
            Segment(self.position, self.position + self.dimension.with_y(0.)),
            Segment(self.position, self.position + self.dimension.with_x(0.)),
            Segment(self.position.with_y(0.), self.position + self.dimension),
            Segment(self.position.with_x(0.), self.position + self.dimension),
        ]
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
    pub fn get_intersecting_segments(&self, segment: &Segment) -> Vec<(Segment, Vec2)> {
        self.make_segments()
            .into_iter()
            .filter_map(|&candidate| {
                match math::segment_segment_distance(&candidate, &segment) {
                    math::HitOrDistance::Hit(point) => Some((candidate, point)),
                    _ => None,
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

    pub fn tick_segment(&self) -> Segment {
        Segment(self.prev_position, self.hitbox.position)
    }
}

trait GameObject {
    fn get_prev_position(&self) -> &Vec2;
    fn get_body(&mut self) -> &mut Body;
    fn update(&mut self, delta: f64);

    fn collision_type(&self) -> CollisionType;
    fn on_collision(&mut self, other: &mut GameObject);

    fn despawn(&mut self);
}

pub struct Player {
    pub body: Body,
    pub direction: Direction,
}

pub struct Ball {
    pub body: Body,
}

// ToDo: Duplicate code; how to propagate the intersection as option? Probably use with fold to find the closest one instead?
#[allow(dead_code)]
fn get_shortest_distance_segment(
    point: &Vec2,
    segments: &Vec<Segment>,
) -> Option<Segment> {
    let mut shortest_distance = f64::INFINITY;
    let mut result = None;
    for &s in segments {
        let d = math::point_segment_distance(&point, &s);
        if d < shortest_distance {
            shortest_distance = d;
            result = Some(s);
        }
    }

    result
}

// Returns None if there are no segments in the list
fn get_shortest_distance_segment_with_intersection(
    point: &Vec2,
    segments_with_intersection: &Vec<(Segment, Vec2)>,
) -> Option<(Segment, Vec2)> {
    let mut shortest_distance = f64::INFINITY;
    let mut result = None;
    for &tup in segments_with_intersection {
        let d = math::point_segment_distance(&point, &tup.0);
        if d < shortest_distance {
            shortest_distance = d;
            result = Some(tup);
        }
    }

    result
}

fn get_first_collision_helper(body: &Body, rect: &Rectangle) -> Option<(Segment, Vec2)> {
    let width_vec = Vec2::new(body.hitbox.dimension.x, 0.);
    let height_vec = Vec2::new(0., body.hitbox.dimension.y);
    let body_segment = body.tick_segment();

    get_shortest_distance_segment_with_intersection(
        &body.prev_position,
        &[Vec2::zero(), width_vec, height_vec, body.hitbox.dimension]
            .into_iter()
            .flat_map(|&v| rect.get_intersecting_segments(&(body_segment + v)))
            .collect()
    )
}

fn get_first_collision_point(body: &Body, rect: &Rectangle) -> Option<Vec2> {
    get_first_collision_helper(&body, &rect).map(|(_, point)| point)
}

fn get_first_colliding_segment(body: &Body, rect: &Rectangle) -> Option<Segment> {
    get_first_collision_helper(&body, &rect).map(|(seg, _)| seg)
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
                if let Some(point) = get_first_collision_point(&self.body, &other.get_body().hitbox)
                {
                    match self.direction {
                        Direction::Left => {
                            self.body.prev_position = self.body.hitbox.position;
                            self.body.hitbox.position.x = point.x;
                        }
                        Direction::Right => {
                            self.body.prev_position = self.body.hitbox.position;
                            self.body.hitbox.position.x = point.x - self.body.hitbox.dimension.x;
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

    fn despawn(&mut self) {}
}

impl GameObject for Ball {
    fn get_prev_position(&self) -> &Vec2 {
        &self.body.prev_position
    }

    fn get_body(&mut self) -> &mut Body {
        &mut self.body
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
                get_first_colliding_segment(&self.body, &other.get_body().hitbox);

                if let Some(Segment(w1, w2)) = colliding_wall_opt {
                    self.body.velocity = self.body.velocity.reflect_on(&(w2 - w1).get_closer_normal(&self.body.prev_position));
                }
            }
            CollisionType::Block => {
                let colliding_wall_opt = get_first_colliding_segment(&self.body, &other.get_body().hitbox);

                if let Some(Segment(w1, w2)) = colliding_wall_opt {
                    self.body.velocity = self.body.velocity.reflect_on(&(w2 - w1).get_closer_normal(&self.body.prev_position));

                    other.despawn();
                }
            }
        }
    }

    fn despawn(&mut self) {}
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

    fn despawn(&mut self) {}
}

pub struct Block {
    pub body: Body,
    pub color: Color
}

impl Block {
    fn new(position: Vec2, dimension: Vec2, color: Color) -> Self {
        Self {
            body: Body::new(
                Rectangle::new(position.x, position.y, dimension.x, dimension.y),
                Vec2::zero(),
            ),
            color,
        }
    }

    fn make_factory(dimension: Vec2, color: Color) -> impl Fn(Vec2) -> Self {
        move |position: Vec2| Self::new(position, dimension, color)
    } 
}

impl GameObject for Block {
    fn get_prev_position(&self) -> &Vec2 {
        &self.body.prev_position
    }

    fn get_body(&mut self) -> &mut Body {
        return &mut self.body;
    }

    fn update(&mut self, _: f64) {}

    fn collision_type(&self) -> CollisionType {
        CollisionType::Block
    }

    fn on_collision(&mut self, _: &mut GameObject) {}

    fn despawn(&mut self) {
        self.body.hitbox.position = Vec2::new(-1000., -1000.)
    }
}

type Color = [u8; 4];

enum ColorSettings {
    Single(Color),
    PerRow(Vec<Color>),
    PerBlock(Vec<Vec<Color>>),
}

struct BlockLayout {
    pub blocks: Vec<Block>,
}

impl BlockLayout {
    fn from_rows(start_position: Vec2, dimension: Vec2, color_settings: ColorSettings, blocks_per_row: u32, rows: u32) -> Vec<Block> {
        // Note that we maintain a matrix internally, this is to support row based manipulation in the future, otherwise we could just append instead of push new rows
        match color_settings {
            ColorSettings::Single(c) => {
                let make = Block::make_factory(dimension, c);
                let mut blocks: Vec<Vec<Block>> = Vec::new();
                
                for i in 0..rows {
                    let mut curr = Vec::new();
                    for j in 0..blocks_per_row {
                        curr.push(make(start_position + Vec2::new(j as f64 * (dimension.x + 2.), i as f64 * (dimension.y + 2.))));
                    }
                    blocks.push(curr);
                }

                blocks.into_iter().flatten().collect()
            } 
            ColorSettings::PerRow(c_vec) => {
                if c_vec.len() != rows as usize {
                    println!("BlockLayout::from_rows called with PerRow color setting with inconsistent amount of rows for color vector and rows input");
                    return Vec::new();
                }

                let mut blocks: Vec<Vec<Block>> = Vec::new();


                for (i, c) in c_vec.into_iter().enumerate() {
                    let mut curr = Vec::new();
                    let make = Block::make_factory(dimension, c);
                    for j in 0..blocks_per_row {
                        curr.push(make(start_position + Vec2::new(j as f64 * dimension.x, i as f64 * dimension.y)));
                    }
                    blocks.push(curr);
                }

                blocks.into_iter().flatten().collect()
            }
            _ => {
                println!("BlockLayout::from_rows called with a ColorSetting which is not yet supported");
                Vec::new()
            }
        }
    }
}

/// Stores game board information.
pub struct Gameboard {
    pub player: Player,
    pub ball: Ball,
    pub walls: [Wall; 3],
    pub blocks: Vec<Block>,
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
            blocks: BlockLayout::from_rows(Vec2::new(20., 20.), Vec2::new(40., 10.), ColorSettings::Single([255, 255, 0, 255]), 8, 10),
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

        for block in &mut self.blocks {
            let before = self.ball.body.velocity;
            if self.ball.body.hitbox.intersects(&block.body.hitbox) {
                self.ball.on_collision(block)
            }
            // Only ever hit one ball per tick
            if before != self.ball.body.velocity {
                break;
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
            Segment(Vec2::new(5., 5.), Vec2::new(4., 4.)),
            Segment(Vec2::new(5., 5.), Vec2::new(6., 6.)),
        ];
        let result = get_shortest_distance_segment(&point, &segments);
        assert_eq!(result, Some(Segment(Vec2::new(5., 5.), Vec2::new(4., 4.))));
    }
}
