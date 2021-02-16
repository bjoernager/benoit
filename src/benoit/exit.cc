# include <benoit.hh>
# include <fmt/core.h>
# include <string>
# include <unistd.h>
using namespace std::literals::string_literals;
[[noreturn]] void benoit::exit(int code,std::string msg) noexcept {
	std::string const funcname = "benoit::exit(int,std::string)"s;
	this->notiffunc(funcname);
	if(msg != ""s) {
		this->print(fmt::format("Exited with error code {}: {}",code,msg),true);
	}
	::_exit(code);
}
