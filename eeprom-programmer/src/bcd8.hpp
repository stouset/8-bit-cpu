#ifndef BCD8_H
#define BCD8_H

#include <Arduino.h>

struct BCD8 {
    byte digits[3];
    bool sign;

    public:

    BCD8(uint8_t value);
    BCD8(int8_t  value);

    bool IsPositive() { return !sign; }
    bool IsNegative() { return sign; }

    byte Ones()     { return digits[0]; }
    byte Tens()     { return digits[1]; }
    byte Hundreds() { return digits[2]; }
};

#endif
