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

use crate::benoit::{PRECISION};
use crate::benoit::app::App;
use crate::benoit::fractal::Fractal;
use crate::benoit::palette::Palette;
use crate::benoit::render::{IteratorFunction, RowRenderer};
use crate::benoit::rendering::Rendering;

extern crate rug;
extern crate sdl2;

use rug::Float;
use sdl2::keyboard::Scancode;

pub const MIN_COLOUR_RANGE: f32 = 2.0;

impl App {
	#[must_use]
	pub fn handle_keys(&mut self, scan_code: Scancode) -> bool {
		match scan_code {
			Scancode::C      => self.do_render = true,
			Scancode::Escape => return true,
			Scancode::F1     => self.do_textual_feedback = !self.do_textual_feedback,
			Scancode::LAlt   => (self.fractal, self.multibrot_exponent, self.iterator_function) = cycle_fractal(self.fractal, -0x1),
			Scancode::Left   => self.palette = cycle_palette(self.palette, -0x1),
			Scancode::RAlt   => (self.fractal, self.multibrot_exponent, self.iterator_function) = cycle_fractal(self.fractal, 0x1),
			Scancode::Right  => self.palette = cycle_palette(self.palette, 0x1),
			Scancode::Tab    => (self.rendering, self.row_renderer) = toggle_julia(self.rendering),
			Scancode::Z      => eprintln!("c = {}{:+}i, {}x @ {} iter. (range.: {:.3})", &self.centre_real, &self.centre_imag, &self.zoom, self.max_iter_count, self.colour_range),
			_                => {},
		}

		self.handle_translation(scan_code);

		self.max_iter_count = match scan_code {
			Scancode::F => self.max_iter_count * 0x2,
			Scancode::R => (self.max_iter_count / 0x2).max(0x1),
			_           => self.max_iter_count,
		};

		const COLOUR_RANGE_FACTOR: f32 = 1.0 + 1.0 / 16.0;

		self.colour_range = match scan_code {
			Scancode::Up   => self.colour_range * COLOUR_RANGE_FACTOR,
			Scancode::Down => (self.colour_range / COLOUR_RANGE_FACTOR).max(MIN_COLOUR_RANGE),
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

fn cycle_fractal(fractal: Fractal, distance: i8) -> (Fractal, f32, IteratorFunction) {
	let fractal  = fractal + distance;
	let exponent = fractal.get_exponent();

	let iterator_function = fractal.get_iterator();

	eprintln!("rendering the {}", fractal.get_name());

	return (fractal, exponent, iterator_function);
}

fn toggle_julia(rendering: Rendering) -> (Rendering, RowRenderer) {
	let rendering = rendering.cycle();

	let row_renderer = rendering.get_row_renderer();

	match rendering {
		Rendering::Julia  => eprintln!("enabled the julia set"),
		Rendering::Normal => eprintln!("disabled the julia set"),
	};

	return (rendering, row_renderer);
}

fn cycle_palette(palette: Palette, direction: i8) -> Palette {
	let palette = palette.cycle(direction);

	eprintln!("using palette \"{}\"", palette.get_name());

	return palette;
}
