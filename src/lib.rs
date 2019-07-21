//! Generic segment tree implementation. Purely for learning
//! and experimental purposes.
//!
//! `segtree` provides:
//! - Ability to define your own `combine`/`reduce`.
//! - Fundamental `query`, `update`, and `build` operations.
//! - Performance tests (benchmarking) (WIP)
//!
//! It may *potentially*:
//! - Be published as crate (after bringing more functionality in).
//! - Leverage Rust's multithreading features for a performance speed-up.
//! - Get strict in defining custom `combine`/`reduce` operations (example of a bifunctor).
//!

/**
 * Author: Batyr N. <oneturkmen@gmail.com>
 * Date: July, 2019
 **/

use std::fmt::Debug;

/// Trait for custom user-provided `reduce` operation.
pub trait Reducer: Clone + Default {
    fn reduce(&self, other: Self) -> Self;
}

/// Gets the `mid` index given `start` and `end`
/// for binary splitting.
fn get_mid(start: usize, end: usize) -> usize {
    return start + ((end - start) / 2);
}

/// Holds the tree as a linear vector as well as
/// number of elements in the (original) source array.
pub struct SegmentTree<T> {
    storage: Vec<T>,
    num_of_elements: usize, // number of elements
}

impl<T> SegmentTree<T>
where
T: Reducer + Debug,
{

    /// Helper function for `query` operation
    /// that works in a recursive manner (if necessary).
    ///
    /// * `start` - current segment's starting index.
    /// * `end`   - current segment's end index.
    /// * `from`  - from index of the requested range (inclusive).
    /// * `to`    - to index of the requested range (inclusive).
    /// * `i`     - current element's (tree) storage index.
    fn query_helper(
        &self,
        start: usize,
        end: usize,
        from: usize,
        to: usize,
        i: usize
    ) -> T {
        // Returns element if within query range
        if start >= from && end <= to {
            return self.storage[i].clone();
        }

        // Returns default value if out of query range
        if end < from || start > to {
            return Default::default();
        }

        // Divide and combine
        let mid = get_mid(start, end);

        self.query_helper(start, mid, from, to, (i << 1) + 1)
            .reduce(self.query_helper(mid + 1, end, from, to, (i << 1) + 2))
    }

    /// Function that provides `query` operation: reduces the elements
    /// in the given range and returns the combined (reduced) result.
    ///
    /// * `from`  - from index of the requested range (inclusive).
    /// * `to`    - to index of the requested range (inclusive).
    pub fn query(&self, from: usize, to: usize) -> Result<T, &'static str> {
        let num_of_elements = self.num_of_elements;

        // Sanity check
        if from >= num_of_elements {
            // usize is always unsigned (non-negative)
            panic!("_from_ index is out of bounds!");
        }
        if to >= num_of_elements {
            panic!("_to_ index is out of bounds!");
        }
        if from > to {
            panic!("_from_ index cannot be greater than _to_ index");
        }

        Ok(self.query_helper(0, num_of_elements - 1, from, to, 0))
    }

    fn update_helper(
        &mut self,
        new_val: &T,
        i: usize,
        start: usize,
        end: usize,
        t_i: usize,
    ) -> Result<(), &'static str> {
        if start > end || start > i || i > end {
            return Ok(());
        }

        if start == end {
            self.storage[t_i] = new_val.clone();
            return Ok(());
        }

        let mid = get_mid(start, end);

        match self.update_helper(&new_val, i, start, mid, (t_i << 1) + 1) {
            Ok(_) => {} // does nothing
            Err(e) => return Err(e),
        }

        match self.update_helper(&new_val, i, mid + 1, end, (t_i << 1) + 2) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        self.storage[t_i] =
            self.storage[(t_i << 1) + 1].reduce(self.storage[(t_i << 1) + 2].clone());

        Ok(())
    }

    // TODO: Comment
    pub fn update(&mut self, new_val: &T, i: usize) -> Result<(), &'static str> {
        if i >= self.storage.len() {
            panic!("Provided index is out of bounds!");
        }

        match self.update_helper(new_val, i, 0, self.storage.len() - 1, 0) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    // TODO: Comment
    fn build_helper(
        tree: &mut SegmentTree<T>,
        source: &[T],
        i: usize,
        start: usize,
        end: usize,
        ) {
        if start > end {
            panic!("start index should not be larger than end index");
        }

        // If one-element list
        if start == end {
            tree.storage[i] = source[start].clone();
            return;
        }

        // Get the mid index for splitting
        let mid: usize = get_mid(start, end);

        // Divide into left and right sub-procedures
        SegmentTree::build_helper(tree, source, (i << 1) + 1, start, mid);
        SegmentTree::build_helper(tree, source, (i << 1) + 2, mid + 1, end);

        // Combine
        tree.storage[i] = tree.storage[(i << 1) + 1]
                              .reduce(tree.storage[(i << 1) + 2].clone());
    }

    // TODO: Comment
    pub fn build(vec: &[T]) -> SegmentTree<T> {
        let tree_height = (vec.len() as f64).log2().ceil();
        let tree_size = 2 * (2 as usize).pow(tree_height as u32) - 1;

        let mut tree: SegmentTree<T> = SegmentTree {
            storage: vec![Default::default(); tree_size],
            num_of_elements: vec.len(),
        };

        SegmentTree::build_helper(&mut tree, vec, 0, 0, vec.len() - 1);

        tree
    }

    // TODO: Comment
    pub fn storage(&self) -> &Vec<T> {
        return &self.storage;
    }
}
