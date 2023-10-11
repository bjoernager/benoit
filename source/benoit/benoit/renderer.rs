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

use crate::benoit::fractal::IteratorFunction;
use crate::benoit::render_data::RenderData;

use std::mem::transmute;

mod render_point;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Renderer {
	Julia,
	Normal,
}

pub type PointRenderer = fn(&RenderData, u32, u32, IteratorFunction) -> (u32, f32);

impl Renderer {
	#[must_use]
	pub fn point_renderer(self) -> PointRenderer {
		return match self {
			Renderer::Julia  => render_point::julia,
			Renderer::Normal => render_point::normal,
		};
	}

	pub fn toggle(&mut self) {
		let raw = !(*self as u8) & 0b00000001;
		let new: Self = unsafe { transmute(raw) };

		*self = new;
	}
}
