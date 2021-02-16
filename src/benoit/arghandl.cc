# include <benoit.hh>
# include <fmt/core.h>
void benoit::arghandl(int const & argc,char const * * & argv) {
	std::string const funcname = "benoit::arghandl(int const &,char const * * &)"s;
	this->notiffunc(funcname);
	if(argc < 0x2) {
		return;
	}
	for(int pos = 0x1;(pos < argc);++pos) {
		std::string arg = argv[pos];
		if(arg == "--help") {
			this->print(fmt::format("benoit {}",this->ver));
			this->exit(EXIT_SUCCESS);
		}
	}
}
