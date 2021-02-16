# include <benoit.hh>
# include <cstdlib>
# include <string>
# include <unistd.h>
using namespace std::literals::string_literals;
[[noreturn]] benoit::~benoit() noexcept {
	std::string const funcname = "benoit::~benoit()"s;
	this->notiffunc(funcname);
	this->exit(EXIT_SUCCESS);
}
