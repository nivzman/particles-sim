use rand::Rng;
use super::def::{Point, WorldEdge, WORLD_HEIGHT_FLOAT, WORLD_WIDTH_FLOAT};


pub fn check_out_of_bounds(pos: &Point) -> Option<WorldEdge> {
    if !is_out_of_bounds(pos) {
        return None;
    }

    let left_edge_out_distance = if pos.x < 0. {Some(-pos.x)} else {None};
    let bottom_edge_out_distance = if pos.y > WORLD_HEIGHT_FLOAT {Some(pos.y - WORLD_HEIGHT_FLOAT)} else {None};
    let top_edge_out_distance = if pos.y < 0. {Some(-pos.y)} else {None};

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
    pos.x > WORLD_WIDTH_FLOAT || pos.x < 0.
        || pos.y > WORLD_HEIGHT_FLOAT || pos.y < 0.
}

pub fn random_position() -> Point {
    let mut rng = rand::rng();
    Point::new(rng.random_range(0.0..WORLD_WIDTH_FLOAT), rng.random_range(0.0..WORLD_HEIGHT_FLOAT))
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
