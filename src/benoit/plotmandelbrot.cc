# include <benoit/dat.hh>
# include <benoit/log.hh>
# include <benoit/logfunc.hh>
# include <benoit/logfuncret.hh>
# include <benoit/plotmandelbrot.hh>
# include <benoit/print.hh>
# include <benoit/t/thrddat.hh>
# include <boost/multiprecision/mpfr.hpp>
# include <cstdint>
# include <fmt/core.h>
# include <pthread.h>
# include <string>
# include <vector>
using namespace std::literals::string_literals;
namespace {
	void * plotarea(void * thrddatptr) {
		benoit::t::thrddat thrddat = *reinterpret_cast<benoit::t::thrddat *>(thrddatptr);
		thrddat.img = new std::vector<std::uint8_t>;
		pthread_exit(nullptr);
	}
}
std::vector<std::uint8_t> benoit::plotmandelbrot() {
	std::string const funcname = "benoit::plotmandelbrot()"s;
	benoit::logfunc(funcname);
	std::vector<std::uint8_t> img;
	unsigned short remainrows = benoit::dat.resy;
	std::vector<benoit::t::thrddat> thrds;
	for(unsigned thrdid = 0x0;(thrdid < benoit::dat.numthrds);++thrdid) {
		thrds.emplace_back();
		thrds[thrdid].id = new unsigned(thrdid);
		if((thrdid + 0x1) == benoit::dat.numthrds) {
			thrds[thrdid].rows = new unsigned(remainrows);
		}
		else {
			thrds[thrdid].rows = new unsigned(benoit::dat.resy / benoit::dat.numthrds);
		}
		remainrows -= *thrds[thrdid].rows;
		thrds[thrdid].thrd = new pthread_t;
		::pthread_create(thrds[thrdid].thrd,nullptr,::plotarea,&thrds[thrdid]);
		benoit::log(fmt::format("Thread #{} has been created."s,thrdid));
	}
	for(auto thrd : thrds) {
		::pthread_join(*thrd.thrd,nullptr);
		benoit::log(fmt::format("Thread #{} has joined."s,*thrd.id));
		delete thrd.id;
		delete thrd.img;
		delete thrd.rows;
		delete thrd.thrd;
	}
	benoit::logfuncret(funcname);
	return img;
}
