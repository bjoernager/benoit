# include <benoit/log.hh>
# include <benoit/logfunc.hh>
# include <fmt/core.h>
# include <string>
using namespace std::literals::string_literals;
void benoit::logfunc(std::string const func) {
	std::string const funcname = "benoit::logfunc(std::string const)"s;
	benoit::log(fmt::format("\33[33mFunction call\33[0m: {}."s,func));
}
