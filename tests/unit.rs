
use segtree::segment_tree::Reducer;
use segtree::segment_tree::SegmentTree;

#[test]
fn test_update() {}

#[test]
fn test_querying() {
    // Testing querying functionality

    // Setup
    let vec = [Sum(2.5), Sum(3.25), Sum(7.0), Sum(1.75), Sum(9.35), Sum(0.90)];

    #[derive(Default, Clone, Debug)]
    struct Sum(f32);

    impl Reducer for Sum {
        fn reduce(&self, other: Sum) -> Sum {
            Sum(self.0 + other.0)
        }
    }

    let tree = SegmentTree::<Sum>::build(&vec);

    // Run
    let result = tree.query(0, 5).unwrap(); // inclusive, indexing = 0, 1, ..., n-1

    // Check
    assert_eq!(result.0, 24.75);
}

#[test]
fn test_building_tree() {
    // Testing the storage and storage size with i32 values

    // Setup
    let vec = [Sum(2), Sum(3), Sum(7), Sum(1), Sum(9), Sum(0)];
    let valid_storage = [22, 12, 10, 5, 7, 10, 0, 2, 3, 0, 0, 1, 9, 0, 0];


    #[derive(Default, Clone, Debug)]
    struct Sum(i32);

    impl Reducer for Sum {
        fn reduce(&self, other: Sum) -> Sum {
            Sum(self.0 + other.0)
        }
    }

    // Run
    let tree = SegmentTree::<Sum>::build(&vec);
    let tree_storage = tree.storage();

    // Check
    let mut status = tree_storage.len() == valid_storage.len();

    for (i, &elem) in valid_storage.iter().enumerate() {
        status &= elem == tree_storage[i].0;
    }

    assert!(status, "Failed to build a tree correctly");
}
