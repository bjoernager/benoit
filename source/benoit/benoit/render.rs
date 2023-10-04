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
use std::sync::Arc;

pub mod colour_data;
pub mod factorise;
pub mod iterate;
pub mod paint;
pub mod render_data;
pub mod render_row;

pub type IteratorFunction = fn(&mut Float, &mut Float, &Float, &Float);

pub type RowRenderer = fn(Arc<RenderData>, u32, IteratorFunction);

// We pass the multibrot exponent to the factoriser
// as it is important with regard to smoothing, as
// the distance grows according to this exponent.
pub type FactoriserFunction = fn(u32, f32, f32, f32) -> f32;

pub type PaletteFunction = fn(f32) -> (f32, f32, f32);
