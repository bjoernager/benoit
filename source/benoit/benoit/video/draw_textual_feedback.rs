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

use crate::benoit::complex::Complex;
use crate::benoit::video::Video;

extern crate rug;
extern crate sdl2;

use rug::Float;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

impl Video {
	pub fn draw_textual_feedback(&mut self, centre: &Complex, zoom: &Float, max_iter_count: u32) {
		let real_text = format!("REAL: {:.18}",  centre.real.to_f64());
		let imag_text = format!("IMAG: {:.18}",  centre.imag.to_f64());
		let zoom_text = format!("ZOOM: 2^{:.9}", zoom.to_f64().log2());
		let iter_text = format!("ITER: {}",      max_iter_count);

		let string = format!("{real_text}\n{imag_text}\n{zoom_text}\n{iter_text}");

		let mut offset_x = 0x1;
		let mut offset_y = 0x1;
		for character in string.chars() {
			if character == '\n' {
				offset_x =  0x1;
				offset_y += TEXTURE_HEIGHT;
				continue;
			}

			let character = convert_character(character);

			draw_character(&mut self.canvas, offset_x, offset_y, character);

			offset_x += TEXTURE_WIDTH;
		}
	}
}

fn draw_character(canvas: &mut WindowCanvas, x: u32, y: u32, character: u8) {
	let texture = &FONT[character as usize];

	for pixel in 0x0..TEXTURE_SIZE {
		let alpha = texture[pixel] as u8 * 0xFF;

		let texture_y = pixel as u32 / TEXTURE_WIDTH;
		let texture_x = pixel as u32 - texture_y * TEXTURE_WIDTH;

		let square = Rect::new(
			(x * CHARACTER_SCALE + texture_x * CHARACTER_SCALE) as i32,
			(y * CHARACTER_SCALE + texture_y * CHARACTER_SCALE) as i32,
			CHARACTER_SCALE,
			CHARACTER_SCALE,
		);

		canvas.set_draw_color(Color::RGBA(0xFF, 0xFF, 0xFF, alpha));
		canvas.fill_rect(square).unwrap();
	}
}

fn convert_character(input: char) -> u8 {
	return match input {
		' ' => 0x00,
		'A' => 0x01,
		'E' => 0x02,
		'G' => 0x03,
		'I' => 0x04,
		'L' => 0x05,
		'M' => 0x06,
		'O' => 0x07,
		'R' => 0x08,
		'T' => 0x09,
		'Z' => 0x0A,
		'0' => 0x0B,
		'1' => 0x0C,
		'2' => 0x0D,
		'3' => 0x0E,
		'4' => 0x0F,
		'5' => 0x10,
		'6' => 0x11,
		'7' => 0x12,
		'8' => 0x13,
		'9' => 0x14,
		'-' => 0x15,
		'^' => 0x16,
		'.' => 0x17,
		':' => 0x18,
		_   => 0x19,
	};
}

const CHARACTER_SCALE: u32 = 0x2;

const TEXTURE_WIDTH:  u32   = 0x6;
const TEXTURE_HEIGHT: u32   = 0x6;
const TEXTURE_SIZE:   usize = TEXTURE_HEIGHT as usize * TEXTURE_WIDTH as usize;

