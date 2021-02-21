static_assert(((sizeof(short) >= 0x2) && (sizeof(int) >= 0x4) && (sizeof(long) >= 0x4) && (sizeof(long long) >= 0x8) && (sizeof(void *) >= 0x8)),"Benoit expects at least an LLP64 data model.");
# include <benoit/arghandl.hh>
# include <benoit/log.hh>
# include <benoit/logfunc.hh>
# include <benoit/dat.hh>
# include <benoit/exit.hh>
# include <benoit/loadcfg.hh>
# include <benoit/log.hh>
# include <benoit/logfunc.hh>
# include <benoit/logfuncret.hh>
# include <benoit/main.hh>
# include <benoit/plotmandelbrot.hh>
# include <benoit/print.hh>
# include <cstddef>
# include <cstdint>
# include <cstdlib>
# include <fcntl.h>
# include <fmt/core.h>
# include <string>
# include <sys/types.h>
# include <thread>
# include <unistd.h>
# include <vector>
# include <webp/encode.h>
# include <webp/types.h>
using namespace std::literals::string_literals;
[[noreturn]] void benoit::main(int const argc,char const * * argv) noexcept {
	std::string const funcname = "benoit::main(int const,char const * *)"s;
	benoit::log(fmt::format("{}"s,argv[0x0]));
	benoit::logfunc(funcname);
	benoit::dat.numthrds = std::thread::hardware_concurrency();
	benoit::log(fmt::format("{} availabe threads have been detected.",benoit::dat.numthrds));
	benoit::arghandl(argc,argv);
	benoit::loadcfg();
	std::vector<std::uint8_t> buf = benoit::plotmandelbrot();
	benoit::exit(EXIT_SUCCESS);
	std::string msg = ("P3 "s + std::to_string(benoit::dat.resx) + " "s +std::to_string(benoit::dat.resy) + " 255 "s);
	for(auto val : buf) {
		benoit::print(fmt::format("Found value {} in image."s,val));
		msg.append(std::to_string(val));
		msg.append(" ");
	}
	int file = ::open(benoit::dat.outimg.c_str(),O_WRONLY);
	//for(::ssize_t bytesleft = msg.size();(bytesleft > 0x0);) {
		if(::write(file,msg.c_str(),msg.size()) < 0x0) {
			benoit::print(fmt::format("Unable to write to “{}”."s,benoit::dat.outimg));
		}
	//}
	if(::close(file) < 0x0) {
		benoit::print(fmt::format("Unable to close file “{}”."s,benoit::dat.outimg));
	}
	benoit::exit(EXIT_SUCCESS);
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
	auto file = std::fstream(benoit::outimt,std::fstream::binary | std::fstream::out | std::fstream::trunc);
	if(!file.is_open()) {
		::_exit(EXIT_FAILURE);
	}
	const double maxR = 2.25;
	const double minR = -2.25;
	const double maxI = 2.25;
	const double minI = -2.25;

	for(unsigned int y = 0; y < benoit::dat.resy; ++y)
		for(unsigned int x = 0; x < benoit::dat.resx; ++x) {
			long double r  = x * ((maxR + benoit::real * benoit::dat.zoom) / benoit::dat.zoom - (minR + benoit::real * benoit::dat.zoom) / benoit::dat.zoom) / benoit::dat.resx + (minR + benoit::real * benoit::dat.zoom) / benoit::dat.zoom;
			long double i  = y * ((maxI + benoit::imag * benoit::dat.zoom) / benoit::dat.zoom - (minI + benoit::imag * benoit::dat.zoom) / benoit::dat.zoom) / benoit::dat.resx + (minI + benoit::imag * benoit::dat.zoom) / benoit::dat.zoom;
			long double r2 = 0.0;
			long double i2 = 0.0;

			unsigned int iter;
			for(iter = 0; iter < benoit::maxiter; ++iter) {
				if(r2 * r2 + i2 * i2 >= 4.0)
					break;

				long double r2Temp = r2 * r2 - i2 * i2 + r;
				long double i2Temp = 2.0 * r2 * i2 + i;

				if(r2 == r2Temp && i2 == i2Temp) {
					iter = benoit::maxiter;
					break;
				}

				r2 = r2Temp;
				i2 = i2Temp;
			}

			std::uint8_t blue  = 0x0;
			std::uint8_t green = 0x0;
			std::uint8_t red   = 0x0;

			if(iter != benoit::maxiter) {
				float tmp = ((float)(iter) / (float)(benoit::maxiter));
				blue      = (std::uint8_t)((1.0 - tmp) * 255.0);
				green     = (std::uint8_t)((1.0 - tmp) * 255.0);
				red       = (std::uint8_t)((1.0 - tmp) * 255.0);
				//double t = (double)(iter) / (double)(benoit::maxiter);
				//red      = (int)(8.0 * (1.0 - t) * t * t * t * 255.0);
				//green    = (int)(16.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0);
				//blue     = (int)(8.0 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0);
			}
			buf.push_back(blue);
			buf.push_back(green);
			buf.push_back(red);
		}
*/
