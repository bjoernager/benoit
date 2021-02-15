# include <benoit.hh>
# include <cstdint>
# include <cstdlib>
# include <fcntl.h>
# include <string>
# include <unistd.h>
# include <vector>
# include <webp/encode.h>
# include <webp/types.h>
using namespace std::literals::string_literals;
benoit::benoit(int const argc,char const * * argv) {
	/*
	let mut x0 = 0x0;
	let mut y0 = 0x0;
	let     maxiter = 0x100;
	let mut pos     = 0x1;
	while pos <= (resx * resy) {
		let mut x    = 0x0;
		let mut y    = 0x0;
		let mut iter = 0x1;
		while x * x + y * y <= 0x4 && iter <= maxiter {
			let xtmp = x * x - y * y + x0;
			y = 2 * x * y + y0;
			x = xtmp;
			iter += 0x1;
		}
		if iter == maxiter {
			buf.push(0xFF);
		}
		else {
			buf.push(0x0);
		}
		buf.push(0x0);
		buf.push(0xFF);
		pos += 0x1;
	}
	*/
	/*
	auto webpconf = ::WebPConfig();
	webpconf.lossless = 0x1;
	webpconf.quality  = 0x64;
	webpconf.method   = 0x6;
	webpconf.segments  = 0x4;
	webpconf.sns_strength  = 0x4;
	webpconf.filter_strength  = 0x0;
	webpconf.filter_sharpness = 0x0;
	webpconf.autofilter = 0x0;
	webpconf.alpha_compression = 0x1;
	webpconf.alpha_filtering = 0x2;
	webpconf.alpha_quality = 0x64;
	webpconf.pass = 0xA;
	webpconf.preprocessing = 0x1;
	webpconf.partitions = 0x3;
	webpconf.partitions = 0x0;
	webpconf.use_sharp_yuv = 0x0;
	auto conferr = ::WebPValidateConfig(&webpconf);
	auto webpimg = ::WebPPicture();
	WebPPictureInit(&webpimg);
	webpimg.use_argb = 0x1;
	webpimg.resx = 0x1;
	webpimg.resy = 0x1;
	WebPPictureAlloc(&webpimg);
	auto webpmemwrit = WebPMemoryWriter();
	WebPMemoryWriterInit(&webpmemwrit);
	webpimg.writer = WebPMemoryWrite;
	webpimg.custom_ptr = &webpmemwrit;
	webpimg.argb = {0xFF,0xFF,0x0,0x0};
	if(WebPEncode(&webpconf,&webpimg) == 0x0) {
	}
	else {

	}
	WebPPictureFree(&webpimg);
	*/
	//auto buf  = std::vector<std::uint8_t>();
	//buf.push_back(0xFF);
	//buf.push_back(0x0);
	//buf.push_back(0x0);
	//auto file = std::fstream("image.webp",std::fstream::binary | std::fstream::out | std::fstream::trunc);
	//if(!file.is_open()) {
	//	::_exit(EXIT_FAILURE);
	//}
	auto buf = std::vector<std::uint8_t>();
	const double maxR = 2.25;
	const double minR = -2.25;
	const double maxI = 2.25;
	const double minI = -2.25;

	for(unsigned int x = 0; x < this->resx; ++x)
		for(unsigned int y = 0; y < this->resy; ++y) {
			long double r  = x * ((maxR + this->real * this->zoom) / this->zoom - (minR + this->real * this->zoom) / this->zoom) / this->resx + (minR + this->real * this->zoom) / this->zoom;
			long double i  = y * ((maxI + this->imag * this->zoom) / this->zoom - (minI + this->imag * this->zoom) / this->zoom) / this->resx + (minI + this->imag * this->zoom) / this->zoom;
			long double r2 = 0.0;
			long double i2 = 0.0;

			unsigned int iter;
			for(iter = 0; iter < this->maxiter; ++iter) {
				if(r2 * r2 + i2 * i2 >= 4.0)
					break;

				long double r2Temp = r2 * r2 - i2 * i2 + r;
				long double i2Temp = 2.0 * r2 * i2 + i;

				if(r2 == r2Temp && i2 == i2Temp) {
					iter = this->maxiter;
					break;
				}

				r2 = r2Temp;
				i2 = i2Temp;
			}

			unsigned short int red   = 0;
			unsigned short int green = 0;
			unsigned short int blue  = 0;

			if(iter != this->maxiter) {
				double t = (double)(iter) / (double)(this->maxiter);
				red      = (int)(9.0 * (1.0 - t) * t * t * t * 255.0);
				green    = (int)(15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0);
				blue     = (int)(8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0);
			}
			buf.push_back(red);
			buf.push_back(green);
			buf.push_back(blue);
		}
	auto file = ::open("image.webp",O_WRONLY);
	//std::uint8_t * data;
	//auto outimglen = WebPEncodeLosslessRGB(buf.data(),this->resx,this->resy,0x8,&data);
	//file.write(reinterpret_cast<char *>(data),outimglen);
	std::string msg = ("P3 "s + std::to_string(this->resx) + " "s +std::to_string(this->resy) + " 255 "s);
	for(auto val : buf) {
		msg.append(std::to_string(val));
		msg.append(" ");
	}
	if(::write(file,msg.c_str(),this->strlen(msg.c_str())) < 0x0) {
		this->print("Unable to write to \"image.webp\".");
	}
	if(::close(file) < 0x0) {
		this->print("Unable to close file \"image.webp\".");
	}
	//file.close();
	::_exit(EXIT_SUCCESS);
}
