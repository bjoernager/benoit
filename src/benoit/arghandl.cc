# include <benoit/archstr.hh>
# include <benoit/arghandl.hh>
# include <benoit/d/dobt.hh>
# include <benoit/d/imgfmt.hh>
# include <benoit/d/numthrds.hh>
# include <benoit/d/outimg.hh>
# include <benoit/d/resx.hh>
# include <benoit/d/resy.hh>
# include <benoit/exit.hh>
# include <benoit/helpscrn.hh>
# include <benoit/kernelstr.hh>
# include <benoit/log.hh>
# include <benoit/logfunc.hh>
# include <benoit/logfuncret.hh>
# include <benoit/print.hh>
# include <benoit/t/imgfmt.hh>
# include <charconv>
# include <fmt/core.h>
# include <string>
# include <unordered_map>
using namespace std::literals::string_literals;
void benoit::arghandl(int const & argc,char const * * & argv) {
	std::string const funcname = "benoit::arghandl(int const &,char const * * &)"s;
	benoit::logfunc(funcname);
	if(argc > 0x1) {
		benoit::log(fmt::format("{} argument(s) have been detected."s,(argc - 0x1)));
		for(int pos = 0x1;(pos < argc);++pos) {
			std::string arg = argv[pos];
			benoit::log(fmt::format("Found argument “{}”.",arg));
			std::string::size_type eqpos = arg.find("="s);
			if(eqpos != std::string::npos) {
				std::unordered_map<std::string,bool> strtobool = {
					{
						"false"s,
						false,
					},
					{
						"true"s,
						true,
					}
				};
				std::string invalvalforobj = "Unrecognised value “{}” for object “{}”."s;
				std::string obj = arg.substr(0x0,eqpos);
				benoit::log(funcname,fmt::format("Found object “{}”.",obj));
				std::string val = arg.substr(eqpos + 0x1);
				benoit::log(funcname,fmt::format("Found value “{}”.",val));
				if(obj == "force-backtrace"s) {
					if(!strtobool.contains(val)) {
						benoit::print(fmt::format(invalvalforobj,val,obj));
					}
					else {
						benoit::d::dobt = strtobool[val];
					}
				}
				else if(obj == "format"s) {
					if(val == "PNG"s) {
						benoit::d::imgfmt = benoit::t::imgfmt::png;
					}
					else if(val == "WebP"s) {
						benoit::d::imgfmt = benoit::t::imgfmt::webp;
					}
					else {
						benoit::print(fmt::format(invalvalforobj,val,obj),true);
					}
				}
				else if(obj == "height"s) {
					std::from_chars(val.c_str(),(val.c_str() + val.size()),benoit::d::resy);
					if(benoit::d::resy > 0x10000) {
						benoit::print(fmt::format("Argument “{}” sets the height to {}, but the maximum width is 65536.",arg,benoit::d::resy),true);
						benoit::d::resy = 0x10000;
					}
				}
				else if(obj == "output"s) {
					benoit::d::outimg = val;
				}
				else if(obj == "threads"s) {
					std::from_chars(val.c_str(),(val.c_str() + val.size()),benoit::d::numthrds);
					if(benoit::d::numthrds > 0x10000) {
						benoit::print(fmt::format("Argument “{}” sets the number of threads to {}, but the maximum number of threads is 65536."s,arg,benoit::d::numthrds),true);
						benoit::d::numthrds = 0x10000;
					}
				}
				else if(obj == "width"s) {
					std::from_chars(val.c_str(),(val.c_str() + val.size()),benoit::d::resx);
					if(benoit::d::resx > 0x10000) {
						benoit::print(fmt::format("Argument “{}” sets the width to {}, but the maximum width is 65536."s,arg,benoit::d::resx),true);
						benoit::d::resx = 0x10000;
					}
				}
				else {
					benoit::print(fmt::format("Invalid object “{}”."s,obj),true);
				}
			}
			else {
				if((arg == "help"s) || (arg == "--help"s)) {
					benoit::helpscrn();
				}
				else {
					benoit::print(fmt::format("Invalid argument “{}”."s,arg),true);
				}
			}
		}
	}
	else {
		benoit::log("No arguments have been detected."s);
	}
	switch(benoit::d::imgfmt) {
	case benoit::t::imgfmt::png:
		benoit::d::outimg.append(".png"s);
		break;
	case benoit::t::imgfmt::webp:
		if((benoit::d::resx > 0x3FFF) || (benoit::d::resy > 0x3FFF)) {
			benoit::exit(EXIT_FAILURE,"WebP does not support a resolution of more than 16383."s);
		}
		benoit::d::outimg.append(".webp"s);
		break;
	}
	benoit::log(funcname,fmt::format("The output image will be \u201C{}\u201D."s,benoit::d::outimg));
	benoit::logfuncret(funcname);
}
