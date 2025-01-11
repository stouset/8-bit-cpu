#ifndef AT28C64B_H
#define AT28C64B_H

#include <Arduino.h>

struct AT28C64B {
    public:

    AT28C64B();

    uint8_t Get(uint16_t addr);
    void    Set(uint16_t addr, byte data);
};

#endif
