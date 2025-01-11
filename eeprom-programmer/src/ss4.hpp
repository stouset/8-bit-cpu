#ifndef SS4_H
#define SS4_H

#include "bcd8.hpp"

#include <Arduino.h>

struct SS4 {
    uint8_t values[4];

    public:

    SS4(BCD8 digits);

    SS4(uint8_t value) : SS4(BCD8(value)) {}
    SS4(int8_t  value) : SS4(BCD8(value)) {}

    byte Sign()     { return values[3]; }
    byte Hundreds() { return values[2]; }
    byte Tens()     { return values[1]; }
    byte Ones()     { return values[0]; }
};

#endif
