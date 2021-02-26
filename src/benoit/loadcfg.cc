# include <benoit/crtcfg.hh>
# include <benoit/d/cfg.hh>
# include <benoit/loadcfg.hh>
# include <iostream>
# include <pugixml.hpp>
# include <string>
# include <unistd.h>
using namespace std::literals::string_literals;
void benoit::loadcfg() {
	std::string const funcname = "benoit::loadcfg()"s;
	if(::access(benoit::d::cfg.c_str(),R_OK) < 0x0) {
		benoit::crtcfg();
		return;
	}
}
