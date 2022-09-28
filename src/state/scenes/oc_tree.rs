use cgmath::Point3;
use std::rc::Rc;

use super::{aabb::AABB, instance::Instance};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OcTree {
    value: Node,
    len: usize,
    scope: usize,
}

impl OcTree {
    pub fn new() -> Self {
        Self {
            value: Node::default(),
            len: 0,
            scope: 0,
        }
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

    fn insert(&mut self, v: Instance, last_central: Point3<i32>, scope: usize) {
        match self {
            Node::Trunk(t) => t.insert(scope, v),
            Node::Leaf(Some(vo)) => {
                *self = Node::Trunk(Trunk::new(last_central, scope, (vo.clone(), v)))
            }
            Node::Leaf(None) => *self = Node::Leaf(Some(v)),
        }
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
    fn new(last_central: Point3<i32>, scope: usize, v: (Instance, Instance)) -> Self {
        let central = Self::__get_new_central__(last_central, scope, v.0.pos());
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

        let v0t = v.0.toword(central);
        let v1t = v.1.toword(central);

        if v0t == v1t {
            let mut node = Box::from(Node::default());

            node.insert(v.0, central, scope);
            node.insert(v.1, central, scope);

            branches[v0t] = Box::from(node);
        } else {
            branches[v0t] = Box::from(Node::new(v.0));
            branches[v1t] = Box::from(Node::new(v.1));
        }

        Self { central, branches }
    }

    fn insert(&mut self, scope: usize, v: Instance) {
        self.branches[v.toword(self.central)].insert(v, self.central, scope)
    }

    fn __get_new_central__(
        last_central: Point3<i32>,
        scope: usize,
        vo_pos: Point3<i32>,
    ) -> Point3<i32> {
        todo!()
    }
}
