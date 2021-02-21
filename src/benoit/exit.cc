# include <benoit/dat.hh>
# include <benoit/exit.hh>
# include <benoit/logfunc.hh>
# include <benoit/print.hh>
# include <cstdlib>
# include <fmt/core.h>
# include <string>
# include <time.h>
using namespace std::literals::string_literals;
[[noreturn]] void benoit::exit(int code,std::string msg) noexcept {
	std::string const funcname = "benoit::exit(int,std::string)"s;
	benoit::logfunc(funcname);
	if(benoit::dat.debug) {
		benoit::print(fmt::format("Exited with code {}: \"{}\"."s,code,msg),true);
	}
	if((code == EXIT_FAILURE) || benoit::dat.dobt || benoit::dat.debug) {
		benoit::dat.printdolog = false;
		benoit::print(""s);
		benoit::print("+-------------"s);
		benoit::print("| :Backtrace:"s);
		benoit::print("+-"s);
		benoit::print(""s);
		for(auto entry : benoit::dat.thelog) {
			{
				::timespec sleepfor;
				sleepfor.tv_sec  = 0x0;
				sleepfor.tv_nsec = (0x3B9ACA00L / 0x3L);
				::nanosleep(&sleepfor,nullptr);
			}
			benoit::print(entry);
		}
	}
	::_exit(code);
}
