//! Traits for encoding and decoding list types.
use crate::decoder::{DecodeError, Decoder};
use crate::encoder::{EncodeError, Encoder};
use crate::mem::MemoryManager;
use crate::value::SchemaValue;
use allocator_api2::alloc::Allocator;
use allocator_api2::vec::Vec;

/// A visitor for encoding list types.
pub trait ListEncodeVisitor {
    /// Get the size of the list.
    fn size(&self) -> usize;
    /// Encode the list.
    fn encode(&self, idx: usize, encoder: &mut dyn Encoder) -> Result<(), EncodeError>;
}

/// A visitor for decoding list types.
pub trait ListDecodeVisitor<'a> {
    /// Initialize the visitor with the length of the list.
    /// This method may or may not be called depending on whether the underlying
    /// encoding specifies the length of the list.
    fn reserve(&mut self, len: usize, scope: &'a MemoryManager) -> Result<(), DecodeError>;
    /// Decode the next element in the list.
    fn next(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError>;
}

/// A builder for decoding Vec's with a specified allocator.
pub struct AllocatorVecBuilder<'a, T: SchemaValue<'a>> {
    pub(crate) xs: Option<Vec<T, &'a dyn Allocator>>,
}

impl<'a, T: SchemaValue<'a>> Default for AllocatorVecBuilder<'a, T> {
    fn default() -> Self {
        Self { xs: None }
    }
}

impl<'a, T: SchemaValue<'a>> AllocatorVecBuilder<'a, T> {
    fn get_xs(&mut self, mem: &'a MemoryManager) -> &mut Vec<T, &'a dyn Allocator> {
        if self.xs.is_none() {
            self.xs = Some(Vec::new_in(mem));
        }
        self.xs.as_mut().unwrap()
    }
}

impl<'a, T: SchemaValue<'a>> ListDecodeVisitor<'a> for AllocatorVecBuilder<'a, T> {
    fn reserve(&mut self, len: usize, scope: &'a MemoryManager) -> Result<(), DecodeError> {
        self.get_xs(scope).reserve(len);
        Ok(())
    }

    fn next(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        let mut x = T::default();
        x.decode(decoder)?;
        self.get_xs(decoder.mem_manager()).push(x);
        Ok(())
    }
}

#[cfg(feature = "std")]
impl<'a, T: SchemaValue<'a>> ListDecodeVisitor<'a> for alloc::vec::Vec<T> {
    fn reserve(&mut self, len: usize, _scope: &'a MemoryManager) -> Result<(), DecodeError> {
        self.reserve(len);
        Ok(())
    }

    fn next(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        let mut x = T::default();
        x.decode(decoder)?;
        self.push(x);
        Ok(())
    }
}

impl<'a, T: SchemaValue<'a>> ListEncodeVisitor for &'a [T] {
    fn size(&self) -> usize {
        self.len()
    }

    fn encode(&self, idx: usize, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        self[idx].encode(encoder)
    }
}

#[cfg(feature = "std")]
impl<'a, T: SchemaValue<'a>> ListEncodeVisitor for alloc::vec::Vec<T> {
    fn size(&self) -> usize {
        self.len()
    }

    fn encode(&self, idx: usize, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        self[idx].encode(encoder)
    }
}
