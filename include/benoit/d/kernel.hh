# pragma once
# include <benoit/t/kernel.hh>
namespace benoit {
	namespace d {
		benoit::t::kernel constexpr kernel =
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
	}
}
