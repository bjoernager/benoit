# include <benoit.hh>
# include <fcntl.h>
# include <stdexcept>
# include <unistd.h>
void benoit::print(char const * msg) {
	auto file = ::open("/dev/stdout",O_WRONLY);
	if(::write(file,msg,this->strlen(msg)) < 0x0) {
		throw std::runtime_error("Unable to write to Stdout.\u000A");
	}
	if(::close(file) < 0x0) {
		throw std::runtime_error("Unable to close Stdout\u000A");
	}
}
