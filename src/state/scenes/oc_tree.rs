use cgmath::{Point3, Vector3};
use std::rc::Rc;

use super::{aabb::AABB, instance::Instance};

const CENTRAL: Point3<i32> = Point3 { x: 0, y: 0, z: 0 };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OcTree {
    value: Node,
    len: usize,
    scope: usize,
}

impl OcTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, v: Instance) {
        self.len += 1;

        loop {
            if self.is_in_scope(&v.pos()) {
                self.value.insert(v, CENTRAL, self.scope);
                break;
            } else {
                self.update_scope(self.scope * 2);
            }
        }
    }

    fn is_in_scope(&self, p: &Point3<i32>) -> bool {
        p.x > -(self.scope as i32)
            && p.x <= self.scope as i32
            && p.y > -(self.scope as i32)
            && p.y <= self.scope as i32
            && p.z > -(self.scope as i32)
            && p.z <= self.scope as i32
    }

    fn update_scope(&self, scope: usize) -> bool {
        todo!()
    }
}

impl Default for OcTree {
    fn default() -> Self {
        Self {
            value: Node::default(),
            len: 0,
            scope: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    Trunk(Trunk),
    Leaf(Option<Instance>),
}

impl Node {
    fn new(v: Instance) -> Self {
        Self::Leaf(Some(v))
    }

    fn insert(&mut self, v: Instance, central: Point3<i32>, offset: usize) {
        match self {
            Node::Trunk(t) => t.insert(offset, v),
            Node::Leaf(Some(vo)) => {
                *self = Node::Trunk(Trunk::new(
                    Self::__get_new_central__(central, offset, vo.toward(central)),
                    offset,
                    (vo.clone(), v),
                ))
            }
            Node::Leaf(None) => *self = Node::Leaf(Some(v)),
        }
    }

    fn __get_new_central__(last_central: Point3<i32>, offset: usize, toward: usize) -> Point3<i32> {
        let mut ans = last_central;

        if toward % 2 != 0 {
            ans.x -= offset as i32
        } else {
            ans.x += offset as i32
        };

        if toward % 4 / 2 != 0 {
            ans.y -= offset as i32
        } else {
            ans.y += offset as i32
        };

        if toward / 4 != 0 {
            ans.z -= offset as i32
        } else {
            ans.z += offset as i32
        };

        ans
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::Leaf(None)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Trunk {
    central: Point3<i32>,
    branches: [Box<Node>; 8],
}

impl Trunk {
    fn new(central: Point3<i32>, offset: usize, v: (Instance, Instance)) -> Self {
        let mut branches = [
            Box::from(Node::default()),
            Box::from(Node::default()),
            Box::from(Node::default()),
            Box::from(Node::default()),
            Box::from(Node::default()),
            Box::from(Node::default()),
            Box::from(Node::default()),
            Box::from(Node::default()),
        ];

        let v0t = v.0.toward(central);
        let v1t = v.1.toward(central);

        if v0t == v1t {
            let mut node = Box::from(Node::default());

            node.insert(v.0, central, offset);
            node.insert(v.1, central, offset);

            branches[v0t] = Box::from(node);
        } else {
            branches[v0t] = Box::from(Node::new(v.0));
            branches[v1t] = Box::from(Node::new(v.1));
        }

        Self { central, branches }
    }

    fn insert(&mut self, offset: usize, v: Instance) {
        self.branches[v.toward(self.central)].insert(v, self.central, offset / 2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_octree_works() {
        todo!()
    }
}
