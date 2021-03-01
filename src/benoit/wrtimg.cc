# include <benoit/d/imgfmt.hh>
# include <benoit/d/outimg.hh>
# include <benoit/d/resx.hh>
# include <benoit/d/resy.hh>
# include <benoit/err/clsfil.hh>
# include <benoit/err/wrttofil.hh>
# include <benoit/logfunc.hh>
# include <benoit/logfuncret.hh>
# include <benoit/print.hh>
# include <benoit/t/imgfmt.hh>
# include <benoit/wrtimg.hh>
# include <cstdint>
# include <cstdlib>
# include <fcntl.h>
# include <fmt/core.h>
# include <png.h>
# include <string>
# include <sys/types.h>
# include <unistd.h>
# include <vector>
# include <webp/encode.h>
# include <webp/types.h>
using namespace std::literals::string_literals;
void benoit::wrtimg(std::vector<std::uint8_t> * img) {
	std::string const funcname = "benoit::wrtimg(std::vector<std::uint8_t> *)"s;
	benoit::logfunc(funcname);
	std::uint8_t *     dat;
	unsigned long long datsiz = 0x0;
	switch(benoit::d::imgfmt) {
	case benoit::t::imgfmt::png:
		break;
	case benoit::t::imgfmt::webp:
		datsiz = WebPEncodeLosslessRGBA(img->data(),benoit::d::resx,benoit::d::resy,(benoit::d::resx * 0x4),&dat);
		break;
	}
	int fil = ::open(benoit::d::outimg.c_str(),(O_CREAT | O_TRUNC | O_WRONLY),0x1B4);
	for(unsigned long long pos = 0x0;(pos < datsiz);++pos) {
		::ssize_t byteswrtn = ::write(fil,&dat[pos],0x1);
		if(byteswrtn < 0x0) {
			benoit::err::wrttofil(benoit::d::outimg);
			return;
		}
	}
	delete dat;
	delete img;
	if(::close(fil) < 0x0) {
		benoit::err::clsfil(benoit::d::outimg);
	}
	benoit::logfuncret(funcname);
}
/*
auto webpconf = ::WebPConfig();
webpconf.lossless = 0x1;
webpconf.quality  = 0x64;
webpconf.method   = 0x6;
webpconf.segments  = 0x4;
webpconf.sns_strength  = 0x4;
webpconf.filter_strength  = 0x0;
webpconf.filter_sharpness = 0x0;
webpconf.autofilter = 0x0;
webpconf.alpha_compression = 0x1;
webpconf.alpha_filtering = 0x2;
webpconf.alpha_quality = 0x64;
webpconf.pass = 0xA;
webpconf.preprocessing = 0x1;
webpconf.partitions = 0x3;
webpconf.partitions = 0x0;
webpconf.use_sharp_yuv = 0x0;
auto conferr = ::WebPValidateConfig(&webpconf);
auto webpimg = ::WebPPicture();
WebPPictureInit(&webpimg);
webpimg.use_argb = 0x1;
webpimg.resx = 0x1;
webpimg.resy = 0x1;
WebPPictureAlloc(&webpimg);
auto webpmemwrit = WebPMemoryWriter();
WebPMemoryWriterInit(&webpmemwrit);
webpimg.writer = WebPMemoryWrite;
webpimg.custom_ptr = &webpmemwrit;
webpimg.argb = {0xFF,0xFF,0x0,0x0};
if(WebPEncode(&webpconf,&webpimg) == 0x0) {
}
else {

}
WebPPictureFree(&webpimg);
auto buf  = std::vector<std::uint8_t>();
buf.push_back(0xFF);
buf.push_back(0x0);
buf.push_back(0x0);
auto fil = std::fstream(benoit::outimt,std::fstream::binary | std::fstream::out | std::fstream::trunc);
if(!fil.is_open()) {
	::_exit(EXIT_FAILURE);
}
*/
