CXX=g++
CXXFLAGS=-Iinclude -D_ATFILE_SOURCE -D_FORTIFY_SOURCE=0x2 -D_LARGEFILE_SOURCE -D_LARGEFILE64_SOURCE -D_ISOC99_SOURCE -D_ISOC11_SOURCE -D_ISOC2X_SOURCE -D_POSIX_C_SOURCE=200809L -D_XOPEN_SOURCE -D_XOPEN_SOURCE_EXTENDED -D__STDC_WANT_IEC_60559_BFP_EXT__ -D__STDC_WANT_IEC_60559_FUNCS_EXT__ -D__STDC_WANT_IEC_60559_TYPES_EXT__ -D__STDC_WANT_LIB_EXT2__=0x1
ifneq ($(debug),1)
CXXFLAGS += -DNDEBUG -g
endif
CXXFLAGS += -std=c++20 -Wall -Wextra -Wpedantic
CXXFLAGS += -march=native -mtune=native -O3
LDFLAGS = -lfmt -lgmp -lmpfr -lpng -lpthread -lpugixml -lwebp
HDRS_CXX = \
	include/benoit/archstr.hh \
	include/benoit/arghandl.hh \
	include/benoit/crtcfg.hh \
	include/benoit/d/arch.hh \
	include/benoit/d/cfg.hh \
	include/benoit/d/debug.hh \
	include/benoit/d/dobt.hh \
	include/benoit/d/imgfmt.hh \
	include/benoit/d/kernel.hh \
	include/benoit/d/logdoprint.hh \
	include/benoit/d/maxiter.hh \
	include/benoit/d/numthrds.hh \
	include/benoit/d/outimg.hh \
	include/benoit/d/pos.hh \
	include/benoit/d/printdolog.hh \
	include/benoit/d/resx.hh \
	include/benoit/d/resy.hh \
	include/benoit/d/thelog.hh \
	include/benoit/d/ver.hh \
	include/benoit/exit.hh \
	include/benoit/helpscrn.hh \
	include/benoit/kernelstr.hh \
	include/benoit/loadcfg.hh \
	include/benoit/log.hh \
	include/benoit/logfunc.hh \
	include/benoit/logfuncret.hh \
	include/benoit/main.hh \
	include/benoit/plotmandelbrot.hh \
	include/benoit/print.hh \
	include/benoit/t/arch.hh \
	include/benoit/t/imgfmt.hh \
	include/benoit/t/kernel.hh \
	include/benoit/t/pos.hh \
	include/benoit/t/rgba.hh \
	include/benoit/t/thrddat.hh \
	include/benoit/wrtimg.hh
SRCS_CXX = \
	src/benoit/archstr.cc \
	src/benoit/arghandl.cc \
	src/benoit/crtcfg.cc \
	src/benoit/d/cfg.cc \
	src/benoit/d/dobt.cc \
	src/benoit/d/imgfmt.cc \
	src/benoit/d/logdoprint.cc \
	src/benoit/d/maxiter.cc \
	src/benoit/d/numthrds.cc \
	src/benoit/d/outimg.cc \
	src/benoit/d/pos.cc \
	src/benoit/d/printdolog.cc \
	src/benoit/d/resx.cc \
	src/benoit/d/resy.cc \
	src/benoit/d/thelog.cc \
	src/benoit/d/ver.cc \
	src/benoit/exit.cc \
	src/benoit/helpscrn.cc \
	src/benoit/kernelstr.cc \
	src/benoit/loadcfg.cc \
	src/benoit/log.cc \
	src/benoit/logfunc.cc \
	src/benoit/logfuncret.cc \
	src/benoit/main.cc \
	src/benoit/plotmandelbrot.cc \
	src/benoit/print.cc \
	src/benoit/t/pos/pos.cc \
	src/benoit/wrtimg.cc \
	src/main.cc
OBJS_CXX=$(SRCS_CXX:.cc=.o)
OBJS=$(OBJS_CXX)
benoit: $(OBJS)
	$(CXX) $(LDFLAGS) -o $@ $(OBJS)
$(OBJS_CXX): $(HDRS_CXX) $(SRCS_CXX)
.PHONY: clean
clean:
	rm $(OBJS)
.PHONY: purge
purge:
	rm benoit $(OBJS)
