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

use crate::benoit::render::render_data::RenderData;

extern crate rug;

use rug::Float;

pub mod colour;
pub mod colour_data;
pub mod iterate;
pub mod render;
pub mod render_data;
pub mod render_point;

pub use colour::*;
pub use render::*;

pub type IteratorFunction = fn(&mut Float, &mut Float, &Float, &Float);

pub type PointRenderer = fn(&RenderData, u32, u32, IteratorFunction) -> (u32, f32);
