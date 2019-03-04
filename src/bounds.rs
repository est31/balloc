use super::AllocError;

pub trait AllocBound {
	fn try_alloc(&mut self, amount :usize) -> Result<(), AllocError>;
	fn dealloc(&mut self, amount :usize);
}

pub struct Unbounded;

impl AllocBound for Unbounded {
	fn try_alloc(&mut self, _amount :usize) -> Result<(), AllocError> {
		Ok(())
	}
	fn dealloc(&mut self, _amount :usize) {
		// Nothing to do.
	}
}

pub struct NumberBounded(usize);

impl NumberBounded {
	pub fn new(bound :usize) -> Self {
		NumberBounded(bound)
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
