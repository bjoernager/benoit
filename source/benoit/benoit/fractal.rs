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
	julia:   bool,
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum FractalKind {
	// Sorted according to exponent.
	Mandelbrot,
	BurningShip,
	Tricorn,
	Multibrot3,
	Multibrot4,
}

pub type IteratorFunction = fn(&mut Complex, &Complex);

impl Fractal {
	#[must_use]
	pub const fn new(kind: FractalKind, inverse: bool, julia: bool) -> Self {
		let fractal = Fractal {
			kind:    kind,
			inverse: inverse,
			julia:   julia,
		};

		return fractal;
	}

	pub fn name(&self) -> String {
		let kind = self.kind.name();

		let extra = if self.inverse && !self.julia {
			"inverse"
		} else if !self.inverse && self.julia {
			"julia"
		} else if self.inverse && self.julia {
			"inverse julia"
		} else {
			"normal"
		};

		let name = format!("{kind} ({extra})");
		return name;
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
	pub fn julia(&self) -> bool {
		return self.julia;
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

	pub fn set_julia(&mut self, julia: bool) {
		self.julia = julia;
	}

	pub fn cycle(&mut self, direction: i8) {
		// Not important.
		debug_assert!(direction != 0x0);

		let raw = self.kind as i8 + direction;

		let new: FractalKind = if raw < 0x0 {
			FractalKind::MAX
		} else if raw > FractalKind::MAX as i8 {
			FractalKind::MIN
		} else {
			unsafe { transmute(raw) }
		};

		let new: FractalKind = unsafe { transmute(new) };

		self.kind = new;
	}
}

impl FractalKind {
	const MIN: FractalKind = FractalKind::Mandelbrot;
	const MAX: FractalKind = FractalKind::Multibrot4;

	pub fn name(self) -> &'static str {
		return match self {
			FractalKind::BurningShip => "burning ship",
			FractalKind::Mandelbrot  => "mandelbrot set",
			FractalKind::Multibrot3  => "multibrot3 set",
			FractalKind::Multibrot4  => "multibrot4 set",
			FractalKind::Tricorn     => "tricorn",
		};
	}
}
