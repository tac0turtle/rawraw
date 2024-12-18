//! Utility functions for working with allocators.
use crate::code::{ErrorCode, SystemCode};
use allocator_api2::alloc::Allocator;
use core::alloc::Layout;
use core::str::from_utf8_unchecked;

/// Copies bytes into the given allocator.
///
/// # Safety
/// This function is considered unsafe because it returns memory from the allocator
/// that will not be deallocated by the allocator because it must live as long as the
/// allocator.
/// If you use the global allocator, this is unsafe because the global allocator
/// lives as long as the process.
/// This method is only safe for scoped allocators such as bump allocators.
pub unsafe fn copy_bytes<'a>(
    allocator: &'a dyn Allocator,
    s: &[u8],
) -> Result<&'a [u8], ErrorCode> {
    let copy = allocator
        .allocate(Layout::from_size_align_unchecked(s.len(), 1))
        .map_err(|_| ErrorCode::SystemCode(SystemCode::FatalExecutionError))?;
    let ptr = copy.as_ptr();
    (*ptr).copy_from_slice(s);
    Ok(&*ptr)
}

/// Copies a string into the given allocator.
///
/// # Safety
/// See [`copy_bytes`]
pub unsafe fn copy_str<'a>(allocator: &'a dyn Allocator, s: &str) -> Result<&'a str, ErrorCode> {
    let bytes_copy = copy_bytes(allocator, s.as_bytes())?;
    Ok(from_utf8_unchecked(bytes_copy))
}
