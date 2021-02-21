# include <benoit/crtcfg.hh>
# include <benoit/dat.hh>
# include <benoit/loadcfg.hh>
# include <iostream>
# include <pugixml.hpp>
# include <unistd.h>
void benoit::loadcfg() {
	if(::access(benoit::dat.cfg.c_str(),R_OK) < 0x0) {
		benoit::crtcfg();
		return;
	}
}
