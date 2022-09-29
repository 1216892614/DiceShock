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

    /// left handed position
    /// - self at front of the other
    ///     - +1
    /// - self at right of the other
    ///     - +2
    /// - self at above of the other
    ///     - +4
    pub fn toward(&self, other: cgmath::Point3<i32>) -> usize {
        super::toward(self.pos(), other)
    }
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            position: cgmath::Point3 { x: 0, y: 0, z: 0 },
            style_id: "".to_owned(),
        }
    }
}
