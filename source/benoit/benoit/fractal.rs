/*
	Copyright 2021, 2023 Gabriel Bjørnager Jensen.

	This file is part of Benoit.

	Benoit is free software: you can redistribute it
	and/or modify it under the terms of the GNU
	Affero General Public License as published by
	the Free Software Foundation, either version 3
	of the License, or (at your option) any later
	version.

	Benoit is distributed in the hope that it will
	be useful, but WITHOUT ANY WARRANTY; without
	even the implied warranty of MERCHANTABILITY or
	FITNESS FOR A PARTICULAR PURPOSE. See the GNU
	Affero General Public License for more details.

	You should have received a copy of the GNU
	Affero General Public License along with Benoit.
	If not, see <https://www.gnu.org/licenses/>.
*/

use crate::benoit::render::IteratorFunction;
use crate::benoit::render::iterate;

use std::mem::transmute;

pub struct Fractal {
	kind:    Kind,
	inverse: bool,
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Kind {
	BurningShip,
	Mandelbrot,
	Multibrot3,
	Tricorn,
}

impl Fractal {
	pub const fn new(kind: Kind, inverse: bool) -> Self {
		let fractal = Fractal {
			kind:    kind,
			inverse: inverse,
		};

		return fractal;
	}

	#[must_use]
	pub fn kind(&self) -> Kind {
		return self.kind;
	}

	#[must_use]
	pub fn inverse(&self) -> bool {
		return self.inverse;
	}

	#[must_use]
	pub fn exponent(&self) -> f32 {
		return match self.kind {
			Kind::BurningShip => 2.0,
			Kind::Mandelbrot  => 2.0,
			Kind::Multibrot3  => 3.0,
			Kind::Tricorn     => 2.0,
		};
	}

	pub fn iterator(&self) -> IteratorFunction {
		return match self.kind {
			Kind::BurningShip => iterate::burning_ship,
			Kind::Mandelbrot  => iterate::mandelbrot,
			Kind::Multibrot3  => iterate::multibrot3,
			Kind::Tricorn     => iterate::tricorn,
		};
	}

	pub fn set_kind(&mut self, kind: Kind) {
		self.kind = kind;
	}

	pub fn set_inverse(&mut self, inverse: bool) {
		self.inverse = inverse;
	}

	pub fn cycle(&mut self, direction: i8) {
		// Not important.
		debug_assert!(direction != 0x0);

		let raw = self.kind as i8 + direction;
		let raw: u8 = if raw < 0x0 {
			Kind::MAX
		} else if raw > Kind::MAX as i8 {
			0x0
		} else {
			raw as u8
		};

		let new: Kind = unsafe { transmute(raw) };

		self.kind = new;
	}
}

impl Kind {
	const MAX: u8 = Kind::Tricorn as u8;

	pub fn name(self) -> &'static str {
		return match self {
			Kind::BurningShip => "burning ship",
			Kind::Mandelbrot  => "mandelbrot set",
			Kind::Multibrot3  => "multibrot (d=3) set",
			Kind::Tricorn     => "tricorn",
		};
	}
}
