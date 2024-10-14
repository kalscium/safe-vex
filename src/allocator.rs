//! Defines the global allocator for the PROS runtime

use core::alloc::Layout;
use newlib_alloc::Alloc;

/// Sets the global allocator
#[global_allocator]
static ALLOCATOR: Alloc = Alloc;

#[alloc_error_handler]
fn handle(layout: Layout) -> ! {
    panic!("memory allocation failed: {:#?}", layout);
}
