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
use std::ops::Add;

#[derive(Clone, Copy)]
pub enum Fractal {
	BurningShip,
	Mandelbrot,
	Multibrot3,
	Tricorn,
}

impl Fractal {
	const MAX: u8 = Fractal::Tricorn as u8;

	pub fn get_name(self) -> &'static str {
		return match self {
			Fractal::BurningShip => "burning ship",
			Fractal::Mandelbrot  => "mandelbrot set",
			Fractal::Multibrot3  => "multibrot (d=3)",
			Fractal::Tricorn     => "tricorn",
		};
	}

	pub fn get_exponent(self) -> f32 {
		return match self {
			Fractal::BurningShip => 2.0,
			Fractal::Mandelbrot  => 2.0,
			Fractal::Multibrot3  => 3.0,
			Fractal::Tricorn     => 2.0,
		};
	}

	pub fn get_iterator(self) -> IteratorFunction {
		return match self {
			Fractal::BurningShip => iterate::burning_ship,
			Fractal::Mandelbrot  => iterate::mandelbrot,
			Fractal::Multibrot3  => iterate::multibrot3,
			Fractal::Tricorn     => iterate::tricorn,
		};
	}

}

impl Add<i8> for Fractal {
	type Output = Fractal;

	fn add(self, direction: i8) -> Self {
		assert!(direction != 0x0);

		let raw = self as i8 + direction;
		let raw: u8 = if raw < 0x0 {
			Fractal::MAX
		} else if raw > Fractal::MAX as i8 {
			0x0
		} else {
			raw as u8
		};

		let new: Self = unsafe { transmute(raw) };

		return new;
	}
}
