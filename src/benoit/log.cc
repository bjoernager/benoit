# include <benoit/dat.hh>
# include <benoit/log.hh>
# include <benoit/print.hh>
# include <fmt/core.h>
# include <string>
void benoit::log(std::string msg) {
	if(benoit::dat.debug) {
		benoit::print(msg,true);
	}
	else {
		benoit::dat.thelog.insert(benoit::dat.thelog.begin(),msg);
	}
}
void benoit::log(std::string const func,std::string msg) {
	msg = fmt::format("[{}] {}"s,func,msg);
}
