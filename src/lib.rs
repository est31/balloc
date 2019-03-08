/*!
Bounded allocation datastructures
*/

mod bounds;

pub use bounds::{AllocBound, Unbounded, NumberBounded};

use std::mem::size_of;
use std::borrow::{Borrow, BorrowMut};
use std::ops::{Index, IndexMut, Deref};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct AllocError;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
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
	pub fn push(&mut self, value :T) -> Result<(), AllocError> {
		if self.inner.capacity() == self.inner.len() {
			let to_alloc = self.inner.capacity();
			try!(self.bound.try_alloc(to_alloc));
			self.inner.push(value);
			debug_assert_eq!(self.inner.capacity(), to_alloc * 2);
		} else {
			self.inner.push(value);
		}
		Ok(())
	}
}
impl<B :AllocBound, T :Clone> Bvec<B, T> {
	pub fn from_elem(mut bound :B, elem :T, count :usize) -> Result<Self, AllocError> {
		try!(bound.try_alloc(count * size_of::<T>()));
		Ok(Self {
			inner : vec![elem; count],
			bound,
		})
	}
}

impl<B :AllocBound, T> Drop for Bvec<B, T> {
	fn drop(&mut self) {
		self.bound.dealloc(self.inner.capacity());
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

impl<B :AllocBound, T> Deref for Bvec<B, T> {
	type Target = [T];

	fn deref(&self) -> &[T] {
		&self.inner
	}
}

impl<B :AllocBound, T, I> Index<I> for Bvec<B, T> where Vec<T> :Index<I> {
	type Output = <Vec<T> as Index<I>>::Output;
	fn index(&self, index :I) -> &Self::Output {
		Index::index(&self.inner, index)
	}
}

impl<B :AllocBound, T, I> IndexMut<I> for Bvec<B, T>
		where Vec<T> :IndexMut<I> + Index<I> {
	fn index_mut(&mut self, index :I) -> &mut Self::Output {
		<Vec<T> as IndexMut<I>>::index_mut(&mut self.inner, index)
	}
}

#[macro_export]
macro_rules! bvec {
	{$b:expr; $v:expr; $cnt:expr} => {
		$crate::Bvec::from_elem($b, $v, $cnt)
	};
}
