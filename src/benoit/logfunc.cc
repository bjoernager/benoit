# include <benoit/log.hh>
# include <benoit/logfunc.hh>
# include <fmt/core.h>
# include <string>
using namespace std::literals::string_literals;
void benoit::logfunc(std::string const func) {
	benoit::log(fmt::format("Function call: {}."s,func));
}
