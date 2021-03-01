# include <benoit/err/clsfil.hh>
# include <benoit/print.hh>
# include <fmt/core.h>
# include <string>
using namespace std::literals::string_literals;
void benoit::err::clsfil(std::string const fil) noexcept {
	benoit::print(fmt::format("Unable to close file “{}”."s,fil),true);
}
