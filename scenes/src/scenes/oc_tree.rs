use super::instance::Instance;
use cgmath::Point3;

pub mod iter;

const CENTRAL: Point3<i32> = Point3 { x: 0, y: 0, z: 0 };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OcTree {
    __value__: Node,
    __len__: usize,
    __scope__: usize,
}

impl OcTree {
    /// Create a OcTree with a sized AABB-Box
    ///
    /// # Performance
    /// ```ignore
    /// // Create a new OcTree with scope as 4
    /// // its a AABB-Box as:
    /// // frontX: 3, rightY: 3, topZ: 3,
    /// // backX: -4, leftY: -4, bottom: -4,
    /// let mut a = Self::from_scope(3);
    ///
    /// // ...1.pos() = P3(3,-4,3)
    /// // still in this scope,
    /// // wouldn't extend the scope
    /// a.insert(...1);
    ///
    /// // ...2.pos() = P3(0,0,4)
    /// // out of this scope,
    /// // would extend the scope as 8 (4*2)
    /// // its a AABB-Box as:
    /// // frontX: 7, rightY: 7, topZ: 7,
    /// // backX: -8, leftY: -8, bottom: -8,
    /// a.insert(...2);
    /// ```
    ///
    /// # Example
    /// ```
    /// # use super::instance::Instance;
    /// # use cgmath::Point3;
    /// let mut a = Self::from_scope(3);
    ///
    /// a.insert(Instance::default());
    ///
    /// assert_eq!(
    ///     a.get(Point3 { x: 0, y: 0, z: 0 }),
    ///     Some(Instance::default())
    /// );
    /// ```
    pub fn from_scope(scope: usize) -> Self {
        for i in (0..).map(|i| (2.0_f32.powf(scope as f32)) as usize) {
            if i >= scope {
                return Self {
                    __value__: Node::default(),
                    __len__: 0,
                    __scope__: i,
                };
            }
        }

        Self::default()
    }

    /// get the Instance by its position
    ///
    /// # Example
    /// ```
    /// # use super::instance::Instance;
    /// # use cgmath::Point3;
    /// let mut a = Self::from_scope(3);
    ///
    /// a.insert(Instance::default());
    ///
    /// assert_eq!(
    ///     a.get(Point3 { x: 0, y: 0, z: 0 }),
    ///     Some(Instance::default())
    /// );
    /// ```
    pub fn get(&self, pos: Point3<i32>) -> Option<Instance> {
        self.__value__.get(pos)
    }

    /// return a iterator for all the instances.
    ///
    /// Index as x++ -overflow-> y++ -overflow-> z++
    ///
    /// # Examples
    /// ```
    /// # use super::instance::Instance;
    /// # use cgmath::Point3;
    /// let mut a = Self::from_scope(3);
    ///
    /// a.insert(Instance::default());
    ///
    /// assert_eq!(a.iter().next(), Some(Instance::default()));
    /// ```
    pub fn iter(&self) -> iter::RefIter {
        iter::RefIter::new(self)
    }

    /// - loop:
    ///     - if in scope {driect insert}
    ///     - else {extend}
    ///
    /// # Example
    /// ```
    /// # use super::instance::Instance;
    /// # use cgmath::Point3;
    /// let mut a = Self::from_scope(3);
    ///
    /// a.insert(Instance::default());
    ///
    /// assert_eq!(
    ///     a.get(Point3 { x: 0, y: 0, z: 0 }),
    ///     Some(Instance::default())
    /// );
    /// ```
    pub fn insert(&mut self, v: Instance) {
        self.__len__ += 1;

        loop {
            if self.is_in_scope(&v.pos()) {
                self.__value__.insert(v, CENTRAL, self.__scope__);
                break;
            } else {
                self.update_scope(self.__scope__ * 2);
            }
        }
    }

    fn is_in_scope(&self, p: &Point3<i32>) -> bool {
        p.x > -(self.__scope__ as i32)
            && p.x <= self.__scope__ as i32
            && p.y > -(self.__scope__ as i32)
            && p.y <= self.__scope__ as i32
            && p.z > -(self.__scope__ as i32)
            && p.z <= self.__scope__ as i32
    }

    fn update_scope(&mut self, new_scope: usize) {
        let mut new = Self::from_scope(new_scope);

        for i in self.iter() {
            new.insert(i);
        }

        *self = new;
    }
}

impl Default for OcTree {
    fn default() -> Self {
        Self {
            __value__: Node::default(),
            __len__: 0,
            __scope__: 0,
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

    fn get(&self, pos: Point3<i32>) -> Option<Instance> {
        match self {
            Node::Trunk(t) => t.get(pos),
            Node::Leaf(Some(v)) => Some(v.clone()),
            Node::Leaf(None) => None,
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

    fn get(&self, pos: Point3<i32>) -> Option<Instance> {
        let toward = super::toward(pos, self.central);
        self.branches[toward].get(pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oc_tree_base_work() {
        let mut a = OcTree::from_scope(3);

        a.insert(Instance::default());

        assert_eq!(
            a.get(Point3 { x: 0, y: 0, z: 0 }),
            Some(Instance::default())
        );
    }

    #[test]
    fn oc_tree_work() {
        let instances: Vec<_> = (0..100)
            .map(|i| {
                Instance::new(
                    Point3 {
                        x: -i,
                        y: i - 1,
                        z: i * 2,
                    },
                    i.to_string(),
                )
            })
            .collect();

        let mut octree = OcTree::from_scope(4);

        for v in instances.clone() {
            octree.insert(v);
        }

        for v in instances {
            let gv = octree.get(v.pos());

            assert_eq!(gv, Some(v));
        }
    }
}
