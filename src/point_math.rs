use firefly_rust::{Angle, Point, math};

pub trait PointMath {
    fn angle_to(&self, other: &Point) -> Angle;
    fn distance(&self, other: &Point) -> f32;
    fn point_from_distance_and_angle(&self, distance: f32, angle: Angle) -> (Point, f32);
}

impl PointMath for Point {
    fn distance(&self, other: &Point) -> f32 {
        math::sqrt(((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f32)
    }

    fn angle_to(&self, other: &Point) -> Angle {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let theta = math::atan2(dy as f32, dx as f32);
        Angle::from_radians(theta)
    }

    fn point_from_distance_and_angle(&self, distance: f32, angle: Angle) -> (Point, f32) {
        let xx = self.x as f32 + (distance * math::cos(angle.to_radians()));
        let yy = self.y as f32 + (distance * math::sin(angle.to_radians()));

        let new_point = Point {
            x: math::floor(xx) as i32,
            y: math::floor(yy) as i32,
        };

        let remainder = distance - self.distance(&new_point);

        (new_point, remainder)
    }
}

#[cfg(test)]
mod tests {
    use crate::point_math::*;
    use firefly_rust::Point;

    #[test]
    fn angle_to() {
        let p1 = Point { x: 10, y: 10 };
        let p2 = Point { x: 20, y: 20 };
        let angle = p1.angle_to(&p2);
        assert_eq!(angle.to_degrees(), 45.0);
    }
}
