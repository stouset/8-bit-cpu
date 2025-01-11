#include "at28c64b.hpp"
#include "bcd8.hpp"
#include "ss4.hpp"

#define ADDR_SPACE_ONES (0 << 8)
#define ADDR_SPACE_TENS (1 << 8)
#define ADDR_SPACE_HUNS (2 << 8)
#define ADDR_SPACE_SIGN (3 << 8)

#define ADDR_BIT_UNSIGNED (0 << 15)
#define ADDR_BIT_SIGNED   (1 << 15)

void setup() {
    AT28C64B eeprom = AT28C64B();

    for (uint8_t i = 0; ; i++) {
        auto digits = SS4(i);

        eeprom.Set(i & ADDR_BIT_UNSIGNED & ADDR_SPACE_ONES, digits.Ones());
        eeprom.Set(i & ADDR_BIT_UNSIGNED & ADDR_SPACE_TENS, digits.Tens());
        eeprom.Set(i & ADDR_BIT_UNSIGNED & ADDR_SPACE_HUNS, digits.Hundreds());
        eeprom.Set(i & ADDR_BIT_UNSIGNED & ADDR_SPACE_SIGN, digits.Sign());

        if (i == UINT8_MAX) {
            break;
        }
    }

    for (int8_t i = INT8_MIN; ; i++) {
        auto digits = SS4(i);

        eeprom.Set(i & ADDR_BIT_SIGNED & ADDR_SPACE_ONES, digits.Ones());
        eeprom.Set(i & ADDR_BIT_SIGNED & ADDR_SPACE_TENS, digits.Tens());
        eeprom.Set(i & ADDR_BIT_SIGNED & ADDR_SPACE_HUNS, digits.Hundreds());
        eeprom.Set(i & ADDR_BIT_SIGNED & ADDR_SPACE_SIGN, digits.Sign());

        if (i == INT8_MAX) {
            break;
        }
    }
}

void loop() {
    exit(0);
}