const FONT: [[bool; TEXTURE_SIZE]; 0x1A] = [
	[
		false, false, false, false, false, false,
		false, false, false, false, false, false,
		false, false, false, false, false, false,
		false, false, false, false, false, false,
		false, false, false, false, false, false,
		false, false, false, false, false, false,
	],
	[
		false, true,  true,  true,  false, false,
		true,  true,  true,  true,  true,  false,
		true,  true,  false, true,  true,  false,
		true,  true,  true,  true,  true,  false,
		true,  true,  false, true,  true,  false,
		false, false, false, false, false, false,
	],
	[
		true,  true,  true,  true,  true,  false,
		true,  true,  false, false, false, false,
		true,  true,  true,  true,  false, false,
		true,  true,  false, false, false, false,
		true,  true,  true,  true,  true,  false,
		false, false, false, false, false, false,
	],
	[
		true,  true,  true,  true,  true,  false,
		true,  true,  false, false, false, false,
		true,  true,  false, true,  true,  false,
		true,  true,  false, false, true,  false,
		true,  true,  true,  true,  true,  false,
		false, false, false, false, false, false,
	],
	[
		true,  true,  true,  true,  true,  false,
		false, false, true,  false, false, false,
		false, false, true,  false, false, false,
		false, false, true,  false, false, false,
		true,  true,  true,  true,  true,  false,
		false, false, false, false, false, false,
	],
	[
		true,  true,  false, false, false, false,
		true,  true,  false, false, false, false,
		true,  true,  false, false, false, false,
		true,  true,  false, false, false, false,
		true,  true,  true,  true,  true,  false,
		false, false, false, false, false, false,
	],
	[
		true,  true,  false, true,  true,  false,
		true,  true,  true,  true,  true,  false,
		true,  true,  true,  true,  true,  false,
		true,  true,  false, true,  true,  false,
		true,  true,  false, true,  true,  false,
		false, false, false, false, false, false,
	],
	[
		false, true,  true,  true,  false, false,
		true,  true,  false, true,  true,  false,
		true,  true,  false, true,  true,  false,
		true,  true,  false, true,  true,  false,
		false, true,  true,  true,  false, false,
		false, false, false, false, false, false,
	],
	[
		true,  true,  true,  true,  false, false,
		true,  true,  false, true,  true,  false,
		true,  true,  true,  true,  false, false,
		true,  true,  false, true,  true,  false,
		true,  true,  false, false, true,  false,
		false, false, false, false, false, false,
	],
	[
		true,  true,  true,  true,  true,  false,
		false, false, true,  false, false, false,
		false, false, true,  false, false, false,
		false, false, true,  false, false, false,
		false, false, true,  false, false, false,
		false, false, false, false, false, false,
	],
	[
		true,  true,  true,  true,  true,  false,
		false, false, true,  true,  true,  false,
		false, true,  true,  true,  false, false,
		true,  true,  true,  false, false, false,
		true,  true,  true,  true,  true,  false,
		false, false, false, false, false, false,
	],
	[
		false, true,  true,  true,  false, false,
		true,  true,  false, true,  true,  false,
		true,  true,  false, true,  true,  false,
		true,  true,  false, true,  true,  false,
		false, true,  true,  true,  false, false,
		false, false, false, false, false, false,
	],
	[
		false, true,  true,  false, false, false,
		true,  true,  true,  false, false, false,
		false, false, true,  false, false, false,
		false, false, true,  false, false, false,
		true,  true,  true,  true,  true,  false,
		false, false, false, false, false, false,
	],
	[
		false, true,  true,  true,  false, false,
		true,  false, false, true,  true,  false,
		false, false, true,  true,  false, false,
		false, true,  true,  false, false, false,
		true,  true,  true,  true,  true,  false,
		false, false, false, false, false, false,
	],
	[
		true,  true,  true,  true,  true,   false,
		true,  false, false, true,  true,  false,
		false, false, true,  true,  false, false,
		false, false, false, true,  true,  false,
		true,  true,  true,  true,  false, false,
		false, false, false, false, false, false,
	],
	[
		false, true,  false, true,  true,  false,
		true,  true,  false, true,  true,  false,
		true,  true,  true,  true,  true,  false,
		false, false, false, true,  true,  false,
		false, false, false, true,  true,  false,
		false, false, false, false, false, false,
	],
	[
		true,  true,  true,  true,  true,  false,
		true,  true,  false, false, false, false,
		true,  true,  true,  true,  true,  false,
		false, false, false, true,  true,  false,
		true,  true,  true,  true,  false, false,
		false, false, false, false, false, false,
	],
	[
		false, true,  true,  true,  true,  false,
		true,  true,  false, false, false, false,
		true,  true,  true,  true,  true,  false,
		true,  true,  false, true,  true,  false,
		false, true,  true,  true,  true,  false,
		false, false, false, false, false, false,
	],
	[
		true,  true,  true,  true,  true,  false,
		false, false, false, true,  true,  false,
		false, false, true,  true,  false, false,
		false, true,  true,  false, false, false,
		true,  true,  false, false, false, false,
		false, false, false, false, false, false,
	],
	[
		false, true,  true,  true,  false, false,
		true,  true,  false, true,  true,  false,
		false, true,  true,  true,  false, false,
		true,  true,  false, true,  true,  false,
		false, true,  true,  true,  false, false,
		false, false, false, false, false, false,
	],
	[
		false, true,  true,  true,  true,  false,
		true,  true,  false, true,  true,  false,
		true,  true,  true,  true,  true,  false,
		false, false, false, true,  true,  false,
		true,  true,  true,  true,  false, false,
		false, false, false, false, false, false,
	],
	[
		false, false, false, false, false, false,
		false, false, false, false, false, false,
		true,  true,  true,  true,  true,  false,
		false, false, false, false, false, false,
		false, false, false, false, false, false,
		false, false, false, false, false, false,
	],
	[
		false, false, true,  false, false, false,
		false, true,  true,  true,  false, false,
		true,  true,  false, true,  true,  false,
		false, false, false, false, false, false,
		false, false, false, false, false, false,
		false, false, false, false, false, false,
	],
	[
		false, false, false, false, false, false,
		false, false, false, false, false, false,
		false, false, false, false, false, false,
		true,  false, false, false, false, false,
		true,  false, false, false, false, false,
		false, false, false, false, false, false,
	],
	[
		true,  false, false, false, false, false,
		false, false, false, false, false, false,
		false, false, false, false, false, false,
		false, false, false, false, false, false,
		true,  false, false, false, false, false,
		false, false, false, false, false, false,
	],
	[
		true,  true,  false, false, false, false,
		false, true,  false, false, false, false,
		true,  true,  false, false, false, false,
		false, false, false, false, false, false,
		true,  false, false, false, false, false,
		false, false, false, false, false, false,
	],
];
