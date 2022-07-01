pub mod segment_tree;

#[cfg(test)]
mod tests {
    use super::segment_tree::SegmentTree;

    #[test]
    pub fn basic() {
        let mut s = SegmentTree::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(s.query_range(1, 3).unwrap(), 2 + 3 + 4);
        for i in 0..s.size {
            assert_eq!(s.elems[i], s.query_range(i, i).unwrap());
        }

        s.update_val(2, 10);
        assert_eq!(s.query_range(1, 3).unwrap(), 16);
    }
}
