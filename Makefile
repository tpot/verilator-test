VERILATOR := verilator
TOP       := counter
BUILD     := build
SIM       := $(BUILD)/V$(TOP)

.PHONY: all sim clean

all: $(SIM)

$(SIM): counter.sv sim_main.cpp
	$(VERILATOR) \
		--cc counter.sv \
		--top-module $(TOP) \
		--exe sim_main.cpp \
		--Mdir $(BUILD) \
		--build

sim: $(SIM)
	$(SIM)

clean:
	rm -rf build
