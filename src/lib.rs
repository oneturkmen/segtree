
#[cfg(test)]
mod tests {
    use crate::segment_tree::SegmentTree;
    use crate::segment_tree::Reducer;

    #[test]
    fn test_update() {

    }

    #[test]
    fn test_querying() {
        // Testing querying functionality 

        // Setup
        let vec = [2.5, 3.25, 7.0, 1.75, 9.35, 0.90];

        impl Reducer for f32 {
            fn reduce(&self, other: f32) -> f32 {
                self + other
            }
        }

        let tree = SegmentTree::<f32>::build(&vec);

        // Run
        let result = tree.query(0, 5).unwrap(); // inclusive, indexing = 0, 1, ..., n-1

        // Check
        assert_eq!(result, 24.75);
    }

    #[test]
    fn test_building_tree() {
        // Testing the storage and storage size with i32 values

        // Setup
        let vec = [2, 3, 7, 1, 9, 0];
        let valid_storage = [22, 12, 10, 5, 7, 10, 0, 2, 3, 0, 0, 1, 9, 0, 0];

        impl Reducer for i32 {
            fn reduce(&self, other: i32) -> i32 {
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

    use std::fmt::Debug;

    pub trait Reducer: Clone + Default {
        fn reduce(&self, other: Self) -> Self;
    }

    fn get_mid(start: usize, end: usize) -> usize {
        return start + ((end - start) / 2);
    }

    pub struct SegmentTree<T> {
        storage: Vec<T>,
        num_of_elements: usize, // number of elements
        // TODO:
        // reducer: Fn(T, T) -> T (monad?)
    }

    impl<T> SegmentTree<T>
        where T: Reducer + Debug
    {
            fn query_helper(
                &self,
                start: usize,
                end: usize,
                from: usize,
                to: usize,
                i: usize,
            ) -> T {

                println!("Index = {} & {}", start, end);
                // Returns element if within query range
                if start >= from && end <= to {
                    println!("Returning {:?}", &self.storage[i]);
                    return self.storage[i].clone();
                }

                // Returns default value if out of query range
                if end < from || start > to {
                    println!("Returning default");
                    return Default::default();
                }

                // Divide and conquer!
                let mid = get_mid(start, end);

                println!("Going deeper ...");

                self.query_helper(start, mid, from, to, (i << 1) + 1)
                    .reduce(self.query_helper(mid + 1, end, from, to, (i << 1) + 2))


                /*let left_result: T =
                    self.query_helper(start, mid, from, to, (i << 1) + 1);


                /*println!("hooooooooooooooooowdddyyyyyyyyyyyyyyyyyyyyyyyyy");
                println!("result left = {:?}", &left_result);*/

                let right_result = 
                    self.query_helper(mid + 1, end, from, to, (i << 1) + 2);

                println!("result right = {:?}", &right_result);*/

                //left_result.reduce(&right_result)
            }

            pub fn query(&self, from: usize, to: usize) -> Result<T, &'static str> {
                let num_of_elements = self.num_of_elements;

                // Sanity check
                if from >= num_of_elements { // usize is always unsigned (non-negative)
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
                    Ok(_) => { }, // does nothing
                    Err(e) => return Err(e),
                }

                match self.update_helper(&new_val, i, mid + 1, end, (t_i << 1) + 2) {
                    Ok(_) => { },
                    Err(e) => return Err(e),
                }
                
                self.storage[t_i] = self.storage[(t_i << 1) + 1].reduce(self.storage[(t_i << 1) + 2].clone());

                Ok(())
            }

            // TODO: Updating values in vector
            pub fn update(&mut self, new_val: &T, i: usize) -> Result<(), &'static str> {
                if i >= self.storage.len() {
                    panic!("Provided index is out of bounds!");
                }

                match self.update_helper(new_val, i, 0, self.storage.len() - 1, 0) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
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
                tree.storage[i] = tree.storage[(i << 1) + 1].reduce(tree.storage[(i << 1) + 2].clone());
            }

            pub fn build(vec: &[T]) -> SegmentTree<T> {
                let tree_height = (vec.len() as f64).log2().ceil();
                let tree_size = 2 * (2 as usize).pow(tree_height as u32) - 1;

                let mut tree: SegmentTree<T> = SegmentTree {
                    storage: vec![
                        Default::default(); tree_size
                    ],
                    num_of_elements: vec.len(),
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
