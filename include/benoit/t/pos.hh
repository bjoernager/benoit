# pragma once
# include <boost/multiprecision/mpfr.hpp>
namespace benoit {
	namespace t {
		class pos {
		public:
											  pos(boost::multiprecision::mpfr_float x = 0x0,boost::multiprecision::mpfr_float y = 0x0);
			boost::multiprecision::mpfr_float x    = 0x0;
			boost::multiprecision::mpfr_float y    = 0x0;
			boost::multiprecision::mpfr_float zoom = 0x0;
		};
	}
}
