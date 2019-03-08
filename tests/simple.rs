#[macro_use]
extern crate balloc;

use balloc::{Bvec, NumberBounded};

#[test]
fn test_bvec_ops() {
	let b = NumberBounded::new_wrapped(42);
	let mut v = Bvec::with_capacity(b, 42).unwrap();
	v.push(0u8).unwrap();
	v.push(2u8).unwrap();
	v.push(3u8).unwrap();
	v.push(4u8).unwrap();
	// Test for Index + IndexMut implementations
	assert_eq!(v[0], 0);
	v[0] = 1;
	assert_eq!(v[0], 1);
	// Test for Index implementation
	assert_eq!(&v[..], &[1, 2, 3, 4]);
	// Test for Deref implementation
	assert_eq!({ let sl :&[_] = &v; sl }, &[1, 2, 3, 4]);
}

#[test]
fn test_bvec_macro() {
	let b = NumberBounded::new_wrapped(42);
	let v = bvec![b; 0u8; 42].unwrap();
	assert_eq!(&v[..], &[0u8; 42][..]);
}
