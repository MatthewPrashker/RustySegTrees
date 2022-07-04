pub mod segment_tree;
mod tests;

use std::ops::Add;
pub trait rangeQuerier {
    type Item: Add<Output = Self::Item> + Default + Copy;
    fn update_val(&mut self, index: usize, val: Self::Item) -> Result<(), ()>;
    fn query_range(&self, left: usize, right: usize) -> Result<Self::Item, ()>;
}
