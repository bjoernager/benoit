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

use crate::config::{Field, Section};
use crate::error::Error;

use benoit::PRECISION;
use benoit::complex::Complex;
use rug::Float;
use std::borrow::Cow;
use std::fs::canonicalize;
use std::path::PathBuf;
use std::str::FromStr;
use toml::Value;

/// Used for loading configuration fields.
///
/// More specifically, this is to be used with the untyped [`Field`] type, where using [`take`](Take::take) collapses the field into a definitively typed object.
pub trait Take<T> {
	/// Collapses the field into the given type `T`.
	///
	/// # Errors
	///
	/// Returns an error if the field doesn't exist or is a different type.

	fn take(self) -> Result<T, Error>;
}

fn test_field_domain<T, U>(name: &str, value: &T, min: U, max: U) -> Result<(), Error>
where
	T: PartialOrd + ToString,
	U: Copy + Into<T> + ToString, {
	if *value < min.into() {
		Err(Error::FieldLowerBounds {
			name:  name.to_owned(),
			value: value.to_string(),
			limit: min.to_string(),
		})
	} else if *value > max.into() {
		Err(Error::FieldUpperBounds {
			name:  name.to_owned(),
			value: value.to_string(),
			limit: max.to_string(),
		})
	} else {
		Ok(())
	}
}

impl Take<bool> for Field<'_> {
	fn take(mut self) -> Result<bool, Error> {
		let Value::Boolean(value) = *self.borrow_value()? else {
			return Err(Error::WrongFieldType { name: self.name, ok_type: "bool" });
		};

		Ok(value)
	}
}

impl Take<Complex> for Field<'_> {
	fn take(mut self) -> Result<Complex, Error> {
		let s = if let Value::String(ref s) = *self.borrow_value()? {
			Ok(s)
		} else {
			Err(Error::WrongFieldType { name: self.name, ok_type: "complex" })
		}?;

		FromStr::from_str(s).map_err(Into::into)
	}
}

impl Take<f32> for Field<'_> {
	fn take(mut self) -> Result<f32, Error> {
		if let Value::Float(value) = *self.borrow_value()? {
			// Is this even possible?
			if value.is_nan() {
				return Err(Error::NonNumberField { name: self.name });
			}

			if value.is_infinite() {
				return Err(Error::InfiniteField { name: self.name });
			}

			test_field_domain(&self.name, &value, -f32::MAX, f32::MAX)?;

			Ok(value as f32)
		} else {
			Err(Error::WrongFieldType { name: self.name, ok_type: "float" })
		}
	}
}

impl Take<f64> for Field<'_> {
	fn take(mut self) -> Result<f64, Error> {
		if let Value::Float(value) = *self.borrow_value()? {
			// Is this even possible?
			if value.is_nan() {
				return Err(Error::NonNumberField { name: self.name });
			}

			if value.is_infinite() {
				return Err(Error::InfiniteField { name: self.name });
			}

			Ok(value)
		} else {
			Err(Error::WrongFieldType { name: self.name, ok_type: "float" })
		}
	}
}

impl Take<Float> for Field<'_> {
	fn take(mut self) -> Result<Float, Error> {
		if let Value::String(ref value) = *self.borrow_value()? {
			if let Ok(value) = Float::parse(value) {
				return Ok(Float::with_val(PRECISION, value));
			}
		}

		Err(Error::WrongFieldType { name: self.name, ok_type: "bigfloat" })
	}
}

impl Take<i16> for Field<'_> {
	fn take(mut self) -> Result<i16, Error> {
		if let Value::Integer(value) = *self.borrow_value()? {
			test_field_domain(&self.name, &value, i16::MIN, i16::MAX)?;

			return Ok(TryFrom::try_from(value).unwrap());
		};

		Err(Error::WrongFieldType { name: self.name, ok_type: "integer" })
	}
}

