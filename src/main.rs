use libwebp::WebPEncodeLosslessRGBA;
use std::fs::File;
use std::io::prelude::*;
fn main() -> std::io::Result<()> {
	let buf: &[u8] = &[
		255,255,255,255,
		255,0,0,255,
		0,255,0,255,
		0,0,255,255,
		255,255,255,127,
	];
	let data = WebPEncodeLosslessRGBA(buf,5,1,8).unwrap();
	let mut file = File::create("image.webp")?;
	let mut pos = 0;
	while pos < data.len() {
		let bytes_written = file.write(&data[pos..])?;
		pos += bytes_written;
	}
	Ok(())
}
