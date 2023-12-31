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

use crate::benoit::app::App;

extern crate sdl2;

use sdl2::EventPump;
use sdl2::event::Event;

impl App {
	#[must_use]
	pub(super) fn poll_events(&mut self, pump: &mut EventPump) -> bool {
		loop {
			let event = match pump.poll_event() {
				Some(event) => event,
				None        => break,
			};

			let quit = match event {
				Event::KeyDown {
					timestamp: _,
					window_id: _,
					keycode:   _,
					scancode:  scan_code,
					keymod:    _,
					repeat:    _,
				} => {
					let state = pump.keyboard_state();

					self.handle_keys(scan_code.unwrap(), state)
				},
				Event::Quit { .. } => true,
				_ => false,
			};

			if quit { return true };
		}

		return false;
	}
}
