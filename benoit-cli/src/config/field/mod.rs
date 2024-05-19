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

use crate::error::Error;

use std::str::FromStr;
use toml::Value;

/// Denotes a configuration field.
///
/// By default, these fields are typeless.
/// They can, however, be "transformed" to a given type using the [`Take`](crate::config::Take) trait.
pub struct Field<'a> {
	pub(in super) name:  String,
	pub(in super) value: Option<&'a Value>,
}

impl<'a> Field<'a> {
	/// Borows the contained value.
	///
	/// The returned reference is still considered "typeless," as is defined by `toml`'s own [`Value`] type.
	///
	/// # Errors
	///
	/// Returns an error if the field could not be found.
	pub(in super) fn borrow_value(&mut self) -> Result<&'a Value, Error> {
		self.value
			.take()
			.ok_or_else(|| Error::MissingConfigField { name: self.name.clone() })
	}
}

impl Field<'_> {
	/// Transforms the field into the given type `T`.
	/// The result of [`FromStr`] implementation is passed on.
	///
	/// # Errors
	///
	/// Returns an error if the field doesn't exist, or is a different type, or if the [`FromStr`] implementation failed.
	pub fn take_from_str<T: FromStr>(mut self) -> Result<Result<T, <T as FromStr>::Err>, Error> {
		let Value::String(ref s) = *self.borrow_value()? else {
			return Err(Error::WrongFieldType { name: self.name, ok_type: "string" })
		};

		Ok(FromStr::from_str(s))
	}
}
