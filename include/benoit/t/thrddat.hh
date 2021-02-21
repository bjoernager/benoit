# pragma once
# include <cstdint>
# include <pthread.h>
# include <vector>
namespace benoit {
	namespace t {
		class thrddat {
		public:
			pthread_t *                 thrd = nullptr;
			std::vector<std::uint8_t> * img  = nullptr;
			unsigned *                  rows = nullptr;
			unsigned *                  id   = nullptr;
		};
	}
}
