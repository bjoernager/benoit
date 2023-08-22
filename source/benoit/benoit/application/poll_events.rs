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

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;

impl Application {
	pub fn poll_events(&mut self, event_pump: &mut EventPump) -> bool {
		for event in event_pump.poll_iter() {
			match event {
				Event::KeyDown {
					timestamp: _,
					window_id: _,
					keycode:   _,
					scancode,
					keymod:    _,
					repeat:    _,
				} => {
					let scancode = scancode.unwrap();

					match scancode {
						Scancode::Escape => return true,
						Scancode::X      => self.dump_image(),
						_                => {},
					}

					self.zoom = match scancode {
						Scancode::E => self.zoom * 2.0,
						Scancode::Q => self.zoom / 2.0,
						_           => self.zoom,
					};

					let translate_ammount: f64 = 1.0 / 16.0 / self.zoom;

					self.position_x += match scancode {
						Scancode::A => -translate_ammount,
						Scancode::D => translate_ammount,
						_           => 0.0,
					};

					self.position_y += match scancode {
						Scancode::S => translate_ammount,
						Scancode::W => -translate_ammount,
						_           => 0.0,
					};

					self.maximum_iteration_count = match scancode {
						Scancode::F => self.maximum_iteration_count * 0x2,
						Scancode::R => self.maximum_iteration_count / 0x2,
						_           => self.maximum_iteration_count,
					};

					self.do_draw = match scancode {
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
				},
				Event::Quit { .. } => return true,
				_ => {},
			}
		}

		return false;
	}
}
