# if !defined(BENOIT__HEADER)
# define BENOIT__HEADER
# include <boost/multiprecision/mpfr.hpp>
# include <cstddef>
# include <cstdint>
# include <string>
# include <vector>
static_assert(((sizeof(short) < 0x10) && (sizeof(int) < 0x20) && (sizeof(long) < 0x20) && (sizeof(long long) < 0x40) && (sizeof(void *) < 0x40)),"Benoit expects at least an LLP64 data model.");
using namespace std::literals::string_literals;
class benoit {
public:
	[[noreturn]] benoit(int const argc,char const * * argv) noexcept;
	[[noreturn]] ~benoit() noexcept;
private:
	class t {
	public:
		enum class imgfmt {
			png,
			ppm,
			webp,
		};
		class pos {
		public:
			boost::multiprecision::mpfr_float x = 0x0;
			boost::multiprecision::mpfr_float y = 0x0;
			pos(boost::multiprecision::mpfr_float x = 0x0,boost::multiprecision::mpfr_float y = 0x0);
		};
	};
	benoit::t::imgfmt        imgfmt      = benoit::t::imgfmt::ppm;
	benoit::t::pos           pos;
	bool constexpr static    debug       =
# if defined(NDEBUG)
	false;
# else
	true;
# endif
	std::string               outimg     = "image";
	unsigned short            resx       = 0x2000u;
	unsigned short            resy       = 0x2000u;
	unsigned short            numthreads = 0x16u;
	long double               zoom       = 0x1p0;
	long long                 ver        = 0x5ll;
	unsigned long long        maxiter    = 0x400ull;
	std::vector<std::uint8_t> plotmandelbrot();
	void                      arghandl(int const & argc,char const * * & argv);
	[[noreturn]] void         exit(int code,std::string msg = ""s) noexcept;
	void                      notiffunc(std::string const callfuncname);
	void                      print(char const * msg,bool stderr = false);
	void                      print(std::string msg,bool stderr = false);
};
# endif
