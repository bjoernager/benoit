# include <benoit/crtcfg.hh>
# include <benoit/d/cfg.hh>
# include <benoit/err/clsfil.hh>
# include <benoit/err/wrttofil.hh>
# include <benoit/logfunc.hh>
# include <benoit/logfuncret.hh>
# include <benoit/print.hh>
# include <cstdint>
# include <fcntl.h>
# include <fmt/core.h>
# include <string>
# include <unistd.h>
using namespace std::literals::string_literals;
void benoit::crtcfg() {
	std::string const funcname = "benoit::crtcfg()"s;
	benoit::logfunc(funcname);
	std::string cfgstr = ""s;
	cfgstr.append("{\u000A"s);
	cfgstr.append("\u0009\u0022benoit\u0022:[\u000A"s);
	cfgstr.append("\u0009\u0009{\u000A"s);
	cfgstr.append("\u0009\u0009\u0009\u0022x\u0022:\u00221÷1\u0022,\u000A"s);
	cfgstr.append("\u0009\u0009\u0009\u0022y\u0022:\u00221÷1\u0022,\u000A"s);
	cfgstr.append("\u0009\u0009\u0009\u0022zoom\u0022:\u00220÷1\u0022\u000A"s);
	cfgstr.append("\u0009\u0009}\u000A"s);
	cfgstr.append("\u0009]\u000A"s);
	cfgstr.append("}\u000A"s);
	std::uint8_t * dat = reinterpret_cast<std::uint8_t *>(const_cast<char *>(cfgstr.c_str()));
	int fil = ::open(benoit::d::cfg.c_str(),(O_CREAT | O_TRUNC | O_WRONLY),0x1B4);
	for(unsigned long long pos = 0x0;(pos < cfgstr.size());++pos) {
		::ssize_t byteswrtn = ::write(fil,&dat[pos],0x1);
		if(byteswrtn < 0x0) {
			benoit::err::wrttofil(benoit::d::cfg);
			return;
		}
	}
	if(::close(fil) < 0x0) {
		benoit::err::clsfil(benoit::d::cfg);
	}
	benoit::logfuncret(funcname);
}
