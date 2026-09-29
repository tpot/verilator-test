use hegel::TestCase;
use hegel::generators as gs;
use verilator_test::Counter;

#[derive(Default)]
struct CounterMachine {
    // Verilated implementation
    dut: Counter,

    // Model
    enabled: bool,
    expected_count: u64,
    prev_count: u64,
}

impl CounterMachine {
    fn do_tick(&mut self) {
        self.prev_count = u64::from(self.dut.count());
        self.dut.tick();
        if self.enabled {
            self.expected_count = (self.expected_count + 1) % 256;
        }
    }
}

#[hegel::state_machine]
impl CounterMachine {
    #[rule]
    fn tick(&mut self, _: TestCase) {
        self.do_tick();
    }

    #[rule]
    fn tick_many(&mut self, tc: TestCase) {
        let ticks = tc.draw(gs::integers::<u16>().min_value(1).max_value(1000));
        for _ in 0..ticks {
            self.do_tick();
        }
    }

    #[rule]
    fn enable(&mut self, _: TestCase) {
        self.dut.enable();
        self.enabled = true;
    }

    #[rule]
    fn disable(&mut self, _: TestCase) {
        self.dut.disable();
        self.enabled = false;
    }

    #[rule]
    fn reset(&mut self, _: TestCase) {
        self.dut.reset();
        self.expected_count = 0;
    }

    // Check that our model matches the real thing
    #[invariant(always_run)]
    fn count_matches_model(&self, _: TestCase) {
        assert!(u64::from(self.dut.count()) == self.expected_count);
    }

    // Check that we have either just reset, or the DUT count is greater or
    // eqeual to the previous count.
    #[invariant(always_run)]
    fn count_increasing(&self, _: TestCase) {
        assert!(self.dut.count() == 0 || u64::from(self.dut.count()) >= self.prev_count);
    }
}

#[hegel::test]
fn counter_matches_model(tc: TestCase) {
    hegel::stateful::machine(CounterMachine::default())
        .steps(10000)
        .run(tc);
}
