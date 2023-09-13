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

use crate::benoit::PRECISION;
use crate::benoit::application::{Application, RowRenderer};
use crate::benoit::fractal::Fractal;
use crate::benoit::iteration::IteratorFunction;

extern crate rug;
extern crate sdl2;

use rug::Float;
use sdl2::keyboard::Scancode;

fn cycle_fractal(fractal: Fractal) -> (Fractal, IteratorFunction) {
	let fractal = match fractal {
		Fractal::BurningShip => Fractal::Mandelbrot,
		Fractal::Mandelbrot  => Fractal::Tricorn,
		Fractal::Tricorn     => Fractal::BurningShip,
	};

	let iterator_function = Application::get_iterator_function(fractal);

	eprintln!("rendering the {}", fractal.get_name());

	return (fractal, iterator_function);
}

fn toggle_julia(julia: bool) -> (bool, RowRenderer) {
	let julia = !julia;

	let row_renderer = Application::get_row_renderer(julia);

	match julia {
		false => eprintln!("disabled the julia set"),
		true  => eprintln!("enabled the julia set"),
	};

	return (julia, row_renderer);
}

impl Application {
	pub fn handle_keys(&mut self, scan_code: Scancode) -> bool {
		match scan_code {
			Scancode::LAlt   => (self.fractal, self.iterator_function) = cycle_fractal(self.fractal),
			Scancode::Escape => return true,
			Scancode::C      => self.do_render = true,
			Scancode::Tab    => (self.julia, self.row_renderer) = toggle_julia(self.julia),
			Scancode::X      => self.do_dump = true,
			Scancode::Z      => eprintln!("c = {}{:+}i -- {}x @ {} iter.", &self.centre_real, &self.centre_imag, &self.zoom, self.max_iter_count),
			_                => {},
		}

		match scan_code {
			Scancode::E => self.zoom *= 2.0,
			Scancode::Q => self.zoom /= 2.0,
			_           => {},
		};

		let translate_ammount = {
			let mut ammount = Float::with_val(PRECISION, 4.0);
			ammount /= 16.0;
			ammount /= &self.zoom;

			ammount
		};

		match scan_code {
			Scancode::A => self.centre_real -= &translate_ammount,
			Scancode::D => self.centre_real += &translate_ammount,
			_           => {},
		};

		match scan_code {
			Scancode::S => self.centre_imag += &translate_ammount,
			Scancode::W => self.centre_imag -= &translate_ammount,
			_           => {},
		};

		self.max_iter_count = match scan_code {
			Scancode::F => self.max_iter_count * 0x2,
			Scancode::R => self.max_iter_count / 0x2,
			_           => self.max_iter_count,
		};

		return false;
	}
}
