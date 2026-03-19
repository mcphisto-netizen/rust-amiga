#include <setjmp.h>
#include <stdint.h>

typedef uint32_t NI32;
typedef NI32 JmpBuf[20];
typedef JmpBuf *PBuf;

NI32 nim_setjmp(PBuf env) {
    return (NI32)setjmp((jmp_buf *)env);
}

__attribute__((noreturn))
void nim_longjmp(PBuf env, NI32 val) {
    longjmp((jmp_buf *)env, (int)val);
    __builtin_unreachable();
}