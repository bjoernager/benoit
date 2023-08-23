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

impl Application {
	pub fn poll_events(&mut self, event_pump: &mut EventPump) -> bool {
		for event in event_pump.poll_iter() {
			let quit = match event {
				Event::KeyDown {
					timestamp: _,
					window_id: _,
					keycode:   _,
					scancode:  scan_code,
					keymod:    _,
					repeat:    _,
				} => self.handle_key(scan_code.unwrap()),
				Event::Quit { .. } => true,
				_ => false,
			};

			if quit { return true }
		}

		return false;
	}
}
