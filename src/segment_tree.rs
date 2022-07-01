use std::ops::Add;
pub struct SegmentTree<T>
where
    T: Add<Output = T> + Default + Copy,
{
    pub size: usize,
    pub elems: Vec<T>,
    pub tree: Vec<T>,
}

impl<T> SegmentTree<T>
where
    T: Add<Output = T> + Default + Copy,
{
    pub fn new(elems: Vec<T>) -> Self {
        let size = elems.len();
        let tree = vec![T::default(); 4 * size];
        let mut seg_tree = SegmentTree { size, elems, tree };
        seg_tree.build(1, 0, size - 1);
        seg_tree
    }

    pub fn query_range(&self, left: usize, right: usize) -> Result<T, ()> {
        if (left > right) || (left >= self.size) {
            return Err(());
        }
        Ok(self.query_util(left, right, 1, 0, self.size - 1))
    }

    pub fn update_val(&mut self, index: usize, new_val: T) -> Result<(), ()> {
        if index >= self.size {
            return Err(());
        }
        self.update_util(index, new_val, 1, 0, self.size - 1);
        Ok(())
    }

    fn build(&mut self, k: usize, left: usize, right: usize) {
        if left == right {
            self.tree[k] = self.elems[left];
        } else {
            let mid = (left + right) / 2;
            self.build(2 * k, left, mid);
            self.build(2 * k + 1, mid + 1, right);
            self.tree[k] = self.tree[2 * k] + self.tree[2 * k + 1];
        }
    }

    fn query_util(
        &self,
        query_left: usize,
        query_right: usize,
        k: usize,
        left: usize,
        right: usize,
    ) -> T {
        if query_left > right || query_right < left {
            return T::default();
        }
        if query_left <= left && right <= query_right {
            return self.tree[k];
        }

        let mid = (left + right) / 2;

        self.query_util(query_left, query_right, 2 * k, left, mid)
            + self.query_util(query_left, query_right, 2 * k + 1, mid + 1, right)
    }

    fn update_util(&mut self, index: usize, new_val: T, k: usize, left: usize, right: usize) {
        if left == right {
            self.tree[k] = new_val;
            self.elems[index] = new_val;
        } else {
            let mid = (left + right) / 2;
            if index <= mid {
                self.update_util(index, new_val, 2 * k, left, mid);
            } else {
                self.update_util(index, new_val, 2 * k + 1, mid + 1, right);
            }
            self.tree[k] = self.tree[2 * k] + self.tree[2 * k + 1];
        }
    }
}
