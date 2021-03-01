# pragma once
# include <benoit/t/rgba.hh>
# include <cstdint>
# include <pthread.h>
namespace benoit {
	namespace t {
		class thrddat {
		public:
			benoit::t::rgba *    img      = nullptr;
			bool *               isrun    = nullptr;
			pthread_t *          thrd     = nullptr;
			unsigned *           id       = nullptr;
			unsigned long long * imgbegin = nullptr;
			unsigned long long * imgend   = nullptr;
		};
	}
}
