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
# include <cmath>
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
			unsigned x = (px % benoit::d::resx);
			unsigned y = ((px - (px % benoit::d::resy)) / benoit::d::resy);
			::__float128 r  = x * ((0x2p0 + benoit::d::pos.x * benoit::d::pos.zoom) / benoit::d::pos.zoom - (-0x2p0 + benoit::d::pos.x * benoit::d::pos.zoom) / benoit::d::pos.zoom) / benoit::d::resx + (-0x2p0 + benoit::d::pos.x * benoit::d::pos.zoom) / benoit::d::pos.zoom;
			::__float128 i  = y * ((0x2p0 + benoit::d::pos.y * benoit::d::pos.zoom) / benoit::d::pos.zoom - (-0x2p0 + benoit::d::pos.y * benoit::d::pos.zoom) / benoit::d::pos.zoom) / benoit::d::resx + (-0x2p0 + benoit::d::pos.y * benoit::d::pos.zoom) / benoit::d::pos.zoom;
			::__float128 r1 = 0x0p0;
			::__float128 i1 = 0x0p0;
			unsigned long long iter = 0x0ull;
			for(;(iter < benoit::d::maxiter);++iter) {
				if((r1 * r1 + i1 * i1) >= 0x4p0) {
					break;
				}
				::__float128 r1tmp = (r1 * r1 - i1 * i1 + r);
				::__float128 i1tmp = (0x2p0 * r1 * i1 + i);
				if((r1 == r1tmp) && (i1 == i1tmp)) {
					iter = benoit::d::maxiter;
					break;
				}
				r1 = r1tmp;
				i1 = i1tmp;
			}
			if(iter != benoit::d::maxiter) {
				float fac = (static_cast<float>(iter) / 0x40p0);
				fac       = (fac - std::floor(fac));
				if(fac <= (0x1p0 / 0x4p0)) {
					thrddat->img[px].r = static_cast<std::uint8_t>(fac * 0xFFp0 * 0x4p0);
					thrddat->img[px].g = static_cast<std::uint8_t>(0x0p0);
					thrddat->img[px].b = static_cast<std::uint8_t>(0x0p0);
				}
				else if(fac <= (0x1p0 / 0x2p0)) {
					thrddat->img[px].r = static_cast<std::uint8_t>(0xFFp0);
					thrddat->img[px].g = static_cast<std::uint8_t>(fac * 0xFFp0 * 0x4p0);
					thrddat->img[px].b = static_cast<std::uint8_t>(0x0p0);
				}
				else if(fac <= (0x3p0 / 0x4p0)) {
					thrddat->img[px].r = static_cast<std::uint8_t>(0xFFp0);
					thrddat->img[px].g = static_cast<std::uint8_t>(0xFFp0);
					thrddat->img[px].b = static_cast<std::uint8_t>(fac * 0xFFp0 * 0x4p0);
				}
				else {
					fac = (0x1p0 - fac);
					thrddat->img[px].r = static_cast<std::uint8_t>(fac * 0xFFp0 * 0x4p0);
					thrddat->img[px].g = static_cast<std::uint8_t>(fac * 0xFFp0 * 0x4p0);
					thrddat->img[px].b = static_cast<std::uint8_t>(fac * 0xFFp0 * 0x4p0);
				}
			}
			else {
				thrddat->img[px].a = 0x0u;
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
		benoit::log(fmt::format("Allocating data for thread #{}."s,thrdid));
		thrds[thrdid].id    = ::new unsigned(thrdid);
		thrds[thrdid].img   = rgbaimg;
		thrds[thrdid].isrun = ::new bool(false);
		thrds[thrdid].thrd  = ::new pthread_t;
	}
	for(unsigned thrdid = 0x0,y = 0x0u;(y < benoit::d::resy);++thrdid,++y) {
		if(remainpx == 0x0) {
			break;
		}
		if(thrdid == benoit::d::numthrds) {
			thrdid = 0x0;
		}
		if(*thrds[thrdid].isrun) {
			pthread_join(*thrds[thrdid].thrd,nullptr);
			delete thrds[thrdid].isrun;
			thrds[thrdid].isrun = ::new bool(false);
		}
		thrds[thrdid].imgbegin =  ::new unsigned long long(y * benoit::d::resx);
		thrds[thrdid].imgend   =  ::new unsigned long long(*thrds[thrdid].imgbegin + benoit::d::resx - 0x1);
		::pthread_create(thrds[thrdid].thrd,nullptr,::plotarea,&thrds[thrdid]);
		delete thrds[thrdid].isrun;
		thrds[thrdid].isrun = ::new bool(true);
	}
	for(auto thrd : thrds) {
		benoit::log(fmt::format("Destroying thread #{}'s data."s,*thrd.id));
		if(*thrd.isrun) {
			::pthread_join(*thrd.thrd,nullptr);
		}
		thrd.img = nullptr;
		::delete thrd.id;
		::delete thrd.imgbegin;
		::delete thrd.imgend;
		::delete thrd.isrun;
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
	::delete[] rgbaimg;
	benoit::logfuncret(funcname);
	return img;
}
