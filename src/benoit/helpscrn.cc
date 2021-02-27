# include <benoit/archstr.hh>
# include <benoit/helpscrn.hh>
# include <benoit/d/arch.hh>
# include <benoit/d/kernel.hh>
# include <benoit/d/logdoprint.hh>
# include <benoit/d/printdolog.hh>
# include <benoit/d/ver.hh>
# include <benoit/exit.hh>
# include <benoit/kernelstr.hh>
# include <benoit/logfunc.hh>
# include <benoit/print.hh>
# include <fmt/core.h>
# include <string>
# include <unordered_map>
using namespace std::literals::string_literals;
[[noreturn]] void benoit::helpscrn() noexcept {
	std::string const funcname = "benoit::helpscrn()"s;
	benoit::logfunc(funcname);
	std::string cmdate = ""s;
	{
		std::string date = __DATE__;
		std::string time = __TIME__;
		std::unordered_map<std::string,std::string> monthmap = {
			{
				"Jan"s,
				"01"s,
			},
			{
				"Feb"s,
				"02"s,
			},
			{
				"Mar"s,
				"03"s,
			},
			{
				"Apr"s,
				"04"s,
			},
			{
				"May"s,
				"05"s,
			},
			{
				"Jun"s,
				"06"s,
			},
			{
				"Jul"s,
				"07"s,
			},
			{
				"Aug"s,
				"08"s,
			},
			{
				"Sep"s,
				"09"s,
			},
			{
				"Oct"s,
				"10"s,
			},
			{
				"Nov"s,
				"11"s,
			},
			{
				"Dec"s,
				"12"s,
			},
		};
		std::string year  = date.substr(0x7);
		std::string month = monthmap[date.substr(0x0,0x3)];
		std::string day   = date.substr(0x4,0x2);
		cmdate            = fmt::format("{}-{}-{}T{}"s,year,month,day,time);
	}
	std::string cppver = ""s;
	{
		long constexpr cplusplus = __cplusplus;
		switch(cplusplus) {
			default:
				cppver = fmt::format("{}"s,cplusplus);
				break;
			case 0x30C1Fl:
				cppver = "C++98 / C++03"s;
				break;
			case 0x3118Fl:
				cppver = "C++11"s;
				break;
			case 0x312BAl:
				cppver = "C++14"s;
				break;
			case 0x313E7l:
				cppver = "C++17"s;
				break;
			case 0x31512l:
				cppver = "C++20"s;
				break;
		}
	}
	std::string datmod = fmt::format("{}/{}/{}/{}/{}",sizeof(short),sizeof(int),sizeof(long),sizeof(long long),sizeof(void *));
	if constexpr((sizeof(short) == 0x2) && (sizeof(int) == 0x4) && (sizeof(long) == 0x4) && (sizeof(long long) == 0x8) && (sizeof(void *) == 0x8)) {
		datmod = fmt::format("LLP64 (noob / {})"s,datmod);
	}
	else if constexpr((sizeof(short) == 0x2) && (sizeof(int) == 0x4) && (sizeof(long) == 0x8) && (sizeof(long long) == 0x8) && (sizeof(void *) == 0x8)) {
		datmod = fmt::format("LP64 ({})"s,datmod);
	}
	else if constexpr((sizeof(short) == 0x2) && (sizeof(int) == 0x8) && (sizeof(long) == 0x8) && (sizeof(long long) == 0x8) && (sizeof(void *) == 0x8)) {
		datmod = fmt::format("ILP64 ({})"s,datmod);
	}
	else if constexpr((sizeof(short) == 0x8) && (sizeof(int) == 0x8) && (sizeof(long) == 0x8) && (sizeof(long long) == 0x8) && (sizeof(void *) == 0x8)) {
		datmod = fmt::format("SILP64 (wtf? / {})"s,datmod);
	}
	else {
		datmod = fmt::format("{} AKA how the fuck did this get compiled?"s,datmod);
	}
	benoit::d::logdoprint = false;
	benoit::d::printdolog = false;
	benoit::print(""s);
	benoit::print(fmt::format("benoit {}",benoit::d::ver));
	benoit::print("Copyright 2021 Gabriel Jensen"s);
	benoit::print(""s);
	benoit::print("Arguments:"s);
	benoit::print("\u0009alpha={false,true}:"s);
	benoit::print("\u0009\u0009Sets whether or not to use alpha or background colour for rendered image."s);
	benoit::print("\u0009force-backtrace={false,true}:"s);
	benoit::print("\u0009\u0009Forces the backtrace of the at programme exit."s);
	benoit::print("\u0009height={0..65536}:"s);
	benoit::print("\u0009\u0009Sets the height for the rendered image."s);
	benoit::print("\u0009help, --help:"s);
	benoit::print("\u0009\u0009Displays this information screen."s);
	benoit::print("\u0009maximum-iterations={0..18446744073709551615}:"s);
	benoit::print("\u0009\u0009Sets the maximum number of iterations allowed."s);
	benoit::print("\u0009threads={0..65536}:"s);
	benoit::print("\u0009\u0009Sets the number of threads that will be used."s);
	benoit::print("\u0009height={0..65536}:"s);
	benoit::print("\u0009\u0009Sets the width for the rendered image."s);
	benoit::print(""s);
	benoit::print("Compilation Information:"s);
	benoit::print(fmt::format("\u0009Architecture:          {}"s,benoit::archstr(benoit::d::arch)));
	benoit::print(fmt::format("\u0009Compilation Date:      {}"s,cmdate));
	benoit::print(fmt::format("\u0009Compiler C++ Standard: {}"s,cppver));
	benoit::print(fmt::format("\u0009Data Model:            {}"s,datmod));
	benoit::print(fmt::format("\u0009System Kernel:         {}"s,benoit::kernelstr(benoit::d::kernel)));
	benoit::print(""s);
	benoit::d::logdoprint = true;
	benoit::d::printdolog = true;
	benoit::exit(EXIT_SUCCESS);
}
