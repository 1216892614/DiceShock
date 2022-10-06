use cgmath::Point3;

pub mod instance;
pub mod oc_tree;

/// left handed position
/// - self at front of the other
///     - +1
/// - self at right of the other
///     - +2
/// - self at above of the other
///     - +4
const fn toward(s: Point3<i32>, o: Point3<i32>) -> usize {
    let mut ans = 0;

    if s.x >= o.x {
        ans += 1
    };
    if s.y >= o.y {
        ans += 2
    };
    if s.z >= o.z {
        ans += 4
    };

    ans
}

pub(crate) struct Scenes {}
