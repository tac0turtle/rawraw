use jmt::SimpleHasher;
use jmt::storage::TreeReader;

pub struct Jmt<R: TreeReader> {
    tree_reader: R,
}

impl<R: TreeReader> Jmt<R> {
    pub fn new() -> Self {
        panic!("impl")
    }
}