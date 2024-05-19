/*
	Copyright 2021, 2023-2024 Gabriel Bjørnager Jen-
	sen.

	This file is part of benoit-cli.

	benoit-cli is free software: you can redistrib-
	ute it and/or modify it under the terms of the
	GNU General Public License as published by the
	Free Software Foundation, either version 3 of
	the License, or (at your option) any later ver-
	sion.

	benoit-cli is distributed in the hope that it
	will be useful, but WITHOUT ANY WARRANTY; with-
	out even the implied warranty of MERCHANTABILITY
	or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
	General Public License for more details.

	You should have received a copy of the GNU Gene-
	ral Public License along with benoit-cli. If
	not, see <https://www.gnu.org/licenses/>.
*/

use crate::config::Config;

use benoit::VERSION;
use std::process::exit;

impl Config {
	/// Prints help to `stdout`.
	///
	/// This function marks the end of execution and does therefore not return.
	pub fn print_help() -> ! {
		println!("Benoit {}.{}.{}", VERSION.0, VERSION.1, VERSION.2);
		println!();
		println!("Usage:");
		println!("    benoit <path>");
		println!();
		println!("Wherein <path> denotes the path to the configuration file.");
		println!();

		exit(0x0);
	}
}
