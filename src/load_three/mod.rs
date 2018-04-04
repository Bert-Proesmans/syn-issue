//! Case three
//! Outer doc (sugar) +- inner attribute [seperate modules]
//! == ALL OK

use proc_macro::value_from_type;

/// Outer doc for three_one.
// == OK
pub mod three_one;

/// Outer doc for three_two.
// == OK
pub mod three_two;


