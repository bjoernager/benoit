# include <benoit/archstr.hh>
# include <benoit/logfunc.hh>
# include <benoit/logfuncret.hh>
# include <benoit/t/arch.hh>
# include <string>
# include <unordered_map>
using namespace std::literals::string_literals;
std::string benoit::archstr(benoit::t::arch arch) noexcept {
	std::string const funcname = "benoit::archstr(benoit::t::arch)"s;
	benoit::logfunc(funcname);
	std::unordered_map<benoit::t::arch,std::string> map = {
		{
			benoit::t::arch::aarch64,
			"ARM64 / AArch64"s,
		},
		{
			benoit::t::arch::amd64,
			"AMD64 / x86-64"s,
		},
		{
			benoit::t::arch::ia64,
			"IA-64"s,
		},
		{
			benoit::t::arch::ppc64,
			"PPC64"s,
		},
		{
			benoit::t::arch::unknown,
			"N/A"s,
		}
	};
	std::string str = map[arch];
	benoit::logfuncret(funcname);
	return str;
}
