#include <exec/types.h>
#include <intuition/intuition.h>
#include <intuition/intuitionbase.h>
#include <proto/intuition.h>
#include <proto/exec.h>

struct Library *IntuitionBase = 0;

int intu_open(void) {
    IntuitionBase = OpenLibrary("intuition.library", 0);
    return IntuitionBase != 0;
}

void intu_close(void) {
    if (IntuitionBase) {
        CloseLibrary(IntuitionBase);
        IntuitionBase = 0;
    }
}

struct Window *intu_OpenWindow(WORD w, WORD h) {
    if (!IntuitionBase) return NULL;
    
    struct NewWindow nw = {
        50, 50, w, h,
        0, 1,
        IDCMP_CLOSEWINDOW,
        WFLG_CLOSEGADGET | WFLG_ACTIVATE | WFLG_SMART_REFRESH,
        NULL, NULL,
        (UBYTE *)"Rust Window",
        NULL, NULL,
        100, 50, 640, 256,
        NULL,
        WBENCHWINDOW
    };
    
    return OpenWindow(&nw);
}

void intu_CloseWindow(struct Window *win) {
    if (IntuitionBase && win) CloseWindow(win);
}

struct RastPort *intu_GetRastPort(struct Window *win) {
    return (win && win->RPort) ? win->RPort : NULL;
}

void intu_WaitClose(struct Window *win) {
    if (!win || !IntuitionBase) return;
    
    struct IntuiMessage *msg;
    ULONG sigs = 1L << win->UserPort->mp_SigBit;
    
    while (1) {
        Wait(sigs);
        while ((msg = (struct IntuiMessage *)GetMsg(win->UserPort))) {
            if (msg->Class == IDCMP_CLOSEWINDOW) {
                ReplyMsg((struct Message *)msg);
                return;
            }
            ReplyMsg((struct Message *)msg);
        }
    }
}