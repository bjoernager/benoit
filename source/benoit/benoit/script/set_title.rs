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

use crate::benoit::script::Script;

#[cfg(windows)]
extern crate windows;

#[cfg(windows)]
use windows::Win32::System::Console::SetConsoleTitleA;

impl Script {
	pub(super) fn set_title(title: &str) {
		#[cfg(unix)]
		{ eprint!("\u{1B}]0;{title}\u{07}") };

		#[cfg(windows)]
		unsafe { SetConsoleTitleA(title) };
	}
}
