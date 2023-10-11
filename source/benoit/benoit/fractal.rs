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

use crate::benoit::complex::Complex;

use std::mem::transmute;

mod iterate;

#[derive(Clone, Copy)]
pub struct Fractal {
	kind:    FractalKind,
	inverse: bool,
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum FractalKind {
	BurningShip,
	Mandelbrot,
	Multibrot3,
	Multibrot4,
	Tricorn,
}

pub type IteratorFunction = fn(&mut Complex, &Complex);

impl Fractal {
	#[must_use]
	pub const fn new(kind: FractalKind, inverse: bool) -> Self {
		let fractal = Fractal {
			kind:    kind,
			inverse: inverse,
		};

		return fractal;
	}

	#[must_use]
	pub fn kind(&self) -> FractalKind {
		return self.kind;
	}

	#[must_use]
	pub fn inverse(&self) -> bool {
		return self.inverse;
	}

	#[must_use]
	pub fn exponent(&self) -> f32 {
		return match self.kind {
			FractalKind::BurningShip => 2.0,
			FractalKind::Mandelbrot  => 2.0,
			FractalKind::Multibrot3  => 3.0,
			FractalKind::Multibrot4  => 4.0,
			FractalKind::Tricorn     => 2.0,
		};
	}

	#[must_use]
	pub fn iterator(&self) -> IteratorFunction {
		return match self.kind {
			FractalKind::BurningShip => iterate::burning_ship,
			FractalKind::Mandelbrot  => iterate::mandelbrot,
			FractalKind::Multibrot3  => iterate::multibrot3,
			FractalKind::Multibrot4  => iterate::multibrot4,
			FractalKind::Tricorn     => iterate::tricorn,
		};
	}

	pub fn set_kind(&mut self, kind: FractalKind) {
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
			FractalKind::MAX
		} else if raw > FractalKind::MAX as i8 {
			0x0
		} else {
			raw as u8
		};

		let new: FractalKind = unsafe { transmute(raw) };

		self.kind = new;
	}
}

impl FractalKind {
	const MAX: u8 = FractalKind::Tricorn as u8;

	pub fn name(self) -> &'static str {
		return match self {
			FractalKind::BurningShip => "burning ship",
			FractalKind::Mandelbrot  => "mandelbrot set",
			FractalKind::Multibrot3  => "multibrot (d=3) set",
			FractalKind::Multibrot4  => "multibrot (d=4) set",
			FractalKind::Tricorn     => "tricorn",
		};
	}
}
