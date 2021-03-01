# include <benoit/d/thelog.hh>
# include <benoit/d/printdolog.hh>
# include <benoit/print.hh>
# include <fcntl.h>
# include <stdexcept>
# include <string>
# include <unistd.h>
using namespace std::literals::string_literals;
void benoit::print(std::string msg,bool stderr) {
	std::string const funcname = "benoit::print(std::string,bool)"s;
	if(benoit::d::printdolog) {
		benoit::d::thelog.insert(benoit::d::thelog.begin(),msg);
	}
	msg.append("\u000A"s);
	int fil = 0x0;
	if(stderr) {
		fil = 0x1;
	}
	if(::write(fil,msg.c_str(),msg.size()) < 0x0) {
		throw std::runtime_error("Unable to write to Stdout.");
	}
	fsync(fil);
}
