/*!
Bounded allocation datastructures
*/

pub mod bounds;

use bounds::AllocBound;
use std::mem::size_of;
use std::borrow::{Borrow, BorrowMut};
use std::ops::{Index,IndexMut};
use std::slice::SliceIndex;

pub struct AllocError;

pub struct Bvec<B :AllocBound, T> {
	inner :Vec<T>,
	bound :B,
}

impl<B :AllocBound, T> Bvec<B, T> {
	pub fn with_capacity(mut bound :B, capacity :usize) -> Result<Self, AllocError> {
		try!(bound.try_alloc(capacity * size_of::<T>()));
		Ok(Self {
			inner : Vec::with_capacity(capacity),
			bound,
		})
	}
}

impl<B :AllocBound, T> Borrow<[T]> for Bvec<B, T> {
	fn borrow(&self) -> &[T] {
		&self.inner
	}
}

impl<B :AllocBound, T> BorrowMut<[T]> for Bvec<B, T> {
	fn borrow_mut(&mut self) -> &mut [T] {
		&mut self.inner
	}
}

impl<B :AllocBound, T, I :SliceIndex<[T]>> Index<I> for Bvec<B, T> {
	type Output = I::Output;
	fn index(&self, index :I) -> &Self::Output {
		Index::index(&self.inner, index)
	}
}

impl<B :AllocBound, T, I :SliceIndex<[T]>> IndexMut<I> for Bvec<B, T> {
	fn index_mut(&mut self, index :I) -> &mut Self::Output {
		IndexMut::index_mut(&mut *self.inner, index)
	}
}
