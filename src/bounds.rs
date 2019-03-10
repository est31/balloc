use super::AllocError;
use std::rc::Rc;
use std::cell::{RefCell, Cell};

pub trait AllocBound {
	fn try_alloc(&mut self, amount :usize) -> Result<(), AllocError>;
	fn dealloc(&mut self, amount :usize);
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Unbounded;

impl AllocBound for Unbounded {
	fn try_alloc(&mut self, _amount :usize) -> Result<(), AllocError> {
		Ok(())
	}
	fn dealloc(&mut self, _amount :usize) {
		// Nothing to do.
	}
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct NumberBounded(usize);

impl NumberBounded {
	pub fn new_wrapped(bound :usize) -> Rc<Cell<Self>> {
		Rc::new(Cell::new(NumberBounded(bound)))
	}
}

impl AllocBound for NumberBounded {
	fn try_alloc(&mut self, amount :usize) -> Result<(), AllocError> {
		match self.0.checked_sub(amount) {
			Some(val) => {
				self.0 = val;
				Ok(())
			},
			None => {
				Err(AllocError)
			}
		}
	}
	fn dealloc(&mut self, amount :usize) {
		self.0 = self.0.saturating_add(amount);
	}
}

impl<B :AllocBound> AllocBound for Rc<RefCell<B>> {
	fn try_alloc(&mut self, amount :usize) -> Result<(), AllocError> {
		let b :&mut B = &mut *self.borrow_mut();
		try!(b.try_alloc(amount));
		Ok(())
	}
	fn dealloc(&mut self, amount :usize) {
		let b :&mut B = &mut *self.borrow_mut();
		b.dealloc(amount);
	}
}

pub type NumberBoundedRc = Rc<Cell<NumberBounded>>;

impl<B :AllocBound + Copy> AllocBound for Rc<Cell<B>> {
	fn try_alloc(&mut self, amount :usize) -> Result<(), AllocError> {
		let mut val = self.get();
		try!(val.try_alloc(amount));
		self.set(val);
		Ok(())
	}
	fn dealloc(&mut self, amount :usize) {
		let mut val = self.get();
		val.dealloc(amount);
		self.set(val);
	}
}
