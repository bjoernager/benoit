# if !defined(BENOIT__HEADER)
# define BENOIT__HEADER
# include <cstddef>
# include <cstdint>
# include <vector>
class benoit {
public:
	benoit(int const argc,char const * * argv);
private:
	enum class imgfmt_t {
		jpeg,
		png,
		ppm,
		webp,
	};
	benoit::imgfmt_t   imgfmt  = benoit::imgfmt_t::ppm;
	long               resx    = 0x4000;
	long               resy    = 0x4000;
	long double        imag    = 0x0p0;
	long double        real    = 0x0p0;
	long double        zoom    = 0x1p0;
	unsigned long long maxiter = 0x100;
	void               print(char const * msg);
	std::size_t        strlen(char const * str) noexcept;
};
# endif
