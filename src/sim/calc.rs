use rand::Rng;
use super::def::{Point, WorldEdge, WORLD_HEIGHT_BOUND, WORLD_WIDTH_BOUND, PARTICLE_RADIUS};


pub fn check_out_of_bounds(pos: &Point) -> Option<WorldEdge> {
    if !is_out_of_bounds(pos) {
        return None;
    }

    let left_edge_out_distance = if pos.x < PARTICLE_RADIUS {Some(PARTICLE_RADIUS - pos.x)} else {None};
    let bottom_edge_out_distance = if pos.y > WORLD_HEIGHT_BOUND {Some(pos.y - WORLD_HEIGHT_BOUND)} else {None};
    let top_edge_out_distance = if pos.y < PARTICLE_RADIUS {Some(PARTICLE_RADIUS - pos.y)} else {None};

    let mut curr = (WorldEdge::Right, f32::MAX);

    if let Some(d) = left_edge_out_distance {
        if d < curr.1 {
            curr = (WorldEdge::Left, d)
        }
    }

    if let Some(d) = bottom_edge_out_distance {
        if d < curr.1 {
            curr = (WorldEdge::Bottom, d)
        }
    }

    if let Some(d) = top_edge_out_distance {
        if d < curr.1 {
            curr = (WorldEdge::Top, d)
        }
    }

    Some(curr.0)
}

pub fn is_out_of_bounds(pos: &Point) -> bool {
    pos.x > WORLD_WIDTH_BOUND || pos.x < PARTICLE_RADIUS
        || pos.y > WORLD_HEIGHT_BOUND || pos.y < PARTICLE_RADIUS
}

pub fn random_position() -> Point {
    let mut rng = rand::rng();
    Point::new(rng.random_range(0.0..WORLD_WIDTH_BOUND), rng.random_range(0.0..WORLD_HEIGHT_BOUND))
}

pub fn float_min(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

pub fn float_max(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}
