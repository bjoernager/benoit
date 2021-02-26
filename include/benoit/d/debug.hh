# pragma once
namespace benoit {
	namespace d {
		bool constexpr debug =
# if defined(NDEBUG)
		false;
# else
		true;
# endif
	}
}
