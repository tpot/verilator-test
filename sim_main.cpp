#include "Vcounter.h"

int main()
{
    Vcounter dut;

    dut.reset  = 1;
    dut.enable = 0;
    dut.clk    = 0;

    dut.eval();

    dut.clk = 1;
    dut.eval();

    return 0;
}
