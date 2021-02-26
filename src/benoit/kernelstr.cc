# include <benoit/kernelstr.hh>
# include <benoit/logfunc.hh>
# include <benoit/logfuncret.hh>
# include <benoit/t/kernel.hh>
# include <string>
# include <unordered_map>
using namespace std::literals::string_literals;
std::string benoit::kernelstr(benoit::t::kernel kernel) noexcept {
	std::string const funcname = "benoit::kernelstr(benoit::t::kernel)"s;
	benoit::logfunc(funcname);
	std::unordered_map<benoit::t::kernel,std::string> map = {
		{
			benoit::t::kernel::darwinos,
			"Darwin OS"s,
		},
		{
			benoit::t::kernel::dragonflybsd,
			"DragonFly BSD"s,
		},
		{
			benoit::t::kernel::freebsd,
			"FreeBSD"s,
		},
		{
			benoit::t::kernel::hurd,
			"Hurd"s,
		},
		{
			benoit::t::kernel::linux,
			"Linux"s,
		},
		{
			benoit::t::kernel::minix,
			"MINIX"s,
		},
		{
			benoit::t::kernel::netbsd,
			"NetBSD"s,
		},
		{
			benoit::t::kernel::openbsd,
			"OpenBSD"s,
		},
		{
			benoit::t::kernel::unknown,
			"N/A"s,
		}
	};
	std::string str = map[kernel];
	benoit::logfuncret(funcname);
	return str;
}
