# include <benoit/err/wrttofil.hh>
# include <benoit/print.hh>
# include <fmt/core.h>
# include <string>
using namespace std::literals::string_literals;
void benoit::err::wrttofil(std::string const fil) noexcept {
	benoit::print(fmt::format("Unable to write to “{}”."s,fil),true);
}
