//! Utility functions for working with allocators.
use crate::code::{ErrorCode, SystemCode};
use allocator_api2::alloc::Allocator;
use core::alloc::Layout;
use core::str::from_utf8_unchecked;

/// Copies a string into the given allocator.
pub unsafe fn copy_str<'a>(allocator: &'a dyn Allocator, s: &str) -> Result<&'a str, ErrorCode> {
    let bytes_copy = copy_bytes(allocator, s.as_bytes())?;
    Ok(from_utf8_unchecked(bytes_copy))
}

/// Copies bytes into the given allocator.
pub unsafe fn copy_bytes<'a>(allocator: &'a dyn Allocator, s: &[u8]) -> Result<&'a [u8], ErrorCode> {
    let copy = allocator.allocate(Layout::from_size_align_unchecked(s.len(), 1))
        .map_err(|_| ErrorCode::SystemCode(SystemCode::FatalExecutionError))?;
    let ptr = copy.as_ptr();
    (*ptr).copy_from_slice(s);
    Ok(&*ptr)
}
