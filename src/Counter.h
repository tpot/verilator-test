#pragma once

#include <memory>
#include "Vcounter.h"

class Counter
{
public:

    Counter();

    void tick();
    void reset();
    void enable();
    void disable();
    uint8_t count() const;

private:
    VerilatedContext ctx;
    Vcounter dut;
};

std::unique_ptr<Counter> new_counter();
