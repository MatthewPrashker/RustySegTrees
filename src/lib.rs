pub mod segment_tree_naive;
pub mod segment_tree;

#[cfg(test)] 
mod tests;

use std::ops::Add;
pub trait RangeQuerier {
    type Item: Add<Output = Self::Item> + Default + Copy;
    fn update_val(&mut self, index: usize, val: Self::Item) -> Result<(), ()>;
    fn query_range(&self, left: usize, right: usize) -> Result<Self::Item, ()>;
}
