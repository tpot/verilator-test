#include <iostream>
#include <assert.h>

#include "Vcounter.h"

#ifdef NDEBUG
#error NDEBUG must not be defined for asserts to be anbled
#endif

using namespace std;

class Simulator
{
public:

    // Initialise logic inputs
    Simulator() : ctx(), dut(&ctx)
    {
        dut.clk = 0;
        dut.reset = 0;
        dut.enable = 0;

        dut.eval();

        assert(this->count() == 0);
    }

    void tick()
    {
        dut.clk = 1;
        dut.eval();
        ctx.timeInc(5);

        dut.clk = 0;
        dut.eval();
        ctx.timeInc(5);
    }

    void reset()
    {
        dut.reset = 1;
        tick();

        dut.reset = 0;
        dut.eval();
    }

    void enable()
    {
        dut.enable = 1;
        dut.eval();
    }

    void disable()
    {
        dut.enable = 0;
        dut.eval();
    }

    uint8_t count() const
    {
        return dut.count;
    }

private:
    VerilatedContext ctx;
    Vcounter dut;
};

int main()
{
    cout << ">>> Tests started" << endl;

    Simulator sim;

    // Counter starts at 0
    assert(sim.count() == 0);

    // After reset counter should still be zero
    sim.reset();
    assert(sim.count() == 0);

    // Enable counter
    sim.enable();
    assert(sim.count() == 0);

    // A tick when enabled increments the counter
    sim.tick();
    assert(sim.count() == 1);

    sim.tick();
    assert(sim.count() == 2);

    // Disable counter
    sim.disable();
    assert(sim.count() == 2);

    // A tick when disabled does not affect the counter
    sim.tick();
    assert(sim.count() == 2);

    // After reset counter should be zero
    sim.reset();
    assert(sim.count() == 0);

    cout << ">>> Tests completed" << endl;
}
