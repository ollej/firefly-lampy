use firefly_rust::Point;

pub struct Rectangle {
    pub point: Point,
    pub width: i32,
    pub height: i32,
}

impl Rectangle {
    pub fn new(point: Point, width: i32, height: i32) -> Self {
        Self {
            point,
            width,
            height,
        }
    }

    pub fn x(&self) -> i32 {
        self.point.x
    }

    pub fn y(&self) -> i32 {
        self.point.y
    }

    pub fn bottom_right(&self) -> Point {
        Point {
            x: self.point.x + self.width - 1,
            y: self.point.y + self.height - 1,
        }
    }
}
