static_assert(((sizeof(short) >= 0x2) && (sizeof(int) >= 0x4) && (sizeof(long) >= 0x4) && (sizeof(long long) >= 0x8) && (sizeof(void *) >= 0x8)),"Benoit expects at least an LLP64 data model.");
# include <benoit/arghandl.hh>
# include <benoit/log.hh>
# include <benoit/logfunc.hh>
# include <benoit/d/numthrds.hh>
# include <benoit/exit.hh>
# include <benoit/loadcfg.hh>
# include <benoit/log.hh>
# include <benoit/logfunc.hh>
# include <benoit/logfuncret.hh>
# include <benoit/main.hh>
# include <benoit/plotmandelbrot.hh>
# include <benoit/print.hh>
# include <benoit/wrtimg.hh>
# include <cstdint>
# include <cstdlib>
# include <fmt/core.h>
# include <string>
# include <thread>
using namespace std::literals::string_literals;
[[noreturn]] void benoit::main(int const argc,char const * * argv) noexcept {
	std::string const funcname = "benoit::main(int const,char const * *)"s;
	benoit::log(fmt::format("{}"s,argv[0x0]));
	benoit::logfunc(funcname);
	benoit::d::numthrds = std::thread::hardware_concurrency();
	benoit::log(fmt::format("{} availabe threads have been detected."s,benoit::d::numthrds));
	benoit::arghandl(argc,argv);
	benoit::loadcfg();
	benoit::wrtimg(benoit::plotmandelbrot());
	benoit::exit(EXIT_SUCCESS);
}
