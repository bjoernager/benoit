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

use crate::benoit::render::RowRenderer;
use crate::benoit::render::render_row;

use std::mem::transmute;

#[derive(Clone, Copy)]
pub enum Rendering {
	Julia,
	Normal,
}

impl Rendering {
	pub fn get_row_renderer(self) -> RowRenderer {
		return match self {
			Rendering::Julia  => render_row::julia,
			Rendering::Normal => render_row::normal,
		};
	}

	pub fn cycle(self) -> Self {
		let raw = !(self as u8) & 0b00000001;

		let new: Self = unsafe { transmute(raw) };

		return new;
	}
}
