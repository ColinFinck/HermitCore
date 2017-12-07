// Copyright (c) 2017 Colin Finck, RWTH Aachen University
//
// MIT License
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! Implementation of the HermitCore Allocator for dynamically allocating heap memory
//! in the kernel.
//!
//! The data structures used to manage heap memory require dynamic memory allocations
//! themselves. To solve this chicken-egg problem, HermitCore first uses a
//! "Bootstrap Allocator". This is a simple single-threaded implementation using some
//! preallocated space within KERNEL_START_ADDRESS and KERNEL_END_ADDRESS, along with an
//! index variable. Freed memory is never reused, but this can be neglected for bootstrapping.
//!
//! As soon as all required data structures have been set up, the "System Allocator" is used.
//! It manages all memory >= KERNEL_END_ADDRESS.

use alloc::heap::{Alloc, AllocErr, Layout};
use arch::mm::paging::{BasePageSize, PageSize};
use mm;

static mut IS_INITIALIZED: bool = false;

pub struct HermitAllocator;

unsafe impl<'a> Alloc for &'a HermitAllocator {
	unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
		unsafe { assert!(IS_INITIALIZED, "Attempt to allocate before HermitAllocator is initialized! You must not use Boxed types before MM initialization."); }
		debug!("Allocating {} bytes", layout.size());

		Ok(mm::allocate(layout.size()) as *mut u8)
	}

	unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
		unsafe { assert!(IS_INITIALIZED, "Attempt to allocate before HermitAllocator is initialized! You must not use Boxed types before MM initialization."); }
		let virtual_address = ptr as usize;
		debug!("Deallocating {} bytes at {:#X}", layout.size(), virtual_address);

		mm::deallocate(virtual_address, layout.size());
	}
}

pub fn init() {
	unsafe { IS_INITIALIZED = true; }
}
