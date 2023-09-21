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
use crate::benoit::app::{App, RowRenderer};
use crate::benoit::fractal::Fractal;
use crate::benoit::iteration::IteratorFunction;

extern crate rug;
extern crate sdl2;

use rug::Float;
use sdl2::keyboard::Scancode;

impl App {
	pub fn handle_keys(&mut self, scan_code: Scancode) -> bool {
		match scan_code {
			Scancode::LAlt   => (self.fractal, self.iterator_function) = cycle_fractal(self.fractal),
			Scancode::Escape => return true,
			Scancode::C      => self.do_render = true,
			Scancode::Tab    => (self.julia, self.row_renderer) = toggle_julia(self.julia),
			Scancode::X      => self.do_dump = true,
			Scancode::Z      => eprintln!("c = {}{:+}i -- {}x @ {} iter. (range: {:.3})", &self.centre_real, &self.centre_imag, &self.zoom, self.max_iter_count, self.colour_range),
			_                => {},
		}

		self.handle_translation(scan_code);

		self.max_iter_count = match scan_code {
			Scancode::F => self.max_iter_count * 0x2,
			Scancode::R => self.max_iter_count / 0x2,
			_           => self.max_iter_count,
		};

		const COLOUR_RANGE_FACTOR: f32 = 1.0 + 1.0 / 16.0;

		self.colour_range = match scan_code {
			Scancode::Up   => self.colour_range * COLOUR_RANGE_FACTOR,
			Scancode::Down => self.colour_range / COLOUR_RANGE_FACTOR,
			_              => self.colour_range,
		};

		return false;
	}

	fn handle_translation(&mut self, scan_code: Scancode) {
		const ZOOM_FACTOR: f32 = 1.0 + 1.0 / 4.0;

		match scan_code {
			Scancode::E => self.zoom *= ZOOM_FACTOR,
			Scancode::Q => self.zoom /= ZOOM_FACTOR,
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
			Scancode::S => self.centre_imag -= &translate_ammount,
			Scancode::W => self.centre_imag += &translate_ammount,
			_           => {},
		};
	}
}

fn cycle_fractal(fractal: Fractal) -> (Fractal, IteratorFunction) {
	let fractal = match fractal {
		Fractal::BurningShip => Fractal::Mandelbrot,
		Fractal::Mandelbrot  => Fractal::Tricorn,
		Fractal::Tricorn     => Fractal::BurningShip,
	};

	let iterator_function = App::get_iterator_function(fractal);

	eprintln!("rendering the {}", fractal.get_name());

	return (fractal, iterator_function);
}

fn toggle_julia(julia: bool) -> (bool, RowRenderer) {
	let julia = !julia;

	let row_renderer = App::get_row_renderer(julia);

	match julia {
		false => eprintln!("disabled the julia set"),
		true  => eprintln!("enabled the julia set"),
	};

	return (julia, row_renderer);
}
