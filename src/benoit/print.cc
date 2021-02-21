# include <benoit/dat.hh>
# include <benoit/print.hh>
# include <fcntl.h>
# include <stdexcept>
# include <string>
# include <unistd.h>
void benoit::print(char const * msg,bool stderr) {
	std::string const funcname = "benoit::print(char const *)"s;
	 benoit::print(std::string(msg),stderr);
}
void benoit::print(std::string msg,bool stderr) {
	std::string const funcname = "benoit::print(std::string)"s;
	if(benoit::dat.printdolog) {
		benoit::dat.thelog.insert(benoit::dat.thelog.begin(),msg);
	}
	int file = 0x0;
	if(stderr) {
		::open("/dev/stderr",O_WRONLY);
	}
	else {
		::open("/dev/stdout",O_WRONLY);
	}
	msg.append("\u000A");
	if(::write(file,msg.c_str(),msg.size()) < 0x0) {
		throw std::runtime_error("Unable to write to Stdout.");
	}
	if(::close(file) < 0x0) {
		throw std::runtime_error("Unable to close Stdout");
	}
}
