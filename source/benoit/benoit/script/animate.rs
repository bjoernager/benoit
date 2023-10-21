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

use crate::benoit::PRECISION;
use crate::benoit::image::Image;
use crate::benoit::render::Render;
use crate::benoit::script::Script;

extern crate rug;

use rug::Float;
use rug::ops::PowAssign;

impl Script {
	#[must_use]
	pub(super) fn animate(&self) -> Result<(), String> {
		// TO-DO: Proper animation for centre value when zooming.

		let frame_count = self.stop.frame - self.start.frame + 0x1;
		assert!(frame_count >= 0x2);

		let mut render = Render::allocate(self.canvas_width, self.canvas_height)?;
		let mut image  = Image::allocate( self.canvas_width, self.canvas_height)?;

		// Variables:
		let mut centre         = self.start.centre.clone();
		let mut extra          = self.start.extra.clone();
		let mut zoom           = self.start.zoom.clone();
		let mut max_iter_count = self.start.max_iter_count as f32;
		let mut colour_range   = self.start.colour_range;

		// Steps & factors:
		let centre_real_step    = get_step_bigfloat(self.stop.frame, &self.start.centre.real, &self.stop.centre.real);
		let centre_imag_step    = get_step_bigfloat(self.stop.frame, &self.start.centre.imag, &self.stop.centre.imag);
		let extra_real_step     = get_step_bigfloat(self.stop.frame, &self.start.extra.real, &self.stop.extra.real);
		let extra_imag_step     = get_step_bigfloat(self.stop.frame, &self.start.extra.imag, &self.stop.extra.imag);
		let zoom_factor         = get_factor_bigfloat(self.stop.frame, &self.start.zoom, &self.stop.zoom);
		let max_iter_count_step = get_step(self.stop.frame, self.start.max_iter_count as f32, self.stop.max_iter_count as f32);
		let colour_range_step   = get_step(self.stop.frame, self.start.colour_range, self.stop.colour_range);

		eprintln!("");
		eprintln!("animating {frame_count} frames: the {}", self.fractal.name());
		eprintln!("    re(c):           {} \u{2192} {} (step: {centre_real_step})",    self.start.centre.real,    self.stop.centre.real);
		eprintln!("    im(c):           {} \u{2192} {} (step: {centre_imag_step})",    self.start.centre.imag,    self.stop.centre.imag);
		eprintln!("    re(w):           {} \u{2192} {} (step: {extra_real_step})",     self.start.extra.real,     self.stop.extra.real);
		eprintln!("    im(w):           {} \u{2192} {} (step: {extra_imag_step})",     self.start.extra.imag,     self.stop.extra.imag);
		eprintln!("    zoom:            {} \u{2192} {} (fac.: {zoom_factor})",         self.start.zoom,           self.stop.zoom);
		eprintln!("    max. iter count: {} \u{2192} {} (step: {max_iter_count_step})", self.start.max_iter_count, self.stop.max_iter_count);
		eprintln!("    col. range:      {} \u{2192} {} (step: {colour_range_step})",   self.start.colour_range,   self.stop.colour_range);
		eprintln!("");

		for frame in 0x0..frame_count {
			let frame_name = format!("frame{frame:010}");

			Script::dump_frame(
				self.dump_path.as_str(),
				frame_name.as_str(),
				&mut image,
				&mut render,
				self.fractal,
				self.palette,
				&centre,
				&extra,
				&zoom,
				max_iter_count.round() as u32,
				colour_range,
				self.image_format,
			)?;

			centre.real    += &centre_real_step;
			centre.imag    += &centre_imag_step;
			extra.real     += &extra_real_step;
			extra.imag     += &extra_imag_step;
			zoom           *= &zoom_factor;
			max_iter_count += max_iter_count_step;
			colour_range   += colour_range_step;
		}

		return Ok(());
	}
}

#[must_use]
fn get_step(stop_x: u32, start_y: f32, stop_y: f32) -> f32 {
	assert!(stop_x - START_X != 0x0);

	const START_X: u32 = 0x1;

	// a = (y1-y0)/(x1-x0)
	return (stop_y - start_y) / (stop_x as f32 - START_X as f32);
}

#[allow(dead_code)]
#[must_use]
fn get_factor(stop_x: u32, start_y: f32, stop_y: f32) -> f32 {
	assert!(stop_x - START_X != 0x0);

	const START_X: u32 = 0x1;

	// a = (y1/y0)^(1/(x1-x0))
	return (stop_y / start_y).powf(1.0 / (stop_x as f32 - START_X as f32));
}

#[must_use]
fn get_step_bigfloat(stop_x: u32, start_y: &Float, stop_y: &Float) -> Float {
	assert!(stop_x - START_X > 0x0);

	const START_X: u32 = 0x1;

	let numerator   = Float::with_val(PRECISION, stop_y - start_y);
	let denominator = stop_x - START_X;

	return numerator / denominator;
}

#[must_use]
fn get_factor_bigfloat(stop_x: u32, start_y: &Float, stop_y: &Float) -> Float {
	assert!(stop_x - START_X > 0x0);
	if stop_y == start_y { return Float::with_val(PRECISION, 0x1) };

	const START_X: u32 = 0x1;

	// To get the zoom factor, we first want the 'a'
	// value of the growth function from (0) to
	// (frame_count) on the x-dimension and from
	// (zoom_start) to (stop_zoom) on the y-dimension:
	//
	// a = nroot(x1-x0,y1/y0),
	//
	// but this may be simplified for use with Rug
	// because
	//
	// nroot(a,b) = b^(1/a),
	//
	// making the final equation
	//
	// (y1/y0)^(1/(x1-x0)) = (stop_zoom/zoom_start)^(1/(frame_count-1)).

	let exponent = 1.0 / (stop_x as f64 - START_X as f64);

	let mut factor = Float::with_val(PRECISION, stop_y / start_y);
	factor.pow_assign(exponent);

	return factor;
}
