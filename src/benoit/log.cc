# include <benoit/d/logdoprint.hh>
# include <benoit/d/printdolog.hh>
# include <benoit/d/thelog.hh>
# include <benoit/log.hh>
# include <benoit/print.hh>
# include <fmt/core.h>
# include <string>
using namespace std::literals::string_literals;
void benoit::log(std::string msg) {
	std::string const funcname = "benoit::log(std::string)"s;
	if(benoit::d::logdoprint) {
		benoit::d::printdolog = false;
		benoit::print(msg,true);
		benoit::d::printdolog = true;
	}
	benoit::d::thelog.insert(benoit::d::thelog.begin(),msg);
}
void benoit::log(std::string const func,std::string msg) {
	std::string const funcname = "benoit::log(std::string const,std::string)"s;
	msg = fmt::format("[{}] {}"s,func,msg);
}
