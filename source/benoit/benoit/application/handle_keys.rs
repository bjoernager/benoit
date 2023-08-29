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

use crate::benoit::{Fractal, PRECISION};
use crate::benoit::application::Application;

extern crate rug;
extern crate sdl2;

use rug::Float;
use sdl2::keyboard::Scancode;

impl Application {
	pub fn handle_keys(&mut self, scan_code: Scancode) -> bool {
		let print_values = |fractal: Fractal, center_real: &Float, center_imaginary: &Float, zoom: &Float, maximum_iteration_count: u32, julia_real: &Float, julia_imaginary: &Float| {
			let base = format!("c = {center_real}{center_imaginary:+}i -- {zoom}x @ {maximum_iteration_count} iter.");

			match fractal {
				Fractal::Julia      => eprintln!("{base}:\n  c(M) = {julia_real}{julia_imaginary:+}i"),
				Fractal::Mandelbrot => eprintln!("{base}"),
			};
		};

		match scan_code {
			Scancode::Escape => return true,
			Scancode::C      => self.do_draw = true,
			Scancode::X      => self.do_dump = true,
			Scancode::Z      => print_values(self.fractal, &self.center_real, &self.center_imaginary, &self.zoom, self.maximum_iteration_count, &self.julia_real, &self.julia_imaginary),
			_                => {},
		}

		match scan_code {
			Scancode::E => self.zoom *= 4.0,
			Scancode::Q => self.zoom /= 4.0,
			_           => {},
		};

		let translate_ammount = {
			let mut ammount = Float::with_val(PRECISION, 1.0);
			ammount /= 4.0;
			ammount /= &self.zoom;

			ammount
		};

		match scan_code {
			Scancode::A => self.center_real -= &translate_ammount,
			Scancode::D => self.center_real += &translate_ammount,
			_           => {},
		};

		match scan_code {
			Scancode::S => self.center_imaginary += &translate_ammount,
			Scancode::W => self.center_imaginary -= &translate_ammount,
			_           => {},
		};

		self.maximum_iteration_count = match scan_code {
			Scancode::F => self.maximum_iteration_count * 0x2,
			Scancode::R => self.maximum_iteration_count / 0x2,
			_           => self.maximum_iteration_count,
		};

		match scan_code {
			Scancode::Down  => self.julia_real += &translate_ammount,
			Scancode::Up    => self.julia_real -= &translate_ammount,
			_               => {},
		};

		match scan_code {
			Scancode::Left  => self.julia_imaginary -= &translate_ammount,
			Scancode::Right => self.julia_imaginary += &translate_ammount,
			_               => {},
		};

		return false;
	}
}
