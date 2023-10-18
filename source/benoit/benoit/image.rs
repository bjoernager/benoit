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

use std::ops::{Index, IndexMut};
use std::slice::from_raw_parts;

pub mod allocate;
pub mod colour;
pub mod dump;

pub struct Image {
	width:  u32,
	height: u32,

	data: Vec::<(u8, u8, u8)>,
}

#[derive(Clone, Copy)]
pub enum ImageFormat {
	Png,
	Webp,
}

impl Image {
	#[must_use]
	pub fn size(&self) -> (u32, u32) {
		return (self.width, self.height);
	}

	#[must_use]
	pub fn data<'a>(&'a self) -> &'a [(u8, u8, u8)] {
		return &self.data[..];
	}

	#[must_use]
	pub fn raw<'a>(&'a self) -> &'a [u8] {
		let data_pointer = self.data.as_ptr() as *const u8;

		let length = self.height as usize * self.width as usize * 0x3;
		let slice  = unsafe { from_raw_parts(data_pointer, length) };

		return slice;
	}
}

impl Index<usize> for Image {
	type Output = (u8, u8, u8);

	fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
		return &self.data[index];
	}
}

impl IndexMut<usize> for Image {
	fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Self::Output {
		return &mut self.data[index];
	}
}
