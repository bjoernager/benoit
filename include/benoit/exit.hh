# pragma once
# include <string>
using namespace std::string_literals;
namespace benoit {
	[[noreturn]] void exit(int code,std::string msg = ""s) noexcept;
}
