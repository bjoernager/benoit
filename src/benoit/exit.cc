# include <benoit/d/debug.hh>
# include <benoit/d/dobt.hh>
# include <benoit/d/printdolog.hh>
# include <benoit/d/thelog.hh>
# include <benoit/exit.hh>
# include <benoit/logfunc.hh>
# include <benoit/print.hh>
# include <cstdlib>
# include <fmt/core.h>
# include <string>
# include <unistd.h>
using namespace std::literals::string_literals;
[[noreturn]] void benoit::exit(int code,std::string msg) noexcept {
	std::string const funcname = "benoit::exit(int,std::string)"s;
	benoit::logfunc(funcname);
	if(code == EXIT_FAILURE) {
		benoit::print(fmt::format("Exited with code {}: “{}”."s,code,msg),true);
	}
	if(((code == EXIT_FAILURE) && !benoit::d::debug) || benoit::d::dobt) {
		benoit::d::printdolog = false;
		benoit::print(""s);
		benoit::print("+-------------------"s);
		benoit::print("| :Backtrace Begin:"s);
		benoit::print("+-"s);
		benoit::print(""s);
		for(auto entry : benoit::d::thelog) {
			{
				::timespec sleepfor;
				sleepfor.tv_sec  = 0x0;
				sleepfor.tv_nsec = (0x3B9ACA00l / 0x6L);
				::nanosleep(&sleepfor,nullptr);
			}
			benoit::print(entry);
		}
		benoit::print(""s);
		benoit::print("+-"s);
		benoit::print("| :Backtrace End:"s);
		benoit::print("+-----------------"s);
		benoit::print(""s);
		benoit::d::printdolog = true;
	}
	::_exit(code);
}
