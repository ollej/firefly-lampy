use firefly_rust::{math, Angle, Point};

pub trait PointMath {
    fn angle_to(&self, other: &Point) -> Angle;
    fn distance(&self, other: &Point) -> f32;
    fn point_from_distance_and_angle(&self, distance: f32, angle: Angle) -> Point;
}

impl PointMath for Point {
    fn distance(&self, other: &Point) -> f32 {
        math::sqrt(((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f32)
    }

    fn angle_to(&self, other: &Point) -> Angle {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let theta = math::atan2(dy as f32, dx as f32);
        Angle::from_degrees(theta)
    }

    fn point_from_distance_and_angle(&self, distance: f32, angle: Angle) -> Point {
        let xx = self.x as f32 + (distance * math::cos(angle.to_radians()));
        let yy = self.y as f32 + (distance * math::sin(angle.to_radians()));

        Point {
            x: math::floor(xx) as i32,
            y: math::floor(yy) as i32,
        }
    }
}
