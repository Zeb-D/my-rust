// text_analysis.c
#include "text_analysis.h"
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

struct TextAnalyst {
    char* text;
    size_t len;
    bool own_copy;
    Tokenizer tokenizer;
    size_t index;   // 用于迭代
};

TextAnalyst* ta_new(void) {
    TextAnalyst* ta = calloc(1, sizeof(TextAnalyst));
    if (!ta) return NULL;
    ta->tokenizer = NULL; // 默认使用空格
    return ta;
}

void ta_free(TextAnalyst* ta) {
    if (!ta) return;
    if (ta->own_copy) free(ta->text);
    free(ta);
}

void ta_reset(TextAnalyst* ta) {
    if (!ta) return;
    ta->index = 0;
}

void ta_set_tokenizer(TextAnalyst* ta, Tokenizer* func) {
    if (ta) ta->tokenizer = func ? *func : NULL;
}

TAError ta_set_text(TextAnalyst* ta, const char* text, size_t len, bool make_copy) {
    if (!ta || !text) return TA_ERR_NULL_POINTER;
    // 清理旧文本
    if (ta->own_copy) free(ta->text);
    ta->text = NULL;
    ta->len = 0;
    ta->own_copy = false;

    if (make_copy) {
        char* copy = malloc(len + 1);
        if (!copy) return TA_ERR_OUT_OF_MEMORY;
        memcpy(copy, text, len);
        copy[len] = '\0';
        ta->text = copy;
        ta->own_copy = true;
    } else {
        ta->text = (char*)text; // 丢弃 const
    }
    ta->len = len;
    ta->index = 0;
    return TA_OK;
}

static bool default_tokenizer(Token* token, void* extra) {
    TextAnalyst* ta = (TextAnalyst*)extra;
    if (!ta || ta->index >= ta->len) return false;

    const char* start = ta->text + ta->index;
    while (ta->index < ta->len && isspace(ta->text[ta->index])) ta->index++;
    if (ta->index >= ta->len) return false;

    size_t begin = ta->index;
    while (ta->index < ta->len && !isspace(ta->text[ta->index])) ta->index++;
    token->start = ta->text + begin;
    token->length = ta->index - begin;
    token->index = begin;
    return true;
}

size_t ta_foreach_token(const TextAnalyst* ta, const TokenCallback* callback, void* user_context) {
    if (!ta || !callback || !*callback) return 0;
    // 使用当前 tokenizer（如果没有设置，使用默认）
    Tokenizer tok = ta->tokenizer ? ta->tokenizer : default_tokenizer;
    // 需要一个可修改的副本，因为我们要改变索引
    TextAnalyst mutable_copy = *ta;  // 浅拷贝，但 index 会被修改
    mutable_copy.index = 0;

    size_t count = 0;
    Token token;
    bool cont = true;
    while (cont && tok(&token, &mutable_copy)) {
        bool callback_result = (*callback)(user_context, &token, NULL);
        if (!callback_result) break;
        count++;
    }
    return count;
}

const char* ta_error_string(TAError error) {
    switch (error) {
        case TA_OK: return "No error";
        case TA_ERR_NULL_POINTER: return "Null pointer";
        case TA_ERR_OUT_OF_MEMORY: return "Out of memory";
        default: return "Unknown error";
    }
}