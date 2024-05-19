/*
	Copyright 2021, 2023-2024 Gabriel Bjørnager Jen-
	sen.

	This file is part of benoit.

	benoit is free software: you can redistribute it
	and/or modify it under the terms of the GNU Af-
	fero General Public License as published by the
	Free Software Foundation, either version 3 of
	the License, or (at your option) any later ver-
	sion.

	benoit is distributed in the hope that it will
	be useful, but WITHOUT ANY WARRANTY; without
	even the implied warranty of MERCHANTABILITY or
	FITNESS FOR A PARTICULAR PURPOSE. See the GNU
	Affero General Public License for more details.

	You should have received a copy of the GNU Af-
	fero General Public License along with benoit.
	If not, see <https://www.gnu.org/licenses/>.
*/

//! Utilites for rendering the Mandelbrot Set etc.
//!
//! This library is mostly intended for use by either `benoit-cli` or `benoit-gui`, but could be used by any programme.
//! Do note that API changed can come without notice.

macro_rules! use_mod {
	($visibility:vis, $name:ident) => {
		mod $name;
		$visibility use $name::*;
	};
}
pub(in crate) use use_mod;

pub mod colour;
pub mod complex;
pub mod error;
pub mod fractal;
pub mod palette;
pub mod render;
pub mod stopwatch;

/// The version number of benoit, currently being `3.0.0`.
///
/// The version numbers of `benoit-cli` and `benoit-gui` match this value.
pub const VERSION: (u32, u32, u32) = (0x3, 0x0, 0x1);

/// The precision used for arbitrary precision computations, in bits.
///
/// This is to be used at all times when creating [`rug::Float`] objects for use with benoit.
pub const PRECISION: u32 = 0x100;

/// Logs to `stderr` using predefined formats.
///
/// ## `debug`
///
/// Only logs if debug assertions are activated, i.e. if compiling in debug mode.
///
/// This check is simple done by testing `debug_assertions`.
///
/// ## `error`
///
/// Prints the message as specified by `$format` as if passed directly to [`format`].
///
/// ## `value`
///
/// Prints the value of `$value` using the [`Debug`](std::fmt::Debug) trait.
#[macro_export]
macro_rules! log {
	(debug, $($format:tt)*) => {{
		if cfg!(debug_assertions) { eprintln!("\u{001B}[93m{}\u{001B}[0m", format!($($format)?)) };
	}};

	(error, $($format:tt)*) => {{
		eprintln!("\u{001B}[91merror\u{001B}[0m: {}", format!($($format)?));
	}};

	(value, $value:expr) => {{
		$crate::log!(debug, "{:?}", $value);
	}};
}
