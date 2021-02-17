# include <benoit.hh>
# include <cstddef>
# include <cstdint>
# include <cstdlib>
# include <fcntl.h>
# include <fmt/core.h>
# include <string>
# include <unistd.h>
# include <vector>
# include <webp/encode.h>
# include <webp/types.h>
using namespace std::literals::string_literals;
[[noreturn]] benoit::benoit(int const argc,char const * * argv) noexcept {
	std::string const funcname = "benoit::benoit(int const,char const * *)"s;
	this->notiffunc(funcname);
	this->arghandl(argc,argv);
	switch(this->imgfmt) {
		case benoit::t::imgfmt::png:
			this->outimg.append(".png"s);
			break;
		case benoit::t::imgfmt::ppm:
			this->outimg.append(".ppm"s);
			break;
		case benoit::t::imgfmt::webp:
			if((this->resx > 0x1000) || (this->resy > 0x1000)) {
				this->exit(EXIT_FAILURE,"WebP does not support a resolution of more than 4096"s);
			}
			this->outimg.append(".webp"s);
			break;
	}
	std::vector<std::uint8_t> buf = this->plotmandelbrot();
	this->exit(EXIT_SUCCESS);
	int file = ::open(this->outimg.c_str(),O_TRUNC | O_WRONLY);
	std::string msg = ("P3 "s + std::to_string(this->resx) + " "s +std::to_string(this->resy) + " 255 "s);
	for(auto val : buf) {
		msg.append(std::to_string(val));
		msg.append(" ");
	}
	if(::write(file,msg.c_str(),std::strlen(msg.c_str())) < 0x0) {
		this->print(fmt::format("Unable to write to \"{}\"."s,this->outimg));
	}
	if(::close(file) < 0x0) {
		this->print(fmt::format("Unable to close file \"{]\"."s,this->outimg));
	}
	this->exit(EXIT_SUCCESS);
}
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
	auto buf  = std::vector<std::uint8_t>();
	buf.push_back(0xFF);
	buf.push_back(0x0);
	buf.push_back(0x0);
	auto file = std::fstream(this->outimt,std::fstream::binary | std::fstream::out | std::fstream::trunc);
	if(!file.is_open()) {
		::_exit(EXIT_FAILURE);
	}
	const double maxR = 2.25;
	const double minR = -2.25;
	const double maxI = 2.25;
	const double minI = -2.25;

	for(unsigned int y = 0; y < this->resy; ++y)
		for(unsigned int x = 0; x < this->resx; ++x) {
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

			std::uint8_t blue  = 0x0;
			std::uint8_t green = 0x0;
			std::uint8_t red   = 0x0;

			if(iter != this->maxiter) {
				float tmp = ((float)(iter) / (float)(this->maxiter));
				blue      = (std::uint8_t)((1.0 - tmp) * 255.0);
				green     = (std::uint8_t)((1.0 - tmp) * 255.0);
				red       = (std::uint8_t)((1.0 - tmp) * 255.0);
				//double t = (double)(iter) / (double)(this->maxiter);
				//red      = (int)(8.0 * (1.0 - t) * t * t * t * 255.0);
				//green    = (int)(16.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0);
				//blue     = (int)(8.0 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0);
			}
			buf.push_back(blue);
			buf.push_back(green);
			buf.push_back(red);
		}
*/
