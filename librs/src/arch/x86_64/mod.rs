// Copyright (c) 2017 Stefan Lankes, RWTH Aachen University
//                    Colin Finck, RWTH Aachen University
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

pub mod apic;
pub mod gdt;
pub mod idt;
pub mod irq;
pub mod mm;
pub mod percore;
pub mod pic;
pub mod pit;
pub mod processor;

use alloc::vec::Vec;
use synch::spinlock::*;


extern "C" {
	static mut cpu_online: u32;
}

lazy_static! {
	static ref CPU_ONLINE: Spinlock<&'static mut u32> =
		Spinlock::new(unsafe { &mut cpu_online });
}

static TEST_ARRAY: [usize; 10] = [150, 30, 77, 33, 55, 4, 8, 79, 231, 476];


// FUNCTIONS
pub fn boot_processor_init() {
	percore::init();
	gdt::install();
	idt::install();
	processor::detect_features();
	processor::configure();
	::mm::init();
	::mm::print_information();
	pic::remap();
	pic::mask_all();
	irq::install();
	irq::enable();
	processor::detect_frequency();
	processor::print_information();

	**CPU_ONLINE.lock() += 1;

	apic::init();
	apic::print_information();

	loop {
		info!("Moin");
		processor::udelay(5_000_000);
	}

	/*unsafe {
		signal_init();
	}*/
}

pub fn application_processor_init() {
	percore::init();
	debug!("Application Processor Initializing");

	gdt::install();
	idt::install();
	processor::configure();
	apic::init_x2apic();
	apic::init_local_apic();
	irq::enable();

	**CPU_ONLINE.lock() += 1;

	for i in 0..10 {
		let mut vec1 = Vec::<u8>::new();
		let mut vec2 = Vec::<u8>::new();
		let mut vec3 = Vec::<u8>::new();

		drop(vec2);

		for j in 1..TEST_ARRAY[i] {
			vec1.push(11);
		}

		for j in 1..TEST_ARRAY[i] {
			vec3.push(55);
		}

		info!("Completed test with {} elements", TEST_ARRAY[i]);
	}

	mm::physicalmem::print_information();
	mm::virtualmem::print_information();
}
