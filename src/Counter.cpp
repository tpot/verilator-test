#include "Counter.h"

Counter::Counter() : ctx(), dut(&ctx)
{
    dut.clk = 0;
    dut.reset = 0;
    dut.enable = 0;

    dut.eval();

    assert(this->count() == 0);
}

void Counter::tick()
{
    dut.clk = 1;
    dut.eval();
    ctx.timeInc(5);

    dut.clk = 0;
    dut.eval();
    ctx.timeInc(5);
}

void Counter::reset()
{
    dut.reset = 1;
    tick();

    dut.reset = 0;
    dut.eval();
}

void Counter::enable()
{
    dut.enable = 1;
    dut.eval();
}

void Counter::disable()
{
    dut.enable = 0;
    dut.eval();
}

uint8_t Counter::count() const
{
    return dut.count;
}

std::unique_ptr<Counter> new_counter()
{
    return std::make_unique<Counter>();
}
