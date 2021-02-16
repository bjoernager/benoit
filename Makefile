CXX=clang++
CXXFLAGS=-Iinclude
ifneq ($(DEBUG),1)
CXXFLAGS += -DNDEBUG
endif
ifeq ($(LUMA__X),1)
CXXFLAGS += -DLUMA__X=true
endif
CXXFLAGS += -std=c++20 -Wall -Wextra -Wpedantic
CXXFLAGS += -march=native -mtune=native -O3
LDFLAGS = -lfmt -lgmp -lmpfr -lpthread -lwebp
HDRS_CXX = \
	include/benoit.hh
SRCS_CXX = \
	src/benoit/arghandl.cc \
	src/benoit/benoit.cc \
	src/benoit/~benoit.cc \
	src/benoit/exit.cc \
	src/benoit/notiffunc.cc \
	src/benoit/plotmandelbrot.cc \
	src/benoit/print.cc \
	src/benoit/t/pos/pos.cc \
	src/main.cc
SRCS=$(SRCS_CXX)
OBJS=$(SRCS:.cc=.o)
benoit: $(OBJS)
	$(CXX) $(LDFLAGS) -o $@ $(OBJS)
$(OBJS): $(HDRS_CXX) $(SRCS_CXX)
.PHONY: clean
clean:
	rm benoit $(OBJS)
