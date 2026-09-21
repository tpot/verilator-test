#include "Vcounter.h"

void init(Vcounter &dut)
{
    dut.clk = 0;
    dut.reset = 0;
    dut.enable = 0;
    dut.eval();
}

void clock_low(Vcounter &dut)
{
    dut.clk = 0;
    dut.eval();
}

void clock_high(Vcounter &dut)
{
    dut.clk = 1;
    dut.eval();
}

void tick(Vcounter &dut)
{
    clock_low(dut);
    clock_high(dut);
}

void reset(Vcounter& dut)
{
    dut.reset = 1;
    tick(dut);

    dut.reset = 0;
    dut.eval();
}

int main()
{
    Vcounter dut;

    init(dut);
    reset(dut);
    assert(dut.count == 0);

    dut.enable = 1;

    tick(dut);
    assert(dut.count == 1);

    tick(dut);
    assert(dut.count == 2);
}
