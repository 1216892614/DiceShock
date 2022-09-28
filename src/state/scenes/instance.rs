#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instance {
    position: cgmath::Point3<i32>,
    style_id: String,
}

impl Instance {
    pub fn new(position: cgmath::Point3<i32>, style_id: String) -> Self {
        Self { position, style_id }
    }

    pub fn pos(&self) -> cgmath::Point3<i32> {
        self.position
    }

    pub fn toword(&self, other: cgmath::Point3<i32>) -> usize {
        let mut ans = 0;

        if self.position.x >= other.x {
            ans += 1
        };
        if self.position.y >= other.y {
            ans += 2
        };
        if self.position.z >= other.z {
            ans += 4
        };

        ans
    }
}
