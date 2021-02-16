# include <benoit.hh>
# include <fmt/core.h>
# include <string>
void benoit::notiffunc(std::string const callfuncname) {
	std::string const funcname = "benoit::notiffunc(std:.string constexpr)"s;
	if constexpr(benoit::debug) {
		this->print(fmt::format("Function \"{}\" has been called."s,callfuncname));
	}
}
