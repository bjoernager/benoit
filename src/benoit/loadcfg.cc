# include <benoit/crtcfg.hh>
# include <benoit/d/cfg.hh>
# include <benoit/d/pos.hh>
# include <benoit/loadcfg.hh>
# include <benoit/log.hh>
# include <benoit/logfunc.hh>
# include <benoit/logfuncret.hh>
# include <benoit/print.hh>
# include <fcntl.h>
# include <fmt/core.h>
# include <iostream>
# include <string>
# include <sys/stat.h>
# include <unistd.h>
# include <iostream>
using namespace std::literals::string_literals;
void benoit::loadcfg() {
	std::string const funcname = "benoit::loadcfg()"s;
	benoit::logfunc(funcname);
	if(::access(benoit::d::cfg.c_str(),R_OK) < 0x0) {
		benoit::crtcfg();
		return;
	}
	int filsiz = 0x0;
	{
		struct ::stat st;
		::stat(benoit::d::cfg.c_str(),&st);
		filsiz = static_cast<int>(st.st_size);
	}
	benoit::log(fmt::format("The size of “{}” is {}."s,benoit::d::cfg,filsiz));
	if(filsiz == 0x0) {
		benoit::crtcfg();
		return;
	}
	std::string cfgstr = ""s;
	{
		int fil = ::open(benoit::d::cfg.c_str(),O_RDONLY);
		char * cfgcstr = ::new char[filsiz];
		for(int pos = 0x0;(pos < filsiz);++pos) {
			if(::read(fil,&cfgcstr[pos],0x1) < 0x0) {
			}
		}
		cfgstr = std::string(cfgcstr);
		delete[] cfgcstr;
		::close(fil);
	}
	benoit::logfuncret(funcname);
}
