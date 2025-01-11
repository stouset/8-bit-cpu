#include "bcd8.hpp"

BCD8::BCD8(uint8_t value) {
    digits[0] = div(value, 1e0).rem;
    digits[1] = div(value, 1e1).rem;
    digits[2] = div(value, 1e2).rem;

    sign = false;
}

BCD8::BCD8(int8_t value) {
    digits[0] = div(value, 1e0).rem;
    digits[1] = div(value, 1e1).rem;
    digits[2] = div(value, 1e2).rem;

    sign = (value < 0);
}
