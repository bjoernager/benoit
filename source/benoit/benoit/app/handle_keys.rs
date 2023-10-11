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
use crate::benoit::app::App;
use crate::benoit::configuration::Configuration;
use crate::benoit::renderer::Renderer;

extern crate rug;
extern crate sdl2;

use rug::{Assign, Float};
use sdl2::keyboard::{KeyboardState, Scancode};

pub const MIN_COLOUR_RANGE: f32 = 2.0;

impl App {
	#[must_use]
	pub(super) fn handle_keys(&mut self, scan_code: Scancode, state: KeyboardState) -> bool {
		if scan_code == Scancode::C { self.do_render = true };

		if state.is_scancode_pressed(Scancode::LShift) { return self.handle_shift_keys(scan_code) };

		match scan_code {
			Scancode::Escape => return true,
			Scancode::F1     => self.do_textual_feedback = !self.do_textual_feedback,
			Scancode::LAlt   => self.cycle_fractal(-0x1),
			Scancode::LCtrl  => self.toggle_inverse(),
			Scancode::Left   => self.cycle_palette(-0x1),
			Scancode::RAlt   => self.cycle_fractal(0x1),
			Scancode::Right  => self.cycle_palette(0x1),
			Scancode::Tab    => self.toggle_julia(),
			Scancode::X      => self.reset_viewport(),
			Scancode::Z      => self.dump_info(),
			_                => {},
		};

		self.translate(scan_code);

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

	#[must_use]
	fn handle_shift_keys(&mut self, scan_code: Scancode) -> bool {
		let translate_ammount = Float::with_val(PRECISION, 4.0 / 64.0 / &self.zoom);

		match scan_code {
			Scancode::A => self.extra.real -= &translate_ammount,
			Scancode::D => self.extra.real += &translate_ammount,
			_           => {},
		};

		match scan_code {
			Scancode::S => self.extra.imag -= &translate_ammount,
			Scancode::W => self.extra.imag += &translate_ammount,
			_           => {},
		};

		return false;
	}

	fn translate(&mut self, scan_code: Scancode) {
		const ZOOM_FACTOR: f32 = 1.0 + 1.0 / 4.0;

		match scan_code {
			Scancode::E => self.zoom *= ZOOM_FACTOR,
			Scancode::Q => self.zoom /= ZOOM_FACTOR,
			_           => {},
		};

		let translate_ammount = Float::with_val(PRECISION, 4.0 / 16.0 / &self.zoom);

		match scan_code {
			Scancode::A => self.centre.real -= &translate_ammount,
			Scancode::D => self.centre.real += &translate_ammount,
			_           => {},
		};

		match scan_code {
			Scancode::S => self.centre.imag -= &translate_ammount,
			Scancode::W => self.centre.imag += &translate_ammount,
			_           => {},
		};
	}

	fn cycle_fractal(&mut self, distance: i8) {
		self.fractal.cycle(distance);

		eprintln!("renderer the {}", self.fractal.kind().name());
	}

	fn toggle_julia(&mut self) {
		self.renderer.toggle();

		match self.renderer {
			Renderer::Julia  => eprintln!("enabled the julia set"),
			Renderer::Normal => eprintln!("disabled the julia set"),
		};
	}

	fn toggle_inverse(&mut self) {
		let inverse = !self.fractal.inverse();

		match inverse {
			false => eprintln!("reverting fractal"),
			true  => eprintln!("inverting fractals"),
		};

		self.fractal.set_inverse(inverse);
	}

	fn cycle_palette(&mut self, direction: i8) {
		let palette = self.palette.cycle(direction);

		eprintln!("using palette \"{}\"", palette.name());

		self.palette = palette;
	}

	fn reset_viewport(&mut self) {
		self.centre.real.assign(Configuration::DEFAULT_CENTRE_REAL);
		self.centre.imag.assign(Configuration::DEFAULT_CENTRE_IMAG);
		self.zoom.assign(       Configuration::DEFAULT_ZOOM);

		self.extra.real.assign(Configuration::DEFAULT_EXTRA_REAL);
		self.extra.imag.assign(Configuration::DEFAULT_EXTRA_IMAG);

		self.max_iter_count = Configuration::DEFAULT_MAX_ITER_COUNT;

		self.colour_range = Configuration::DEFAULT_COLOUR_RANGE;
	}

	fn dump_info(&self) {
		eprintln!("info dump: the {}", self.fractal.kind().name());
		eprintln!("  c = {}{:+}i ({}x)", &self.centre.real, &self.centre.imag, &self.zoom);
		eprintln!("  w = {}{:+}i", &self.extra.real, &self.extra.imag);
		eprintln!("  max. iter.: {}, col. range: {}", self.max_iter_count, self.colour_range);
	}
}
