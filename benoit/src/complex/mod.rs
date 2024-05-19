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

//! Complex numbers.

mod from_str;

use rug::{Assign, Float};
use rug::complex::Prec;

/// Complex floating-point type.
///
/// This structure tries to emulate Rug's own `Complex`, but without boilerplate.
///
/// This is mainly done by not defining any arithmetic traits.
/// As we do have performance critical needs, we should instead use the parts (real and imaginary) directly, as doing this can also lead to optimisations on our end.
///
/// We still do, however, define the [`new`](Complex::new) and [`with_val`](Complex::with_val) constructors, altough the latter has a sligthly different syntax.
///
/// The alternative to defining this structure would be to use a tuple `(rug::Float, rug::Float)`.
#[derive(Clone, Debug, PartialEq)]
pub struct Complex {
	/// The real part of the complex number.
	pub real: Float,

	/// The imaginary part of the complex number.
	pub imag: Float,
}

impl Complex {
	/// Constructs a new complex number with the value zero.
	#[must_use]
	pub fn new<P: Prec>(prec: P) -> Self {
		let prec = prec.prec();

		let real = Float::new(prec.0);
		let imag = Float::new(prec.1);

		Self { real, imag }
	}

	/// Constructs a new complex number with the specified value.
	#[must_use]
	pub fn with_val<P: Prec, T, U>(prec: P, real: T, imag: U) -> Self
	where
		Float: Assign<T> + Assign<U>, {
		let prec = prec.prec();

		let real = Float::with_val(prec.0, real);
		let imag = Float::with_val(prec.1, imag);

		Self { real, imag }
	}
}
