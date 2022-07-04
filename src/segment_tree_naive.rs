use crate::common::*;
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
    type EntryType = T;
    fn query_range(&self, left: usize, right: usize) -> Result<Self::EntryType> {
        
        if left > right || right >= self.size {
            return Err( Error {kind: ErrorKind::QueryRangeNotValid { left, right, length: self.size }})
        }
        
        if left == 0 {
            Ok(self.partial_sums[right])
        } else {
            Ok(self.partial_sums[right] - self.partial_sums[left - 1])
        }
    }

    fn update_val(&mut self, index: usize, val: Self::EntryType) -> Result<()> {
        if index >= self.size {
            return Err( Error {kind: ErrorKind::UpdateIndexNotValid { index, length: self.size }})
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
    pub fn new(elems: Vec<T>) -> Result<Self> {
        let size = elems.len();
        if size == 0 {
            return Err(Error {kind: ErrorKind::ZeroLengthConstruction {  }});
        }
        let mut partial_sums = vec![elems[0]];
        for i in 1..size {
            partial_sums.push(partial_sums[i - 1] + elems[i]);
        }
        Ok(Self {
            size,
            elems,
            partial_sums,
        })
    }
}