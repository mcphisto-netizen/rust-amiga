#define ALIGN2 __attribute__((aligned(2)))
#include <exec/types.h>
#include <dos/dos.h>
#include <proto/dos.h>
#include <proto/exec.h>

extern struct DosLibrary *DOSBase;

BPTR nim_open(char *name, LONG mode) {
    if (!DOSBase) return 0;
    return Open((CONST_STRPTR)name, mode);
}

LONG nim_read(BPTR fh, void *buf, LONG len) {
    if (!DOSBase || !fh) return -1;
    return Read(fh, buf, len);
}

LONG nim_write(BPTR fh, const void *buf, LONG len) {
    if (!DOSBase || !fh) return -1;
    return Write(fh, (APTR)buf, len);
}

LONG nim_close(BPTR fh) {
    if (!DOSBase || !fh) return 0;
    return Close(fh);
}

BPTR nim_output(void) {
    if (!DOSBase) return 0;
    return Output();
}

BPTR nim_lock(char *name, LONG access) {
    if (!DOSBase) return 0;
    return Lock((CONST_STRPTR)name, access);
}

void nim_unlock(BPTR lock) {
    if (DOSBase && lock) UnLock(lock);
}

LONG nim_examine(BPTR lock, void *fib) {
    if (!DOSBase || !lock) return 0;
    return Examine(lock, (struct FileInfoBlock *)fib);
}

LONG nim_nextdosentry(BPTR lock, void *fib, LONG mode) {
    if (!DOSBase || !lock) return 0;
    return ExNext(lock, (struct FileInfoBlock *)fib);
}