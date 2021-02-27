# include <benoit/d/alpha.hh>
# include <benoit/d/maxiter.hh>
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
		const long double maxR = 0x2p0;
		const long double minR = -0x2p0;
		const long double maxI = 0x2p0;
		const long double minI = -0x2p0;
		for(unsigned long long px = *thrddat->imgbegin;(px <= *thrddat->imgend);++px) {
			unsigned x = (px % benoit::d::resx);
			unsigned y = ((px - (px % benoit::d::resy)) / benoit::d::resy);
			long double r  = x * ((maxR + benoit::d::pos.x * benoit::d::pos.zoom) / benoit::d::pos.zoom - (minR + benoit::d::pos.x * benoit::d::pos.zoom) / benoit::d::pos.zoom) / benoit::d::resx + (minR + benoit::d::pos.x * benoit::d::pos.zoom) / benoit::d::pos.zoom;
			long double i  = y * ((maxI + benoit::d::pos.y * benoit::d::pos.zoom) / benoit::d::pos.zoom - (minI + benoit::d::pos.y * benoit::d::pos.zoom) / benoit::d::pos.zoom) / benoit::d::resx + (minI + benoit::d::pos.y * benoit::d::pos.zoom) / benoit::d::pos.zoom;
			long double r2 = 0x0p0;
			long double i2 = 0x0p0;
			unsigned iter;
			for(iter = 0x0u; iter < benoit::d::maxiter; ++iter) {
				if(r2 * r2 + i2 * i2 >= 0x4p0) {
					break;
				}
				long double r2Temp = r2 * r2 - i2 * i2 + r;
				long double i2Temp = 0x2p0 * r2 * i2 + i;
				if(r2 == r2Temp && i2 == i2Temp) {
					iter = benoit::d::maxiter;
					break;
				}
				r2 = r2Temp;
				i2 = i2Temp;
			}
			{
				float tmp = ((float)(iter) / (float)(benoit::d::maxiter));
				thrddat->img[px].r = static_cast<std::uint8_t>((0x1p0 - tmp) * 0xFFp0);
				thrddat->img[px].g = static_cast<std::uint8_t>((0x1p0 - tmp) * 0xFFp0);
				thrddat->img[px].b = static_cast<std::uint8_t>((0x1p0 - tmp) * 0xFFp0);
				thrddat->img[px].a = static_cast<std::uint8_t>(tmp * 0xFFp0);
			}
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
		thrds[thrdid].thrd    =  ::new pthread_t;
		benoit::log(fmt::format("Thread #{} will render a total of {} pixels, starting at position {} and ending at position {}, which means there are {} pixels remaining."s,thrdid,(*thrds[thrdid].imgend - *thrds[thrdid].imgbegin + 0x1),*thrds[thrdid].imgbegin,*thrds[thrdid].imgend,remainpx));
		::pthread_create(thrds[thrdid].thrd,nullptr,::plotarea,&thrds[thrdid]);
	}
	for(auto thrd : thrds) {
		::pthread_join(*thrd.thrd,nullptr);
		benoit::log(fmt::format("Thread #{} has joined."s,*thrd.id));
		::delete thrd.id;
		::delete thrd.imgbegin;
		::delete thrd.imgend;
		::delete thrd.thrd;
	}
	std::vector<std::uint8_t> * img = ::new std::vector<std::uint8_t>;
	for(unsigned long long pos = 0x0ull;(pos < numpx);++pos) {
		img->push_back(rgbaimg[pos].r);
		img->push_back(rgbaimg[pos].g);
		img->push_back(rgbaimg[pos].b);
		if(benoit::d::alpha) {
			img->push_back(rgbaimg[pos].a);
		}
		else {
			img->push_back(0xFF);
		}
	}
	::delete rgbaimg;
	benoit::logfuncret(funcname);
	return img;
}
