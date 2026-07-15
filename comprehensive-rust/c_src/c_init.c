// c_src/c_init.c
#include <stddef.h>   // 提供 NULL 定义

typedef struct {
    int x;
    double y;
} ComplexStruct;

void c_init_struct(ComplexStruct *ptr) {
    if (ptr != NULL) {
        ptr->x = 42;
        ptr->y = 3.14159;
    }
}