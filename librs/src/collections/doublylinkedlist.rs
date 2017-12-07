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


pub struct DoublyLinkedList<T> {
	pub head: *mut Node<T>,
	pub tail: *mut Node<T>,
}

#[derive(Clone, Copy)]
pub struct Node<T> {
	pub value: T,
	pub prev: *mut Node<T>,
	pub next: *mut Node<T>,
}

impl<T> Node<T> {
	pub const fn new(value: T) -> Self {
		Self { value: value, prev: 0 as *mut Node<T>, next: 0 as *mut Node<T> }
	}
}

impl<T> DoublyLinkedList<T> {
	pub const fn new() -> Self {
		Self { head: 0 as *mut Node<T>, tail: 0 as *mut Node<T> }
	}

	pub fn push(&mut self, new_node: *mut Node<T>) {
		unsafe {
			assert!(!new_node.is_null());

			(*new_node).prev = self.tail;
			(*new_node).next = 0 as *mut Node<T>;
			self.tail = new_node;

			if (*new_node).prev.is_null() {
				self.head = new_node;
			} else {
				(*(*new_node).prev).next = new_node;
			}
		}
	}

	pub fn insert_before(&mut self, new_node: *mut Node<T>, node: *mut Node<T>) {
		unsafe {
			assert!(!new_node.is_null());
			assert!(!node.is_null());

			(*new_node).prev = (*node).prev;
			(*new_node).next = node;
			(*node).prev = new_node;

			if (*new_node).prev.is_null() {
				self.head = new_node;
			} else {
				(*(*new_node).prev).next = new_node;
			}
		}
	}

	pub fn insert_after(&mut self, new_node: *mut Node<T>, node: *mut Node<T>) {
		unsafe {
			assert!(!new_node.is_null());
			assert!(!node.is_null());

			(*new_node).prev = node;
			(*new_node).next = (*node).next;
			(*node).next = new_node;

			if (*new_node).next.is_null() {
				self.tail = new_node;
			} else {
				(*(*new_node).next).prev = new_node;
			}
		}
	}

	pub fn remove(&mut self, node: *mut Node<T>) {
		unsafe {
			assert!(!node.is_null());

			if (*node).prev.is_null() {
				self.head = (*node).next;
			} else {
				(*(*node).prev).next = (*node).next;
			}

			if (*node).next.is_null() {
				self.tail = (*node).prev;
			} else {
				(*(*node).next).prev = (*node).prev;
			}

			(*node).prev = 0 as *mut Node<T>;
			(*node).next = 0 as *mut Node<T>;
		}
	}

	pub fn iter(&self) -> Iter<T> {
		Iter::<T> { current: self.head }
	}
}

pub struct Iter<T> {
	current: *mut Node<T>
}

impl<T> Iterator for Iter<T> {
	type Item = *mut Node<T>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current.is_null() {
			None
		} else {
			let node = self.current;
			self.current = unsafe { (*self.current).next };
			Some(node)
		}
	}
}
