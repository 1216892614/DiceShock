use cgmath::Point3;

/// pos iterator for 3d AABB-box
/// # Example
/// ```
/// let mut ac = super::AabbCounter::new(1);
/// 
/// assert_eq!(ac.next(), Some(Point3 { x: 0, y: -1, z: -1 }));
/// assert_eq!(ac.next(), Some(Point3 { x: -1, y: 0, z: -1 }));
/// assert_eq!(ac.next(), Some(Point3 { x: 0, y: 0, z: -1 }));
/// assert_eq!(ac.next(), Some(Point3 { x: -1, y: -1, z: 0 }));
/// assert_eq!(ac.next(), Some(Point3 { x: 0, y: -1, z: 0 }));
/// assert_eq!(ac.next(), Some(Point3 { x: -1, y: 0, z: 0 }));
/// assert_eq!(ac.next(), Some(Point3 { x: 0, y: 0, z: 0 }));
/// assert_eq!(ac.next(), None);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct AabbCounter {
    scope: usize,
    pos: Point3<i32>,
}

impl AabbCounter {
    pub(super) fn new(scope: usize) -> Self {
        Self {
            scope,
            pos: Point3 {
                x: -(scope as i32),
                y: -(scope as i32),
                z: -(scope as i32),
            },
        }
    }

    pub(super) fn scope(&self) -> usize {
        self.scope
    }
}

impl Default for AabbCounter {
    fn default() -> Self {
        Self {
            scope: 0,
            pos: Point3 { x: 0, y: 0, z: 0 },
        }
    }
}

impl Iterator for AabbCounter {
    type Item = Point3<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut pos = Point3::new(self.pos.x + 1, self.pos.y, self.pos.z);

        if pos.x >= self.scope as i32 {
            pos = Point3::new(-(self.scope as i32), self.pos.y + 1, self.pos.z)
        };

        if pos.y >= self.scope as i32 {
            pos = Point3::new(-(self.scope as i32), -(self.scope as i32), self.pos.z + 1)
        };

        if pos.z >= self.scope as i32 {
            return None;
        };

        self.pos = pos;
        Some(pos)
    }
}
