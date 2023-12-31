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

use crate::benoit::configuration::Configuration;

pub mod parse_arguments;
pub mod print_help;
pub mod print_message;
pub mod run;
pub mod set_title;
pub mod setup;

pub enum Mode {
	App(   u32, u32),
	Script(Configuration),
}

pub struct Launcher {}

impl Launcher {
	#[must_use]
	pub fn new() -> Launcher {
		return Launcher {};
	}
}
