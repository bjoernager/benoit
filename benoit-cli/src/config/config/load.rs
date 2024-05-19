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
use crate::error::Error;

use std::env::args;

impl Config {
	/// Loads the configuration file as specified on the command line.
	///
	/// # Errors
	///
	/// Returns an error depending on the command line arguments and validity of the configuration file.
	pub fn load() -> Result<Self, Error> {
		let mut args = args().skip(0x1);

		let Some(config_path) = args.next() else {
			return Err(Error::MissingConfigPath);
		};

		if args.next().is_some() { return Err(Error::TooManyCliArguments) };

		let config = Self::load_from(config_path)?;
		config.validate()?;

		Ok(config)
	}
}
