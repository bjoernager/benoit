# include <benoit.hh>
# include <cstddef>
std::size_t benoit::strlen(char const * str) noexcept {
	std::size_t len = 0x0;
	while(str[len] != '\0') {
		++len;
	}
	return len;
}
