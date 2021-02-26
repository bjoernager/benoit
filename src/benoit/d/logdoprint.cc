# include <benoit/d/logdoprint.hh>
bool benoit::d::logdoprint =
# if defined(NDEBUG)
	false;
# else
	true;
# endif
