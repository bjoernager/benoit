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
use crate::benoit::launcher::{Launcher, Mode};

use std::env::args;
use std::str::FromStr;

impl Launcher {
	#[must_use]
	pub(super) fn parse_arguments(&self) -> Result<Mode, String> {
		let arguments = args();

		let mut width:         Option<u32>           = None;
		let mut height:        Option<u32>           = None;
		let mut configuration: Option<Configuration> = None;

		let mut command: Option<String> = None;

		for argument in arguments.skip(0x1) {
			match command {
				Some(_) => {
					match command.take().unwrap().as_str() {
						"height" => height        = Some(u32::from_str(argument.as_str()).unwrap()),
						"width"  => width         = Some(u32::from_str(argument.as_str()).unwrap()),
						"path"   => configuration = Some(Configuration::load(argument.as_str())?),
						_        => {},
					}
					continue;
				},
				_ => {},
			}

			// Check if command doesn't take a value.
			match argument.as_str() {
				"help" => Launcher::print_help(),
				_      => command = Some(argument.clone()),
			};
		}

		if command.is_some() { return Err(format!("missing value to command \"{}\"", command.unwrap())) };

		return if configuration.is_some() {
			if width.is_some() || height.is_some() { eprintln!("width and height will be overwritten in script mode") };

			Ok(Mode::Script(configuration.unwrap()))
		} else {
			// If only one length is specified, the other is
			// assumed equal.

			let width_val = if let Some(val) = width  { val }
			else            if let Some(val) = height { val }
			else                                      { 0x100 };

			let height_val = if let Some(val) = height { val }
			else             if let Some(val) = width  { val }
			else                                       { 0xC0 };

			Ok(Mode::App(width_val, height_val))
		};
	}
}
