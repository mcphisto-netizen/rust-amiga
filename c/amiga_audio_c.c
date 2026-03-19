#include <exec/types.h>
#include <devices/audio.h>
#include <proto/exec.h>

static UBYTE channels_allocated = 0;

int audio_open(void) {
    return 1;
}

void audio_close(void) {
    channels_allocated = 0;
}

int audio_alloc(UWORD ch) {
    if (channels_allocated & ch) {
        return 0;
    }
    channels_allocated |= ch;
    return 1;
}

void audio_free(UWORD ch) {
    channels_allocated &= ~ch;
}

void audio_play(UWORD ch, APTR data, ULONG len, UWORD period) {
    (void)ch; (void)data; (void)len; (void)period;
}