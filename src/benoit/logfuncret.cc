# include <benoit/log.hh>
# include <benoit/logfuncret.hh>
# include <fmt/core.h>
# include <string>
using namespace std::literals::string_literals;
void benoit::logfuncret(std::string const func) {
	std::string const funcname = "benoit::logfuncret(std::string const)"s;
	benoit::log(fmt::format("\33[35mFunction return\33[0m: {}."s,func));
}
