VERILATOR := verilator
TOP       := counter
BUILD     := build
SIM       := $(BUILD)/V$(TOP)

$(SIM): counter.sv sim_main.cpp
	$(VERILATOR) \
		--cc counter.sv \
		--top-module $(TOP) \
		--exe sim_main.cpp \
		--Mdir $(BUILD) \
		--build

clean:
	rm -rf build

.PHONY: clean
