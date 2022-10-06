use cgmath::Point3;
use crate::scenes::instance::Instance;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefIter<'a> {
    oc_tree: &'a super::OcTree,
    counter: AabbCounter,
}

impl<'a> RefIter<'a> {
    pub(super) fn new(oc_tree: &'a super::OcTree) -> Self {
        Self {
            counter: AabbCounter::new(oc_tree.__scope__),
            oc_tree,
        }
    }
}

impl<'a> Iterator for RefIter<'a> {
    type Item = Instance;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(pos) = self.counter.next() {
                if let Some(ans) = self.oc_tree.get(pos) {
                    return Some(ans);
                } else {
                    continue;
                }
            } else {
                return None;
            }
        }
    }
}

/// pos iterator for 3d AABB-box
/// # Example
/// ```
/// # use cgmath::Point3;
/// let mut ac = AabbCounter::new(1);
///
/// assert_eq!(ac.next(), Some(Point3 { x: -1, y: -1, z: -1 }));
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
                x: -(scope as i32) - 1,
                y: -(scope as i32),
                z: -(scope as i32),
            },
        }
    }

    pub(super) fn scope(&self) -> usize {
        self.scope
    }

    pub(super) fn pos(&self) -> Point3<i32> {
        self.pos
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
