use super::{Circle, Line, Point2D, Polygon};

pub trait Shape2D {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
    fn vertex_count(&self) -> usize;
    fn contains(&self, point: &Point2D) -> bool;
    fn intersects_circle(&self, circle: &Circle) -> bool;
    fn intersects_line(&self, line: &Line) -> bool;
    fn intersects_polygon(&self, polygon: &Polygon) -> bool;
}
