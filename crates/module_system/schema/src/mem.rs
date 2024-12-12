//! Memory management utilities for codec implementations.

use allocator_api2::alloc::{AllocError, Allocator};
use allocator_api2::boxed::Box;
use allocator_api2::vec::Vec;
use core::alloc::Layout;
use core::cell::Cell;
use core::mem::transmute;
use core::ptr::{drop_in_place, NonNull};

/// A memory manager that tracks allocated memory using a bump allocator and ensures that
/// memory is deallocated and dropped properly when the manager is dropped.
/// The big difference between a raw bump allocator and the memory manager is that it ensures
/// that any values borrowed as slices (ex. &[T]) are properly dropped.
///
/// Currently, the bump allocator uses the global allocator as its base allocator,
/// but this could be customized in the future.
/// For instance, one strategy could be to have a fixed chunk of memory per thread that is used
/// for the bump allocator under the hood.
pub struct MemoryManager {
    bump: crate::bump::BumpAllocator,
    drop_cells: Cell<Option<NonNull<DropCell>>>,
}

struct DropCell {
    dropper: NonNull<dyn DeferDrop>,
    next: Option<NonNull<DropCell>>,
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryManager {
    /// Create a new memory manager.
    pub fn new() -> MemoryManager {
        MemoryManager {
            bump: Default::default(),
            drop_cells: Cell::new(None),
        }
    }

    /// Converts a BumpVec into a borrowed slice in such a way that the drop code
    /// for T (if any) will be executed when the MemoryManager is dropped.
    pub(crate) fn unpack_slice<'a, T>(&'a self, vec: Vec<T, &'a dyn Allocator>) -> &'a [T] {
        unsafe {
            let ptr = vec.as_ptr();
            let len = vec.len();
            let slice = core::slice::from_raw_parts(ptr, len);
            let (dropper, _) = Box::into_non_null(Box::new_in(vec, &self.bump));
            let drop_cell = Box::new_in(
                DropCell {
                    // Rust doesn't know what the lifetime of this data is, but we do because
                    // we allocated it and own the allocator,
                    // so we transmute it to have the appropriate lifetime
                    dropper: transmute::<NonNull<dyn DeferDrop>, NonNull<dyn DeferDrop>>(
                        dropper as NonNull<dyn DeferDrop>,
                    ),
                    next: self.drop_cells.get(),
                },
                &self.bump,
            );
            let (drop_cell, _) = Box::into_non_null(drop_cell);
            self.drop_cells.set(Some(drop_cell));
            slice
        }
    }
}

unsafe impl Allocator for MemoryManager {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        self.bump.allocate(layout)
    }

    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        self.bump.allocate_zeroed(layout)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        self.bump.deallocate(ptr, layout)
    }

    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        self.bump.grow(ptr, old_layout, new_layout)
    }

    unsafe fn grow_zeroed(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        self.bump.grow_zeroed(ptr, old_layout, new_layout)
    }

    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        self.bump.shrink(ptr, old_layout, new_layout)
    }
}

impl Drop for MemoryManager {
    fn drop(&mut self) {
        let mut drop_cell = self.drop_cells.get();
        while let Some(cell) = drop_cell {
            unsafe {
                let cell = cell.as_ref();
                drop_in_place(cell.dropper.as_ptr());
                drop_cell = cell.next;
            }
        }
    }
}

trait DeferDrop {}
impl<T> DeferDrop for T {}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::format;
    use alloc::string::String;
    use allocator_api2::vec::Vec;

    struct NeedsDrop<'a> {
        x: String,
        drop_counter: &'a Cell<u32>,
    }

    impl<'a> Drop for NeedsDrop<'a> {
        fn drop(&mut self) {
            self.drop_counter.set(self.drop_counter.get() - 1);
        }
    }

    #[test]
    /// This test makes sure that all values that are exposed through unpack_slice are properly dropped.
    fn test_unpack_slice() {
        let mut drop_counter = Cell::new(0);
        {
            let mem = MemoryManager::default();
            let mut v: Vec<NeedsDrop, &dyn Allocator> = Vec::new_in(&mem);
            for i in 0..10 {
                drop_counter.set(drop_counter.get() + 1);
                v.push(NeedsDrop {
                    x: String::from(format!("x{}", i)),
                    drop_counter: &drop_counter,
                });
            }
            let slc = mem.unpack_slice(v);
            assert_eq!(slc.len(), 10);
            assert_eq!(drop_counter.get(), 10);
        }
        assert_eq!(drop_counter.get(), 0);
    }
}
