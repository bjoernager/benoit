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

use sdl2::event::Event;

impl Application {
	pub fn run(&mut self) {
		let mut event_pump = self.sdl.event_pump().expect("unable to get event pump");

		self.render();

		'main_loop: loop {
			for event in event_pump.poll_iter() {
				match event {
					Event::Quit {..} => break 'main_loop,
					_                => {},
				}
			}
		}
	}
}
