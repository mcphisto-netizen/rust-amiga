#include <exec/types.h>
#include <graphics/gfx.h>
#include <graphics/gfxbase.h>
#include <proto/graphics.h>

struct Library *GfxBase = 0;

int gfx_open(void) {
    GfxBase = OpenLibrary("graphics.library", 0);
    return GfxBase != 0;
}

void gfx_close(void) {
    if (GfxBase) {
        CloseLibrary(GfxBase);
        GfxBase = 0;
    }
}

void gfx_SetAPen(struct RastPort *rp, UWORD pen) {
    if (GfxBase && rp) SetAPen(rp, pen);
}

void gfx_Move(struct RastPort *rp, WORD x, WORD y) {
    if (GfxBase && rp) Move(rp, x, y);
}

void gfx_Draw(struct RastPort *rp, WORD x, WORD y) {
    if (GfxBase && rp) Draw(rp, x, y);
}

void gfx_DrawEllipse(struct RastPort *rp, WORD x, WORD y, WORD rx, WORD ry) {
    if (GfxBase && rp) DrawEllipse(rp, x, y, rx, ry);
}

void gfx_RectFill(struct RastPort *rp, WORD x1, WORD y1, WORD x2, WORD y2) {
    if (GfxBase && rp) RectFill(rp, x1, y1, x2, y2);
}

void gfx_BltClear(APTR mem, ULONG bytes, ULONG flags) {
    if (GfxBase && mem) BltClear(mem, bytes, flags);
}

void gfx_LoadRGB4(struct ViewPort *vp, UWORD *colors, WORD count) {
    if (GfxBase && vp && colors) LoadRGB4(vp, colors, count);
}