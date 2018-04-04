//! Case four
//! Outer doc (sugar) + inner doc (sugar) +- inner attribute [seperate modules]
//! == OK

use proc_macro::value_from_type;

/// Outer doc for four_one.
// == OK
pub mod four_one;

/// Outer doc for four_two.
// == Fails to parse (ItemMod)
pub mod four_two;


