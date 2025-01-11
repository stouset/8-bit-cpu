#include "at28c64b.hpp"

#define ADDR_MAX ((1 << 13) - 1)

#define PIN_SR_ADDR_CLK    0
#define PIN_SR_ADDR_LO     1
#define PIN_SR_ADDR_HI     2
#define PIN_EEPROM_DATA_0  3
#define PIN_EEPROM_DATA_1  4
#define PIN_EEPROM_DATA_2  5
#define PIN_EEPROM_DATA_3  6
#define PIN_EEPROM_DATA_4  7
#define PIN_EEPROM_DATA_5  8
#define PIN_EEPROM_DATA_6  9
#define PIN_EEPROM_DATA_7 10
#define PIN_EEPROM_OE     11
#define PIN_EEPROM_WE     12

#define PIN_EEPROM_DATA_MIN PIN_EEPROM_DATA_0
#define PIN_EEPROM_DATA_MAX PIN_EEPROM_DATA_7

void pulse(uint8_t pin, bool value);

void    setAddress(uint16_t addr);
void    setData(uint8_t data);
uint8_t getData(void);

AT28C64B::AT28C64B() {
    pinMode(PIN_SR_ADDR_LO, OUTPUT);
    pinMode(PIN_SR_ADDR_HI, OUTPUT);
    pinMode(PIN_EEPROM_WE,  OUTPUT);

    digitalWrite(PIN_EEPROM_WE, LOW);
}

uint8_t AT28C64B::Get(uint16_t addr) {
    setAddress(addr);
    return getData();
}

void AT28C64B::Set(uint16_t addr, byte data) {
    setAddress(addr);
    setData(data);

    pulse(PIN_EEPROM_WE, LOW);
}

void pulse(uint8_t pin, bool value) {
    digitalWrite(pin, value);
    digitalWrite(pin, !value);
}

void shiftOut16LoHi(uint8_t clockPin, uint8_t dataPin1, uint8_t dataPin2, uint8_t val) {
    for (auto i = 0; i < 8; i++) {
        digitalWrite(dataPin1, val & (1 << 0));
        digitalWrite(dataPin2, val & (1 << 8));

        val >>= 1;
    }

    pulse(clockPin, HIGH);
}

void setAddress(uint16_t addr) {
    // mask away excess address bits
    addr &= ADDR_MAX;

    shiftOut16LoHi(PIN_SR_ADDR_CLK, PIN_SR_ADDR_LO, PIN_SR_ADDR_HI, addr);
}

void setData(uint8_t data) {
    for (auto pin = PIN_EEPROM_DATA_MIN; pin <= PIN_EEPROM_DATA_MAX; pin++) {
        pinMode(pin, OUTPUT);
    }

    for (auto i = 0; i <= PIN_EEPROM_DATA_MAX - PIN_EEPROM_DATA_MIN; i++) {
        digitalWrite(PIN_EEPROM_DATA_MIN + i, data & (1 << i));
    }
}

uint8_t getData(void) {
    uint8_t result = 0;

    for (auto pin = PIN_EEPROM_DATA_MIN; pin <= PIN_EEPROM_DATA_MAX; pin++) {
        pinMode(pin, INPUT);
    }

    for (auto i = 0; i <= PIN_EEPROM_DATA_MAX - PIN_EEPROM_DATA_MIN; i++) {
        result |= digitalRead(PIN_EEPROM_DATA_MIN + i) == LOW ? 0 : (1 << i);
    }

    return result;
}
