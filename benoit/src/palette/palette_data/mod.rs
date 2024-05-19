/*
	Copyright 2021, 2023-2024 Gabriel Bjørnager Jen-
	sen.

	This file is part of benoit.

	benoit is free software: you can redistribute it
	and/or modify it under the terms of the GNU Af-
	fero General Public License as published by the
	Free Software Foundation, either version 3 of
	the License, or (at your option) any later ver-
	sion.

	benoit is distributed in the hope that it will
	be useful, but WITHOUT ANY WARRANTY; without
	even the implied warranty of MERCHANTABILITY or
	FITNESS FOR A PARTICULAR PURPOSE. See the GNU
	Affero General Public License for more details.

	You should have received a copy of the GNU Af-
	fero General Public License along with benoit.
	If not, see <https://www.gnu.org/licenses/>.
*/

use crate::error::Error;
use crate::render::ImageElement;

use std::ops::{Deref, DerefMut, Index, IndexMut};

/// The information of the palette.
///
/// Whilst anyone could create their own [`PaletteData`](crate::palette::PaletteData) object, static data may be retrieved using [`Palette::data`](crate::palette::Palette::data).
#[derive(Clone)]
pub struct PaletteData {
	buffer: Box<[(ImageElement, ImageElement)]>,
}

impl PaletteData {
	/// Allocates new palette data.
	///
	/// # Errors
	///
	/// If the length is zero, a [`ZeroLengthPaletteData`](Error::ZeroLengthPaletteData) instance is returned.
	pub fn new(len: usize) -> Result<Self, Error> {
		if len == 0x0 { return Err(Error::ZeroLengthPaletteData) };

		Ok(Self { buffer: vec![Default::default(); len].into_boxed_slice() })
	}

	#[must_use]
	fn factor_to_index(&self, factor: f64) -> usize {
		let index = factor * self.len() as f64;

		(index.trunc() as usize).clamp(0x0, self.len() - 0x1)
	}
}

impl Deref for PaletteData {
	type Target = [(ImageElement, ImageElement)];

	#[inline(always)]
	fn deref(&self) -> &Self::Target { &self.buffer }
}

impl DerefMut for PaletteData {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.buffer }
}

impl Index<f64> for PaletteData {
	type Output = (ImageElement, ImageElement);

	fn index(&self, index: f64) -> &Self::Output {
		let index = self.factor_to_index(index);
		self.get(index).unwrap()
	}
}

impl IndexMut<f64> for PaletteData {
	fn index_mut(&mut self, index: f64) -> &mut Self::Output {
		let index = self.factor_to_index(index);
		self.get_mut(index).unwrap()
	}
}
