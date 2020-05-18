// Copyright (c) 2020 Wei Tang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation of Chtholly Tree, a data structure originated from
//! CF896C.

#![no_std]
#![warn(missing_docs)]

extern crate alloc;

use core::cmp::{min, max};
use alloc::vec::Vec;

/// Representation of Chtholly Node, used to build Chtholly Tree.
pub struct ChthollyNode {
    left: usize,
    right: usize,
    value: usize,
}

impl ChthollyNode {
    /// Whether the range of the current Chtholly Node contains `x`.
    pub fn contains(&self, x: usize) -> bool {
        self.left <= x && x <= self.right
    }

    /// Get the total length of the current Chtholly Node.
    pub fn len(&self) -> usize {
        self.right - self.left + 1
    }
}

/// Representation of Chtholly Tree. The nodes are sorted by their range.
#[derive(Default)]
pub struct ChthollyTree(Vec<ChthollyNode>);

impl ChthollyTree {
    /// Split the range between `[left, middle - 1]` and `[middle, right]`.
    /// Returns the node representing `[middle, right]`.
    pub fn split(&mut self, middle: usize) -> Option<&ChthollyNode> {
        match self.split_inner(middle) {
            Some(index) => Some(&self.0[index]),
            None => None,
        }
    }

    /// Set all values between `[left, right]` to be `value`, and merge them.
    /// Split nodes when necessary. Create a new node if it does not yet exist.
    pub fn merge(&mut self, left: usize, right: usize, value: usize) {
        self.split_inner(right);
        let index = self.split_inner(left.saturating_sub(1));

        match index {
            Some(index) => {
                self.0[index].value = value;
                self.0[index].right = right;

                while self.0[index + 1].left <= right {
                    self.0.remove(index + 1);
                }
            },
            None => {
                self.0.push(ChthollyNode { left, right, value });
                self.sort_inner();
            },
        }
    }

    /// Add `x` to all values between `[left, right]`.
    pub fn add(&mut self, left: usize, right: usize, value: usize) {
        self.split_inner(right);
        let start = match self.split_inner(left.saturating_sub(1)) {
            Some(start) => start,
            None => return,
        };

        for index in start..self.0.len() {
            if self.0[index].left > right {
                break
            }

            self.0[index].value += value;
        }
    }

    /// Find `n`-th (0-indexed) smallest `value` after `left`.
    pub fn nth(&self, left: usize, mut x: usize) -> Option<usize> {
        let mut index = self.0.binary_search_by(|node| {
            node.left.cmp(&left)
        }).ok()?;

        loop {
            if x == 0 {
                return Some(self.0[index].value)
            }

            let len = self.0[index].right - max(left, self.0[index].left) + 1;

            if x < len {
                return Some(self.0[index].value)
            }

            if index + 1 >= self.0.len() {
                return None
            }

            x -= len;
            index += 1;
        }
    }

    /// Compute the sum of power between `[left, right]`.
    pub fn pow_sum(&self, left: usize, right: usize, power: u32, modulo: usize) -> usize {
        let mut index = match self.0.binary_search_by(|node| {
            node.left.cmp(&left)
        }) {
            Ok(index) => index,
            Err(_) => return 0,
        };

        let mut sum = 0;
        loop {
            if self.0[index].left > right {
                break
            }

            let left = max(left, self.0[index].left);
            let right = min(right, self.0[index].right);
            let len = right - left;

            sum = (sum + len * (self.0[index].value.pow(power) % modulo)) % modulo;
            index += 1;
        }
        sum
    }

    /// Sort the tree. All public operations should already ensure
    /// that the tree is sorted, and this function is only used when
    /// necessary.
    fn sort_inner(&mut self) {
        self.0.sort_unstable_by_key(|node| node.left);
    }

    /// Split the range between `[left, middle - 1]` and `[middle, right]`.
    /// Returns the index representing `[middle, right]`.
    fn split_inner(&mut self, middle: usize) -> Option<usize> {
        let index = self.0.binary_search_by(|node| {
            node.left.cmp(&middle)
        }).ok()?;

        if self.0[index].left == middle {
            // No need to split if left is the same as middle.
            return Some(index)
        }

        let new = ChthollyNode {
            left: middle,
            right: self.0[index].right,
            value: self.0[index].value,
        };

        self.0.insert(index + 1, new);
        self.0[index].right = middle;

        Some(index + 1)
    }
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    const V_MAX: usize = 1_000_000_000;
    const SEED_MAX: usize = 1_000_000_007;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
