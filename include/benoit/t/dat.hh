# pragma once
# include <benoit/t/arch.hh>
# include <benoit/t/imgfmt.hh>
# include <benoit/t/kernel.hh>
# include <benoit/t/pos.hh>
# include <string>
# include <vector>
using namespace std::literals::string_literals;
namespace benoit {
	namespace t {
		class dat {
		public:
			benoit::t::arch constexpr static arch =
# if defined(__aarch64__)
			benoit::t::arch::aarch64;
# elif (defined(_M_AMD64) || defined(__amd64) || defined(__amd64__) || defined(__x86_64) || defined(x86_64__))
			benoit::t::arch::amd64;
# elif (defined(_IA64) defined(_M_IA64) || defined(__IA64__) || defined(__ia64__) || defined(__itanium__))
			benoit::t::arch::ia64;
# elif (defined(_ARCH_PPC64) || defined(__powerpc64__) || defined(__PPC64__) || defined(__ppc64__))
			benoit::t::arch::ppc64;
# else
			benoit::t::arch::unknown;
# endif
			benoit::t::imgfmt                  imgfmt = benoit::t::imgfmt::ppm;
			benoit::t::kernel constexpr static kernel =
# if (defined(Macintosh) || defined(macintosh) || defined(__APPLE__) || defined(__MACH__))
			benoit::t::kernel::darwinos;
# elif defined(__DragonFly__)
			benoit::t::kernel::dragonflybsd;
# elif defined(__FreeBSD__)
			benoit::t::kernel::freebsd;
# elif (defined(__GNU__) || defined(__gnu_hurd__))
			benoit::t::kernel::hurd;
# elif defined(__linux__)
			benoit::t::kernel::linux;
# elif defined(__minix)
			benoit::t::kernel::minix;
# elif defined(__NetBSD__)
			benoit::t::kernel::netbsd;
# elif defined(__OpenBSD__)
			benoit::t::kernel::openbsd;
# else
			benoit::t::kernel::unknown;
# endif
			benoit::t::pos pos;
			bool           dobt       = false;
			bool           printdolog = true;
			bool           debug      =
# if defined(NDEBUG)
			false;
# else
			true;
# endif
			long long                ver      = 0x5;
			std::string              cfg      = "benoit.xml"s;
			std::string              outimg   = "image"s;
			std::vector<std::string> thelog   = {};
			unsigned long long       maxiter  = 0x100ull;
			unsigned                 numthrds = 0x1u;
			unsigned                 resx     = 0x100u;
			unsigned                 resy     = 0x100u;
		};
	}
}
