#ifndef TEXT_ANALYSIS_H
#define TEXT_ANALYSIS_H

#include <stddef.h>
#include <stdbool.h>

typedef struct TextAnalyst TextAnalyst;

typedef struct {
    const char* start;
    size_t length;
    size_t index;
} Token;

typedef enum {
    TA_OK = 0,
    TA_ERR_NULL_POINTER,
    TA_ERR_OUT_OF_MEMORY,
    TA_ERR_OTHER,
} TAError;

/* Return `false` to indicate that no token was found. */
typedef bool (*Tokenizer)(Token* token, void* extra_context);


typedef bool (*TokenCallback)(void* user_context, Token* token, void* result);

/* TextAnalyst constructor */
TextAnalyst* ta_new(void);

/* TextAnalyst destructor */
void ta_free(TextAnalyst* ta);

/* Resets state to clear the current document */
void ta_reset(TextAnalyst* ta);

/* Use custom tokenizer (defaults to whitespace) */
void ta_set_tokenizer(TextAnalyst* ta, Tokenizer* func);

TAError ta_set_text(TextAnalyst* ta, const char* text, size_t len, bool make_copy);

/* Apply `callback` to each token */
size_t ta_foreach_token(const TextAnalyst* ta, const TokenCallback* callback, void* user_context);

/* Get human-readable error message */
const char* ta_error_string(TAError error);

#endif /* TEXT_ANALYSIS_H */
