#include "ss4.hpp"

#define SS_NONE (0)

#define SS_HOR_TOP (1 << 5)
#define SS_HOR_MID (1 << 7)
#define SS_HOR_BOT (1 << 0)
#define SS_VERT_TL (1 << 6)
#define SS_VERT_TR (1 << 4)
#define SS_VERT_BL (1 << 3)
#define SS_VERT_BR (1 << 1)
#define SS_DECIMAL (1 << 2)

#define SS_HOR_ALL (SS_HOR_TOP | SS_HOR_MID | SS_HOR_BOT)
#define SS_VERT_LS (SS_VERT_TL | SS_VERT_BL)
#define SS_VERT_RS (SS_VERT_TR | SS_VERT_BR)

constexpr byte kDigits[] = {
    /* 0 */ SS_HOR_TOP | SS_HOR_BOT | SS_VERT_LS | SS_VERT_RS,
    /* 1 */ SS_VERT_RS,
    /* 2 */ SS_HOR_ALL | SS_VERT_TR | SS_VERT_BR,
    /* 3 */ SS_HOR_ALL | SS_VERT_RS,
    /* 4 */ SS_HOR_MID | SS_VERT_TL | SS_VERT_RS,
    /* 5 */ SS_HOR_ALL | SS_VERT_TL | SS_VERT_BL,
    /* 6 */ SS_HOR_ALL | SS_VERT_LS | SS_VERT_BR,
    /* 7 */ SS_HOR_TOP | SS_VERT_RS,
    /* 8 */ SS_HOR_ALL | SS_VERT_LS | SS_VERT_RS,
    /* 9 */ SS_HOR_ALL | SS_VERT_TL | SS_VERT_RS,
};

SS4::SS4(BCD8 digits) {
    values[0] = kDigits[digits.Ones()];
    values[1] = kDigits[digits.Tens()];
    values[2] = kDigits[digits.Hundreds()];
    values[3] = kDigits[digits.IsPositive() ? SS_NONE : SS_HOR_MID];
}
