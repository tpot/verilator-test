VERILATOR := verilator
TOP       := counter
BUILD     := build
SIM       := $(BUILD)/V$(TOP)

$(SIM): rtl/src/counter.sv cpp/src/sim_main.cpp
	$(VERILATOR) \
		--cc rtl/src/counter.sv \
		--top-module $(TOP) \
		--exe cpp/src/sim_main.cpp \
		--Mdir $(BUILD) \
		--build

clean:
	rm -rf build

.PHONY: clean