impl Take<i32> for Field<'_> {
	fn take(mut self) -> Result<i32, Error> {
		if let Value::Integer(value) = *self.borrow_value()? {
			test_field_domain(&self.name, &value, i32::MIN, i32::MAX)?;

			return Ok(TryFrom::try_from(value).unwrap());
		};

		Err(Error::WrongFieldType { name: self.name, ok_type: "integer" })
	}
}

impl Take<i64> for Field<'_> {
	fn take(mut self) -> Result<i64, Error> {
		if let Value::Integer(value) = *self.borrow_value()? {
			return Ok(value);
		};

		Err(Error::WrongFieldType { name: self.name, ok_type: "integer" })
	}
}

impl Take<i8> for Field<'_> {
	fn take(mut self) -> Result<i8, Error> {
		if let Value::Integer(value) = *self.borrow_value()? {
			test_field_domain(&self.name, &value, i8::MIN, i8::MAX)?;

			return Ok(TryFrom::try_from(value).unwrap());
		};

		Err(Error::WrongFieldType { name: self.name, ok_type: "integer" })
	}
}

impl Take<PathBuf> for Field<'_> {
	fn take(self) -> Result<PathBuf, Error> {
		let s = <Self as Take<String>>::take(self)?;

		canonicalize(&s).map_err(|e| Error::InvalidPath { path: s, source: e })
	}
}

impl<'a> Take<Section<'a>> for Field<'a> {
	fn take(mut self) -> Result<Section<'a>, Error> {
		if let Value::Table(ref table) = *self.borrow_value()? {
			return Ok(Section {
				name:  Some(self.name),
				table: Cow::Borrowed(table),
			});
		}

		Err(Error::WrongFieldType { name: self.name, ok_type: "section" })
	}
}

impl Take<String> for Field<'_> {
	fn take(mut self) -> Result<String, Error> {
		if let Value::String(ref s) = *self.borrow_value()? {
			Ok(s.to_owned())
		} else {
			Err(Error::WrongFieldType { name: self.name, ok_type: "string" })
		}
	}
}

impl Take<u16> for Field<'_> {
	fn take(mut self) -> Result<u16, Error> {
		if let Value::Integer(value) = *self.borrow_value()? {
			if value.is_negative() {
				return Err(Error::NegativeField { name: self.name });
			}

			test_field_domain(&self.name, &value, u16::MIN, u16::MAX)?;

			return Ok(TryFrom::try_from(value).unwrap());
		};

		Err(Error::WrongFieldType { name: self.name, ok_type: "integer" })
	}
}

impl Take<u32> for Field<'_> {
	fn take(mut self) -> Result<u32, Error> {
		if let Value::Integer(value) = *self.borrow_value()? {
			if value.is_negative() {
				return Err(Error::NegativeField { name: self.name });
			}

			test_field_domain(&self.name, &value, u32::MIN, u32::MAX)?;

			return Ok(TryFrom::try_from(value).unwrap());
		};

		Err(Error::WrongFieldType { name: self.name, ok_type: "integer" })
	}
}

impl Take<u64> for Field<'_> {
	fn take(mut self) -> Result<u64, Error> {
		if let Value::Integer(value) = *self.borrow_value()? {
			if value.is_negative() {
				return Err(Error::NegativeField { name: self.name })
			}

			return Ok(TryFrom::try_from(value).unwrap());
		};

		Err(Error::WrongFieldType { name: self.name, ok_type: "integer" })
	}
}

impl Take<u8> for Field<'_> {
	fn take(mut self) -> Result<u8, Error> {
		if let Value::Integer(value) = *self.borrow_value()? {
			if value.is_negative() {
				return Err(Error::NegativeField { name: self.name });
			}

			test_field_domain(&self.name, &value, u8::MIN, u8::MAX)?;

			return Ok(TryFrom::try_from(value).unwrap());
		};

		Err(Error::WrongFieldType { name: self.name, ok_type: "integer" })
	}
}
