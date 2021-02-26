# include <benoit/crtcfg.hh>
# include <benoit/logfunc.hh>
# include <benoit/logfuncret.hh>
# include <string>
using namespace std::literals::string_literals;
void benoit::crtcfg() {
	std::string const funcname = "benoit::crtcfg()"s;
	benoit::logfunc(funcname);
	benoit::logfuncret(funcname);
}
