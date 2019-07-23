use std::convert::From;

use segtree::SegmentTree;
use segtree::Reducer;

#[derive(Debug, Default, Clone)]
struct Sum(f32);

impl From<f32> for Sum {
    fn from(item: f32) -> Self {
        Sum(item)
    }
}

impl Into<f32> for Sum {
    fn into(self) -> f32 {
        self.0
    }
}

fn main() {
    // Testing querying functionality 

    // Setup
    /*let vec = [2.5, 3.25, 7.0, 1.75, 9.35, 0.90];
    let valid_storage = [
        24.75, 12.75, 12.0, 5.75, 7.0, 11.1, 0.9,
        2.5, 3.25, 0.0, 0.0, 1.75, 9.35, 0.0, 0.0
    ];*/


    impl Reducer for Sum {
        fn reduce(&self, other: Sum) -> Sum {
            Sum(self.0 + other.0)
        }
    }


    let vec = [Sum(2.5), Sum(3.25), Sum(7.0), Sum(1.75), Sum(9.35), Sum(0.90)];
    let tree: SegmentTree<Sum> = SegmentTree::build(&vec);
    
    // Run
    let result = tree.query(0, 5); // inclusive, indexing = 0, 1, ..., n-1

    match result {
        Ok(result) => println!("result = {:?}", result),
        Err(_) => println!("failed ..."),
    }

}
