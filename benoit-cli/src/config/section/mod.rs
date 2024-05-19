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

use crate::config::Field;
use crate::error::Error;

use std::borrow::Cow;
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;
use toml::Table;

/// Denotes a configuration section.
pub struct Section<'a> {
	pub(in super) name:  Option<String>,
	pub(in super) table: Cow<'a, Table>,
}

impl<'a> Section<'a> {
	/// Creates a new root section.
	///
	/// This is done by parsing the configuration file at `path`.
	///
	/// # Errors
	///
	/// Returns an error if unable to read and parse the configuration file.
	pub fn create_root<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
		let path = path.as_ref();

		let config = read_to_string(path)
			.map_err(|e| Error::ConfigReadFailure { path: path.to_owned(), source: e })?;

		let table = Table::from_str(&config)
			.map_err(|e| Error::ConfigParseFailure { path: path.to_owned(), source: e })?;

		Ok(Self {
			name:  None,
			table: Cow::Owned(table)
		})
	}

	/// Searches the section's table for children.
	///
	/// The returned child will reference its parent section.
	///
	/// The child isn't guaranteed to exist, and in the event that it does, is typeless.
	/// See the [`Field`] type for more information.
	#[must_use]
	pub fn get_child(&'a self, name: &str) -> Field<'a> {
		let value = self.table.get(name);

		let name = self.name
			.as_ref()
			.map_or_else(
				|| name.to_owned(),

				|parent| format!("{parent}.{name}"),
			);

		Field {
			name,
			value,
		}
	}
}
