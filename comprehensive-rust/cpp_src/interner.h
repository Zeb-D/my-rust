// interner.h (C API for FFI)
#ifndef INTERNER_H
#define INTERNER_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct StringInterner StringInterner;

StringInterner* interner_new(void);
void interner_free(StringInterner* interner);
const char* interner_intern(StringInterner* interner, const char* s);
size_t interner_count(const StringInterner* interner);

#ifdef __cplusplus
}
#endif

#endif /* INTERNER_H */
