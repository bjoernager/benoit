# include <benoit.hh>
# include <boost/multiprecision/mpfr.hpp>
# include <cstdint>
# include <fmt/core.h>
# include <iostream>
# include <pthread.h>
# include <string>
# include <vector>
using namespace std::literals::string_literals;
namespace {
	void * plotarea(void * rowsptr) {
		unsigned short rows = *(unsigned short *)(rowsptr);
		std::cout << fmt::format("Hello there from thread!\u000AI will calculate {} rows.\u000A"s,rows);
		return nullptr;
	}
}
std::vector<std::uint8_t> benoit::plotmandelbrot() {
	std::string const funcname = "benoit::plotmandelbrot()"s;
	this->notiffunc(funcname);
	std::vector<std::uint8_t> img;
	std::vector<::pthread_t>  threads;
	for(unsigned short thread = 0x0;(thread < this->numthreads);++thread) {
		threads.emplace_back();
		unsigned short rowspthread = (this->resy / this->numthreads);
		::pthread_create(&threads[thread],nullptr,::plotarea,&rowspthread);
	}
	for(auto thread : threads) {
		::pthread_join(thread,nullptr);
	}
	return img;
}
