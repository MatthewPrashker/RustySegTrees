use crate::RangeQuerier;
use std::ops::{Add, Sub};

pub struct NaiveSegmentTree<T>
where
    T: Add<Output = T> + Sub<Output = T> + Default + Copy,
{
    pub size: usize,
    pub elems: Vec<T>,
    pub partial_sums: Vec<T>,
}

impl<T> RangeQuerier for NaiveSegmentTree<T>
where 
    T: Add<Output = T> + Sub<Output = T> + Default + Copy
{
    type Item = T;
    fn query_range(&self, left: usize, right: usize) -> Result<Self::Item, ()> {
        
        if left > right || right >= self.size {
            return Err(())
        }
        
        if left == 0 {
            Ok(self.partial_sums[right])
        } else {
            Ok(self.partial_sums[right] - self.partial_sums[left - 1])
        }
    }

    fn update_val(&mut self, index: usize, val: Self::Item) -> Result<(), ()> {
        if index >= self.size {
            return Err(())
        }
        //update partial sums
        let diff = val - self.elems[index];
        self.elems[index] = val;
        for i in index..self.size {
            self.partial_sums[i] = self.partial_sums[i] + diff;
        }
        Ok(())
    }
}


impl<T> NaiveSegmentTree<T>
where 
    T: Add<Output = T> + Sub<Output = T> + Default + Copy
{
    pub fn new(elems: Vec<T>) -> Self {
        let size = elems.len();
        let mut partial_sums = vec![elems[0]];
        for i in 1..size {
            partial_sums.push(partial_sums[i - 1] + elems[i]);
        }
        Self {
            size,
            elems,
            partial_sums,
        }
    }
}