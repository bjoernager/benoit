# pragma once
# include <benoit/t/rgba.hh>
# include <cstdint>
# include <pthread.h>
namespace benoit {
	namespace t {
		class thrddat {
		public:
			pthread_t *          thrd     = nullptr;
			std::uint8_t *       thrdcol  = nullptr;
			benoit::t::rgba *    img      = nullptr;
			unsigned *           id       = nullptr;
			unsigned long long * imgbegin = nullptr;
			unsigned long long * imgend   = nullptr;
		};
	}
}
