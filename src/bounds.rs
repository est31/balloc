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
