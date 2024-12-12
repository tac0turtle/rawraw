use allocator_api2::alloc::{AllocError, Allocator, Global};
use core::alloc::Layout;
use core::cell::Cell;
use core::cmp::max;
use core::ptr::NonNull;

// Very simple, custom bump allocator to avoid third party dependencies,
// reduce code size, and customize where chunks are allocated from and their sizes.
pub struct BumpAllocator<A: Allocator = Global> {
    // the current chunk that is being allocated, if any
    cur: Cell<Option<NonNull<Footer>>>,
    // the base allocator to use for allocating chunks
    base_allocator: A,
}

// a footer describing the chunk that is at the end of the chunk
struct Footer {
    // the start of the chunk that originally got allocated
    start: NonNull<u8>,
    // the current allocation position in the chunk
    pos: Cell<NonNull<u8>>,
    // a pointer to the footer of the previous chunk used in this allocator
    prev: Option<NonNull<Footer>>,
    // the layout of the chunk
    layout: Layout,
}

const FOOTER_SIZE: usize = size_of::<Footer>();

impl<A: Allocator + Default> Default for BumpAllocator<A> {
    fn default() -> Self {
        Self {
            cur: Cell::new(None),
            base_allocator: Default::default(),
        }
    }
}

impl<A: Allocator> BumpAllocator<A> {
    pub fn new(base_allocator: A) -> Self {
        Self {
            cur: Cell::new(None),
            base_allocator,
        }
    }
}

unsafe impl<A: Allocator> Allocator for BumpAllocator<A> {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            match self.cur.get() {
                None => {
                    const START_SIZE: usize = 4096;
                    self.alloc_chunk(START_SIZE, layout)
                }
                Some(mut footer) => {
                    // finding the starting allocation position
                    let pos = footer.as_ref().pos.get();
                    // align to layout.align()
                    let offset = pos.align_offset(layout.align());
                    // add offset to pos
                    let pos = pos.add(offset);
                    // compute the new position for allocation
                    let new_pos = pos.add(layout.size());
                    // check if the new position is before the footer in the chunk
                    if new_pos <= footer.cast() {
                        // update the position in the footer
                        footer.as_mut().pos.set(new_pos);
                        // return the allocated slice
                        Ok(NonNull::slice_from_raw_parts(pos, layout.size()))
                    } else {
                        self.alloc_chunk(footer.as_ref().layout.size() * 2, layout)
                    }
                }
            }
        }
    }

    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
        // we don't need to deallocate, because this is a bump allocator
        // and we deallocate everything at once when the allocator is dropped
    }

    // TODO: attempt to extend the memory block in place
    // unsafe fn grow(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
    //     todo!()
    // }
}

impl<A: Allocator> BumpAllocator<A> {
    unsafe fn alloc_chunk(
        &self,
        start_size: usize,
        layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let mut size = start_size;
        // the minimum size is the size of the layout plus the size of the footer
        let needed = layout.size() + FOOTER_SIZE;
        while size < needed {
            size <<= 1;
        }
        // we align to either he needed alignment or at least 16
        let align = max(layout.align(), 16);
        // the layout of the chunk
        let chunk_layout = Layout::from_size_align(size, align).map_err(|_| AllocError)?;
        // allocate the chunk, this will also be the newly allocated memory for the layout
        let start = self.base_allocator.allocate(chunk_layout)?.cast::<u8>();
        // update the allocation position
        let pos = start.add(layout.size());
        // find the end of the chunk
        let end = start.add(size);
        // the footer is at the end of the chunk
        let footer = end.sub(FOOTER_SIZE).cast::<Footer>();
        // TODO make sure the footer is at an aligned position
        assert_eq!(0, footer.align_offset(align_of::<Footer>()));
        // write the footer
        footer.write(Footer {
            start,
            pos: Cell::new(pos),
            // the previous footer is the current footer
            prev: self.cur.get(),
            layout: chunk_layout,
        });
        // update the current footer
        self.cur.set(Some(footer));
        Ok(NonNull::slice_from_raw_parts(start, layout.size()))
    }
}

impl<A: Allocator> Drop for BumpAllocator<A> {
    fn drop(&mut self) {
        let mut maybe_footer = self.cur.get();
        while let Some(footer) = maybe_footer {
            let footer = unsafe { footer.as_ref() };
            maybe_footer = footer.prev;
            unsafe {
                self.base_allocator.deallocate(footer.start, footer.layout);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::alloc::Layout;
    use alloc::collections::BTreeSet;
    use core::cell::RefCell;
    use proptest::collection::vec;
    use proptest::prelude::*;
    use rangemap::RangeSet;

    fn layout() -> impl Strategy<Value = Layout> {
        (1usize..=4194304).prop_flat_map(|size| {
            (0u32..16).prop_map(move |align_exp| {
                Layout::from_size_align(size, 2usize.pow(align_exp)).unwrap()
            })
        })
    }
    proptest! {
         #[test]
         fn test_alloc(layouts in vec(layout(), 1..100)) {
            test_proper_allocations(Global, layouts);
         }
    }

    proptest! {
        #[test]
        fn test_dealloc(layouts in vec(layout(), 1..100)) {
            #[derive(Default)]
            struct TrackingAllocator {allocations: RefCell<BTreeSet<NonNull<[u8]>>>}
            unsafe impl <'a> Allocator for &'a TrackingAllocator {
                fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
                    let allocation = Global.allocate(layout)?;
                    self.allocations.borrow_mut().insert(allocation);
                    Ok(allocation)
                }

                unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
                    self.allocations.borrow_mut().remove(&NonNull::slice_from_raw_parts(ptr, layout.size()));
                    Global.deallocate(ptr, layout)
                }
            }

            let base_allocator = TrackingAllocator::default();
            test_proper_allocations(&base_allocator, layouts);
            assert_eq!(base_allocator.allocations.borrow().len(), 0);
        }
    }

    fn test_proper_allocations<A: Allocator>(base_allocator: A, layouts: alloc::vec::Vec<Layout>) {
        let bump = BumpAllocator::new(base_allocator);
        let mut alloc_ranges: RangeSet<NonNull<u8>> = RangeSet::new();
        for l in layouts {
            let ptr = bump.allocate_zeroed(l).unwrap();
            // check expected size
            let len = l.size();
            assert_eq!(ptr.len(), len);
            // check expected alignment
            assert_eq!((ptr.as_ptr() as *const u8 as usize) % l.align(), 0);
            // check that the pointer doesn't overlap with any other pointer
            let start = ptr.cast::<u8>();
            let end = unsafe { start.add(len) };
            let range = start..end;
            assert!(!alloc_ranges.overlaps(&range));
            alloc_ranges.insert(range);
            // check that the memory is zeroed
            let all_zero = alloc::vec![0u8; len];
            let bz = unsafe { &*ptr.as_ptr() };
            assert_eq!(bz, &all_zero);
        }
    }
}
