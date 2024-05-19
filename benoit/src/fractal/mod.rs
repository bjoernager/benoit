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

//! Fractals.

mod iters;

use crate::complex::Complex;
use crate::error::Error;

use std::str::FromStr;

/// Defines a fractal.
#[derive(Clone, Debug)]
pub struct Fractal {
	pub iter: fn(&mut Complex, &Complex),

	pub exponent: f64,
}

impl Fractal {
	const ANTIBROT: Self = Self {
		iter: iters::iter_antibrot,

		exponent: 3.0, // But doesn't look right?
	};

	const BURNING_SHIP: Self = Self {
		iter: iters::iter_burning_ship,

		exponent: 2.0,
	};

	const MANDELBROT: Self = Self {
		iter: iters::iter_mandelbrot,

		exponent: 2.0,
	};

	const MULTIBROT3: Self = Self {
		iter: iters::iter_multibrot3,

		exponent: 3.0,
	};

	const MULTIBROT4: Self = Self {
		iter: iters::iter_multibrot4,

		exponent: 4.0,
	};

	const TRICORN: Self = Self {
		iter: iters::iter_tricorn,

		exponent: 2.0,
	};
}

impl Default for Fractal {
	fn default() -> Self { Self::MANDELBROT }
}

impl FromStr for Fractal {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			| "mandelbrot"
			| "multibrot2"
			=> Ok(Self::MANDELBROT),

			| "antibrot"
			| "antibrot2"
			=> Ok(Self::ANTIBROT),

			"burning_ship" => Ok(Self::BURNING_SHIP),
			"multibrot3"   => Ok(Self::MULTIBROT3),
			"multibrot4"   => Ok(Self::MULTIBROT4),
			"tricorn"      => Ok(Self::TRICORN),

			_ => Err(Error::UnknownFractal { string: s.to_owned() })
		}
	}
}

impl PartialEq<Self> for Fractal {
	fn eq(&self, other: &Self) -> bool {
		self.iter == other.iter
	}
}
