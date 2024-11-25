pub mod sled;
mod tracer;
mod jmt;

pub trait State {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, String>;
}