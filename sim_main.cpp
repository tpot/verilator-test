#include "Vcounter.h"

void tick(Vcounter &dut)
{
    //    printf("Tick!\n");

    dut.clk = 0;

    dut.eval();

    dut.clk = 1;
    dut.eval();
}

int main()
{
    Vcounter dut;

    // Establish initial inputs
    dut.clk = 0;
    dut.reset = 1;
    dut.enable = 0;
    dut.eval();

    // Clock reset
    dut.clk = 1;
    dut.eval();
    dut.clk = 0;
    dut.eval();

    // Release reset and enable counting
    dut.reset = 0;
    dut.enable = 1;
    dut.eval();

    for (int i = 0; i < 10; ++i)
    {
        dut.clk = 1;
        dut.eval();
        printf("count = %d\n", static_cast<int>(dut.count));
        dut.clk = 0;
        dut.eval();
    }

    return 0;
}
