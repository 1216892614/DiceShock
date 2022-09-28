use cgmath::Point3;

/// (big one, small one)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AABB(Point3<i32>, Point3<i32>);

impl AABB {
    pub fn new(ax: i32, bx: i32, ay: i32, by: i32, az: i32, bz: i32) -> Self {
        Self(
            Point3 {
                x: ax.max(bx),
                y: ay.max(by),
                z: az.max(bz),
            },
            Point3 {
                x: ax.min(bx),
                y: ay.min(by),
                z: az.min(bz),
            },
        )
    }

    pub fn size(&self) -> usize {
        ((self.0.x - self.1.x) * (self.0.y - self.1.y) * (self.0.z - self.1.z)).abs() as usize
    }
}

impl Default for AABB {
    fn default() -> Self {
        Self(Point3 { x: 0, y: 0, z: 0 }, Point3 { x: 0, y: 0, z: 0 })
    }
}
