//! Case one
//! Inner doc (sugar) +- inner macro

use proc_macro::value_from_type;

// OK
pub mod one_one {
	//! Inner doc for module `one`.
	#![value_from_type(OneItem)]

	/// Docs for struct.
	pub struct One();
}

// Fails to parse (ItemMod)
pub mod one_two {
	//! Inner doc for module `one`.
	#![allow(dead_code)]
	#![value_from_type(OneItem)]

	/// Docs for struct.
	pub struct One();
}
