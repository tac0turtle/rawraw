//! This module contains the definition of the `MessagePacket` struct.

use core::alloc::Layout;
use core::ptr::NonNull;
use crate::handler::Allocator;
use crate::header::{MessageHeader, MESSAGE_HEADER_SIZE};
use allocator_api2::alloc::AllocError;

/// A packet containing a message and its header.
pub struct MessagePacket<'a> {
    pub(crate) data: NonNull<MessageHeader>,
    pub(crate) len: usize,
    _marker: core::marker::PhantomData<&'a ()>,
}

impl<'a> MessagePacket<'a> {
    /// Creates a new message packet.
    pub unsafe fn new(data: NonNull<MessageHeader>, len: usize) -> Self {
        Self {
            data,
            len,
            _marker: Default::default(),
        }
    }

    /// Allocates a new message packet with the given extra capacity.
    pub unsafe fn allocate(
        allocator: &'a dyn Allocator,
        extra_capacity: usize,
    ) -> Result<Self, AllocError> {
        let size = MESSAGE_HEADER_SIZE + extra_capacity;
        let layout =
            unsafe { Layout::from_size_align_unchecked(size, align_of::<MessageHeader>()) };
        let header_ptr = allocator.allocate_zeroed(layout)?;
        Ok(MessagePacket::new(header_ptr.cast(), size))
    }

    /// Returns the message header.
    pub fn header(&self) -> &'a MessageHeader {
        unsafe { &*self.data.as_ptr() }
    }

    /// Returns a mutable reference to the message header.
    pub fn header_mut(&self) -> &'a mut MessageHeader {
        unsafe { &mut *self.data.as_ptr() }
    }
}
