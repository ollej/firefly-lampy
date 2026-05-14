use firefly_rust::{Peer, Point};

#[derive(Debug, Copy, Clone)]
pub struct AttractionTarget {
    pub peer: Peer,
    pub point: Point,
}

impl AttractionTarget {
    pub fn new(peer: Peer, point: Point) -> Self {
        Self { peer, point }
    }
}
