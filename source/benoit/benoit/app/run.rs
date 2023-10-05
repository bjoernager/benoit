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

use crate::benoit::VERSION;
use crate::benoit::app::App;

extern crate sdl2;

impl App {
	#[must_use]
	pub fn run(&mut self) -> i32 {
		println!();
		println!("\u{1B}[1mBENO\u{CE}T\u{1B}[0m {:X}.{:X}.{:X}", VERSION.major, VERSION.minor, VERSION.patch);
		println!("Copyright 2021, 2023 Gabriel Bj\u{F8}rnager Jensen.");
		println!();
		println!("COCITAVIT\u{B7}ERCO\u{B7}FVIT");
		println!();

		return match self.interactive {
			true  => self.interactive(),
			false => match self.frame_count {
				0x1 => self.still(),
				_   => self.animate(),
			},
		};
	}
}
