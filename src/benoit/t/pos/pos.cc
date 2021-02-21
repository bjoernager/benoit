# include <benoit/t/pos.hh>
# include <boost/multiprecision/mpfr.hpp>
benoit::t::pos::pos(boost::multiprecision::mpfr_float x,boost::multiprecision::mpfr_float y) {
	this->x = x;
	this->y = y;
}
