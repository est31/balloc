# balloc

**Bounded allocation for Rust.**

This crate provides a wrapper over `Vec<T>` so that boundedness constraints can be enforced.

Such constraints are useful when dealing with untrusted input where the other party
sends you a zip bomb or similar. Either, you rely on operating system facilities to
handle your program when it crashes, or you use balloc with reasonable bounds to
prevent the OS having to get involved in the first place.

The crate is designed to use language features to make doing the checks wrongly
as hard as possible.

Right now, this crate only provides `Vec<T>` wrappers.
At a later point in time, wrappers for other data structures of std fame might be added.

## License

Licensed under Apache 2 or MIT (at your option). For details, see the [LICENSE](LICENSE) file.

### License of your contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for
inclusion in the work by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
