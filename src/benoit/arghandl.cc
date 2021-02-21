# include <benoit/archstr.hh>
# include <benoit/arghandl.hh>
# include <benoit/dat.hh>
# include <benoit/exit.hh>
# include <benoit/helpscrn.hh>
# include <benoit/kernelstr.hh>
# include <benoit/log.hh>
# include <benoit/logfunc.hh>
# include <benoit/logfuncret.hh>
# include <benoit/print.hh>
# include <benoit/t/imgfmt.hh>
# include <fmt/core.h>
# include <string>
# include <unordered_map>
using namespace std::literals::string_literals;
void benoit::arghandl(int const & argc,char const * * & argv) {
	std::string const funcname = "benoit::arghandl(int const &,char const * * &)"s;
	benoit::logfunc(funcname);
	if(argc < 0x2) {
		benoit::logfuncret(funcname);
		return;
	}
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
			if(obj == "debug"s) {
				if(!strtobool.contains(val)) {
					benoit::print(fmt::format(invalvalforobj,val,obj));
				}
				else {
					benoit::dat.debug = strtobool[val];
				}
			}
			else if(obj == "format"s) {
				if(val == "PNG"s) {
					benoit::dat.imgfmt = benoit::t::imgfmt::png;
				}
				else if(val == "PPM"s) {
					benoit::dat.imgfmt = benoit::t::imgfmt::ppm;
				}
				else if(val == "WebP"s) {
					benoit::dat.imgfmt = benoit::t::imgfmt::webp;
				}
				else {
					benoit::print(fmt::format(invalvalforobj,val,obj),true);
				}
			}
			else if(obj == "output"s) {
				benoit::dat.outimg = val;
			}
			else if(obj == "threads"s) {
				benoit::dat.numthrds = std::stoi(val);
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
	switch(benoit::dat.imgfmt) {
	case benoit::t::imgfmt::png:
		benoit::dat.outimg.append(".png"s);
		break;
	case benoit::t::imgfmt::ppm:
		benoit::dat.outimg.append(".ppm"s);
		break;
	case benoit::t::imgfmt::webp:
		if((benoit::dat.resx > 0x1000) || (benoit::dat.resy > 0x1000)) {
			benoit::exit(EXIT_FAILURE,"WebP does not support a resolution of more than 4096"s);
		}
		benoit::dat.outimg.append(".webp"s);
		break;
	}
	benoit::log(funcname,fmt::format("The output image will be \u201C{}\u201D."s,benoit::dat.outimg));
	benoit::logfuncret(funcname);
}
