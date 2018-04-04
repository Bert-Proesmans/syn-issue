//! Case two
//! Outer doc (sugar) +- inner attribute
//! == ALL OK

use proc_macro::value_from_type;

/// Outer doc for module `two`.
// == OK
pub mod two_one {
	#![value_from_type(TwoItem)]

	/// Docs for struct.
	pub struct Two();
}

/// Outer doc for module `two`.
// == OK
pub mod two_two {
	#![allow(dead_code)]
	#![value_from_type(TwoItem)]

	/// Docs for struct.
	pub struct Two();
}
