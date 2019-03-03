use super::AllocError;

pub trait AllocBound {
	fn try_alloc(&mut self, amount :usize) -> Result<(), AllocError>;
	fn dealloc(&mut self, amount :usize);
}
