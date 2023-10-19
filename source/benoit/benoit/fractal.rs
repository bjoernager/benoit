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

pub mod from_str;

mod iterate;

pub type IteratorFunction = fn(&mut Complex, &Complex);

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum FractalKind {
	Mandelbrot,
	Multibrot3,
	Multibrot4,
	BurningShip,
	Tricorn,
}

impl FractalKind {
	pub fn name(self) -> &'static str {
		return match self {
			FractalKind::BurningShip => "burning ship",
			FractalKind::Mandelbrot  => "mandelbrot set",
			FractalKind::Multibrot3  => "multibrot3 set",
			FractalKind::Multibrot4  => "multibrot4 set",
			FractalKind::Tricorn     => "tricorn",
		};
	}

	pub fn cycle(&mut self, direction: i8) {
		let raw = *self as i16 + direction as i16;

		const NUM: isize = FractalKind::Tricorn as isize + 0x1;
		let new: u8 = match raw as isize {
			-0x1 => (NUM - 0x1) as u8,
			NUM  => 0x0,
			_    => raw as u8,
		};

		*self = unsafe { transmute(new) };
	}
}

#[derive(Clone, Copy)]
pub struct Fractal {
	pub kind:    FractalKind,
	pub inverse: bool,
	pub julia:   bool,
}

impl Fractal {
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
}
