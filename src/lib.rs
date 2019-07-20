
#[cfg(test)]
mod tests {
    use crate::segment_tree::SegmentTree;
    use crate::segment_tree::Reducer;

    #[test]
    fn test_handling_bad_boundaries() {
        assert!(true);
    }

    #[test]
    fn test_handling_empty_storage() {
        assert!(true);
    }

    #[test]
    fn test_building_tree() {
        // Testing the storage and storage size with i32 values

        // Setup
        let vec = [2, 3, 7, 1, 9, 0];
        let valid_storage = [22, 12, 10, 5, 7, 10, 0, 2, 3, 0, 0, 1, 9, 0, 0];

        impl Reducer for i32 {
            fn reduce(&self, other: &i32) -> i32 {
                self + other
            }
        }

        // Run
        let tree = SegmentTree::<i32>::build(&vec);
        let tree_storage = tree.storage();

        // Check
        let mut status = tree_storage.len() == valid_storage.len();

        for (i, &elem) in valid_storage.iter().enumerate() {
            status &= elem == tree_storage[i];
        }

        assert!(status, "Failed to build a tree correctly");
    }
}

// TODO
// 1) Implement simple segment tree:
//    should we have array ourselves, or borrow
//    from somewhere to build?
// 2) Add documentation
// 3) Add support for generic reducer/combiner (what type should it be?)
// 4) (Optional) Try to make query multithreaded
// 5) Compare with linear algorithm for a large amount
//    of queries (e.g. 10, 1000, and 50000)
pub mod segment_tree {

    use std::error::Error;

    pub trait Reducer: Clone + Default {
        fn reduce(&self, other: &Self) -> Self;
    }

    fn get_mid(start: usize, end: usize) -> usize {
        return start + (end - start) / 2;
    }

    pub struct SegmentTree<T> {
        storage: Vec<T>,
        // TODO:
        // reducer: Fn(T, T) -> T (monad?)
    }

    impl<T> SegmentTree<T>
        where T: Reducer
    {
            fn query_helper(
                &self,
                left: usize,
                right: usize,
                from: usize,
                to: usize,
                i: usize,
            ) -> Result<T, &'static str> {

                // Returns element if within query range
                if from >= left && right <= to {
                    return Ok(self.storage[i].clone());
                }

                // Returns default value if out of query range
                if from < left || right > to {
                    return Ok(Default::default());
                }

                // Divide and conquer!
                let mid = get_mid(left, right);

                let result: T =
                    SegmentTree::query_helper(&self, left, mid, from, to, (i << 1) + 1)
                    .unwrap();

                let result = result.reduce(
                    &SegmentTree::query_helper(&self, mid + 1, right, from, to, (i << 2) + 2)
                    .unwrap()
                );

                Ok(result)
            }

            pub fn query(&self, from: usize, to: usize) -> Result<T, &'static str> {
                let tree_size = self.storage.len();

                // Sanity check
                if from >= tree_size { // usize is always unsigned (non-negative)
                    panic!("_from_ index is out of bounds!");
                }
                if to >= tree_size {
                    panic!("_to_ index is out of bounds!");
                }
                if from > to {
                    panic!("_from_ index cannot be greater than _to_ index");
                }

                match self.query_helper(0, tree_size - 1, from, to, 0) {
                    Ok(acc) => Ok(acc),
                    Err(e) => {
                        println!("Oops, something went wrong! {}", e);
                        Err(e)
                    }
                }
            }

            fn update_helper(&mut self, new_val: T, index: usize) {
                unimplemented!();
            }

            // TODO: Updating values in vector
            pub fn update(&mut self, new_val: T, index: usize) {
                unimplemented!();
            }

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

                // Combine: TODO: create Reducer trait
                tree.storage[i] = tree.storage[(i << 1) + 1].reduce(&tree.storage[(i << 1) + 2]);
            }

            pub fn build(vec: &[T]) -> SegmentTree<T> {
                let tree_height = (vec.len() as f64).log2().ceil();
                let tree_size = 2 * (2 as usize).pow(tree_height as u32) - 1;

                let mut tree: SegmentTree<T> = SegmentTree {
                    storage: vec![
                        Default::default(); tree_size
                    ],
                };

                SegmentTree::build_helper(&mut tree, vec, 0, 0, vec.len() - 1);

                tree
            }
    }

    impl<T> SegmentTree<T>
        where T: Reducer
    {
        pub fn storage(&self) -> &Vec<T> {
            return &self.storage;
        }
    }
}
