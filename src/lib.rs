pub mod segment_tree;

#[cfg(test)]
mod tests {
    use super::segment_tree::SegmentTree;
    use std::ops::Add;
    #[test]
    pub fn basic() {
        let mut s = SegmentTree::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(s.query_range(1, 3).unwrap(), 2 + 3 + 4);
        for i in 0..s.size {
            assert_eq!(s.elems[i], s.query_range(i, i).unwrap());
        }

        s.update_val(2, 10);
        assert_eq!(s.query_range(1, 3).unwrap(), 2 + 10 + 4);
    }

    #[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
    struct Point {
        x: u8,
        y: u8,
    }

    impl Add for Point {
        type Output = Point;
        fn add(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    #[test]
    pub fn basic_with_custom_struct() {
        let mut s = SegmentTree::new(vec![
            Point { x: 1, y: 2 },
            Point { x: 2, y: 3 },
            Point { x: 3, y: 4 },
            Point { x: 4, y: 100 },
        ]);
        assert_eq!(s.query_range(1, 2).unwrap(), Point { x: 5, y: 7 });
    }
}
