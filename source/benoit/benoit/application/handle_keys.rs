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

use crate::benoit::application::Application;

extern crate sdl2;

use sdl2::keyboard::Scancode;

impl Application {
	pub fn handle_keys(&mut self, scan_code: Scancode) -> bool {
		match scan_code {
			Scancode::Escape => return true,
			Scancode::X      => self.dump(self.dump_path.clone()),
			_                => {},
		}

		self.zoom = match scan_code {
			Scancode::E => self.zoom * 2.0,
			Scancode::Q => self.zoom / 2.0,
			_           => self.zoom,
		};

		let translate_ammount: f64 = 1.0 / 16.0 / self.zoom;

		self.position_x += match scan_code {
			Scancode::A => -translate_ammount,
			Scancode::D => translate_ammount,
			_           => 0.0,
		};

		self.position_y += match scan_code {
			Scancode::S => translate_ammount,
			Scancode::W => -translate_ammount,
			_           => 0.0,
		};

		self.maximum_iteration_count = match scan_code {
			Scancode::F => self.maximum_iteration_count * 0x2,
			Scancode::R => self.maximum_iteration_count / 0x2,
			_           => self.maximum_iteration_count,
		};

		self.do_draw = match scan_code {
			Scancode::A => true,
			Scancode::D => true,
			Scancode::E => true,
			Scancode::F => true,
			Scancode::Q => true,
			Scancode::R => true,
			Scancode::S => true,
			Scancode::W => true,
			_           => false,
		};

		return false;
	}
}
