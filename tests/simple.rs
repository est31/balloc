extern crate balloc;

use balloc::{Bvec, NumberBounded};

#[test]
fn test_bvec_ops() {
	let b = NumberBounded::new_wrapped(42);
	let mut v = Bvec::with_capacity(b, 42).unwrap();
	v.push(1u8).unwrap();
	v.push(2u8).unwrap();
	v.push(3u8).unwrap();
	v.push(4u8).unwrap();
	assert_eq!({ let sl :&[_] = &v[..]; sl }, &[1, 2, 3, 4]);
}
