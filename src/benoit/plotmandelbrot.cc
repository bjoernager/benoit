# include <benoit/d/numthrds.hh>
# include <benoit/d/pos.hh>
# include <benoit/d/resx.hh>
# include <benoit/d/resy.hh>
# include <benoit/log.hh>
# include <benoit/logfunc.hh>
# include <benoit/logfuncret.hh>
# include <benoit/plotmandelbrot.hh>
# include <benoit/print.hh>
# include <benoit/t/thrddat.hh>
# include <benoit/t/rgba.hh>
# include <boost/multiprecision/mpfr.hpp>
# include <cassert>
# include <cstdint>
# include <fmt/core.h>
# include <pthread.h>
# include <string>
# include <vector>
using namespace std::literals::string_literals;
namespace {
	void * plotarea(void * thrddatptr) {
		benoit::t::thrddat * thrddat = reinterpret_cast<benoit::t::thrddat *>(thrddatptr);
		for(unsigned long long px = *thrddat->imgbegin;(px <= *thrddat->imgend);++px) {
			thrddat->img[px].r = *thrddat->thrdcol;
			thrddat->img[px].g = *thrddat->thrdcol;
			thrddat->img[px].b = *thrddat->thrdcol;
		}
		pthread_exit(nullptr);
	}
}
std::vector<std::uint8_t> * benoit::plotmandelbrot() {
	std::string const funcname = "benoit::plotmandelbrot()"s;
	benoit::logfunc(funcname);
	unsigned long long const numpx = (benoit::d::resx * benoit::d::resy);
	benoit::log(fmt::format("The image has a total of {} pixels."s,numpx));
	unsigned long long              remainpx = numpx;
	benoit::t::rgba *               rgbaimg  = ::new benoit::t::rgba[numpx];
	std::vector<benoit::t::thrddat> thrds(benoit::d::numthrds);
	for(unsigned thrdid = 0x0;(thrdid < benoit::d::numthrds);++thrdid) {
		thrds[thrdid].id  = ::new unsigned(thrdid);
		thrds[thrdid].img = rgbaimg;
		thrds[thrdid].imgbegin = ::new unsigned long long(numpx / benoit::d::numthrds * thrdid);
		if((thrdid + 0x1) == benoit::d::numthrds) {
			thrds[thrdid].imgend = ::new unsigned long long(*thrds[thrdid].imgbegin + remainpx);
		}
		else {
			thrds[thrdid].imgend = ::new unsigned long long(*thrds[thrdid].imgbegin + numpx / benoit::d::numthrds);
		}
		--*thrds[thrdid].imgend;
		assert(*thrds[thrdid].imgend <= numpx);
		remainpx              -= (*thrds[thrdid].imgend - *thrds[thrdid].imgbegin + 0x1);
		thrds[thrdid].thrdcol =  ::new std::uint8_t((float)(thrdid + 0x1) / (float)(benoit::d::numthrds) * 0xFFp0);
		thrds[thrdid].thrd    =  ::new pthread_t;
		benoit::log(fmt::format("Thread #{} will render a total of {} pixels, starting at position {} and ending at position {}, each with the colour {}, which means there are {} pixels remaining."s,thrdid,(*thrds[thrdid].imgend - *thrds[thrdid].imgbegin + 0x1),*thrds[thrdid].imgbegin,*thrds[thrdid].imgend,*thrds[thrdid].thrdcol,remainpx));
		::pthread_create(thrds[thrdid].thrd,nullptr,::plotarea,&thrds[thrdid]);
	}
	for(auto thrd : thrds) {
		::pthread_join(*thrd.thrd,nullptr);
		benoit::log(fmt::format("Thread #{}, which coloured it's pixels with {}, has joined."s,*thrd.id,*thrd.thrdcol));
		::delete thrd.id;
		::delete thrd.imgbegin;
		::delete thrd.imgend;
		::delete thrd.thrd;
		::delete thrd.thrdcol;
	}
	std::vector<std::uint8_t> * img = ::new std::vector<std::uint8_t>;
	for(unsigned long long pos = 0x0ull;(pos < numpx);++pos) {
		img->push_back(rgbaimg[pos].r);
		img->push_back(rgbaimg[pos].g);
		img->push_back(rgbaimg[pos].b);
	}
	::delete rgbaimg;
	benoit::logfuncret(funcname);
	return img;
}
/*
const double maxR = 2.25;
const double minR = -2.25;
const double maxI = 2.25;
const double minI = -2.25;

for(unsigned int y = 0; y < benoit::d::resy; ++y)
	for(unsigned int x = 0; x < benoit::d::resx; ++x) {
		long double r  = x * ((maxR + benoit::real * benoit::d::zoom) / benoit::d::zoom - (minR + benoit::real * benoit::d::zoom) / benoit::d::zoom) / benoit::d::resx + (minR + benoit::real * benoit::d::zoom) / benoit::d::zoom;
		long double i  = y * ((maxI + benoit::imag * benoit::d::zoom) / benoit::d::zoom - (minI + benoit::imag * benoit::d::zoom) / benoit::d::zoom) / benoit::d::resx + (minI + benoit::imag * benoit::d::zoom) / benoit::d::zoom;
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
