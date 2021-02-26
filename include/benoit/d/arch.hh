# pragma once
# include <benoit/t/arch.hh>
namespace benoit {
	namespace d {
		benoit::t::arch constexpr arch =
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
	}
}
