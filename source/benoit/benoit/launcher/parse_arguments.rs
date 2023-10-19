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
use crate::benoit::launcher::Launcher;

use std::env::args;

impl Launcher {
	#[must_use]
	pub(super) fn parse_arguments(&self) -> (Configuration, bool) {
		let mut arguments = args();

		return match arguments.nth(0x1) {
			Some(argument) => {
				if argument == "--help" { Launcher::print_help() };

				let configuration = match Configuration::load(argument.as_str()) {
					Ok( configuration) => configuration,
					Err(message)       => panic!("error parsing configuration: {message}"),
				};

				(configuration, false)
			},
			_ => (Configuration::default(), true),
		};
	}
}
