//! Traits for encoding and decoding list types.
use crate::decoder::{DecodeError, Decoder};
use crate::encoder::{EncodeError, Encoder};
use crate::mem::MemoryManager;
use crate::value::{ListElementValue, SchemaValue, ValueCodec};
use allocator_api2::alloc::Allocator;
use allocator_api2::vec::Vec;
use crate::types::{BytesT, ListElementType, ListT};

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

impl<'a, 'b, T: SchemaValue<'a>> ListEncodeVisitor for &'b [T] {
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

/// A wrapper around a slice or vector which is backed by an allocator with a lifetime.
/// This allows certain types to be used with dynamically sized vectors without
/// introducing global allocation.
/// You can use this type instead of Vec if you want to avoid global allocation
/// but can't use simply &[T].
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum List<'a, T> {
    /// An empty list.
    #[default]
    Empty,
    /// A borrowed list.
    Borrowed(&'a [T]),
    /// An owned list tied to the lifetime of the allocator.
    Owned(Vec<T, &'a dyn Allocator>),
}

impl<'a> ValueCodec<'a> for List<'a, u8> {
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        *self = List::Borrowed(decoder.decode_borrowed_bytes()?);
        Ok(())
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        match self {
            List::Empty => Ok(()),
            List::Borrowed(bytes) => encoder.encode_bytes(bytes),
            List::Owned(v) => encoder.encode_bytes(v.as_slice()),
        }
    }
}

impl<'a> SchemaValue<'a> for List<'a, u8> {
    type Type = BytesT;
}

impl <'a> ListElementValue<'a> for List<'a, u8> {}

impl<'a, V: ListElementValue<'a>> ValueCodec<'a> for List<'a, V>
where
    V::Type: ListElementType {
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        let mut builder = AllocatorVecBuilder::<'a, V>::default();
        decoder.decode_list(&mut builder)?;
        match builder.xs {
            None => *self = List::Empty,
            Some(xs) => *self = List::Borrowed(decoder.mem_manager().unpack_slice(xs)),
        }
        Ok(())
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        match self {
            List::Empty => Ok(()),
            List::Borrowed(v) => encoder.encode_list(v),
            List::Owned(v) => encoder.encode_list(&v.as_slice()),
        }
    }
}

impl<'a, V: ListElementValue<'a>> SchemaValue<'a> for List<'a, V>
where
    V::Type: ListElementType,
{
    type Type = ListT<V::Type>;
}

impl <'a, V: Clone> List<'a, V> {
    /// Return the length of the list.
    pub fn len(&self) -> usize {
        match self {
            List::Empty => 0,
            List::Borrowed(v) => v.len(),
            List::Owned(v) => v.len(),
        }
    }

    /// Return true if the list is empty.
    pub fn is_empty(&self) -> bool {
        match self {
            List::Empty => true,
            List::Borrowed(v) => v.is_empty(),
            List::Owned(v) => v.is_empty(),
        }
    }
}