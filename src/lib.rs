/*!
Bounded allocation datastructures
*/

pub mod bounds;

use bounds::AllocBound;
use std::mem::size_of;

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
