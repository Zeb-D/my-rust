#include "interner.hpp"
#include "interner.h"

extern "C" {

StringInterner* interner_new(void) {
    return new StringInterner();
}

void interner_free(StringInterner* interner) {
    delete interner;
}

const char* interner_intern(StringInterner* interner, const char* s) {
    return interner->intern(s);
}

size_t interner_count(const StringInterner* interner) {
    return interner->count();
}

}