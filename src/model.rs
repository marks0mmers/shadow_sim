use nannou::{
    geom::{Corner, Polygon, Range},
    prelude::*,
};

pub struct Light {
    pub pos: Vec2,
    pub radius: f32,
}

impl Light {
    pub fn new(pos: Vec2) -> Self {
        Self { pos, radius: 10. }
    }
}

trait Slope {
    fn slope_to(&self, other: Self) -> f32;
}

impl Slope for Vec2 {
    fn slope_to(&self, other: Vec2) -> f32 {
        (self.y - other.y) / (self.x - other.x)
    }
}

pub struct Model {
    pub light: Light,
    pub walls: Vec<Polygon<Vec<Vec2>>>,
    pub wall_building: Option<Vec<Vec2>>,
}

impl Model {
    pub fn outer_most_points(&self, points: &[Vec2]) -> (Vec2, Vec2) {
        let all_points_within = points.iter().all(|p| p.x > self.light.pos.x)
            || points.iter().all(|p| p.x < self.light.pos.x);

        (
            *points
                .iter()
                .filter(|p| all_points_within || p.slope_to(self.light.pos) < 0.)
                .max_by(|a, b| {
                    a.slope_to(self.light.pos)
                        .total_cmp(&b.slope_to(self.light.pos))
                })
                .unwrap(),
            *points
                .iter()
                .filter(|p| all_points_within || p.slope_to(self.light.pos) > 0.)
                .min_by(|a, b| {
                    a.slope_to(self.light.pos)
                        .total_cmp(&b.slope_to(self.light.pos))
                })
                .unwrap(),
        )
    }

    pub fn get_intersection_point(&self, point: Vec2, window_rect: Rect) -> Vec2 {
        let slope = point.slope_to(self.light.pos);
        let b = point.y - slope * point.x;
        let x = window_rect.right() * (point.x - self.light.pos.x).signum();
        let y = slope * x + b;

        if window_rect.y.contains(y) {
            vec2(x, y)
        } else if self.light.pos.y < point.y {
            vec2((window_rect.top() - b) / slope, window_rect.top())
        } else {
            vec2((window_rect.bottom() - b) / slope, window_rect.bottom())
        }
    }

    pub fn get_shadow_polygons(&self, window_rect: Rect) -> Vec<Polygon<Vec<Vec2>>> {
        self.walls
            .iter()
            .map(|wall| {
                let (left, right) = self.outer_most_points(&wall.points);
                let left_int = self.get_intersection_point(left, window_rect);
                let right_int = self.get_intersection_point(right, window_rect);
                let mut points = vec![left_int];

                if left_int.y != right_int.y && left_int.x != right_int.x {
                    if left_int.y.abs() == right_int.y.abs() {
                        let (edge_left, edge_right) = if left_int.y < right_int.y {
                            (window_rect.bottom_left(), window_rect.top_left())
                        } else {
                            (window_rect.top_right(), window_rect.bottom_right())
                        };

                        points.push(edge_left);
                        points.push(edge_right);
                    } else if left_int.x.abs() == right_int.x.abs() {
                        let (edge_left, edge_right) = if left_int.x < right_int.x {
                            (window_rect.top_left(), window_rect.top_right())
                        } else {
                            (window_rect.bottom_right(), window_rect.bottom_left())
                        };
                        points.push(edge_left);
                        points.push(edge_right);
                    } else {
                        let rect = Rect {
                            x: Range::new(left_int.x, right_int.x),
                            y: Range::new(left_int.y, right_int.y),
                        };
                        let corner = match window_rect.closest_corner(rect.xy().to_array()) {
                            Corner::TopLeft => window_rect.top_left(),
                            Corner::TopRight => window_rect.top_right(),
                            Corner::BottomLeft => window_rect.bottom_left(),
                            Corner::BottomRight => window_rect.bottom_right(),
                        };
                        points.push(corner);
                    }
                }

                points.push(right_int);
                points.push(right);
                points.push(left);

                Polygon { points }
            })
            .collect::<Vec<_>>()
    }
}
