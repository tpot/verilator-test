// A simple 8-bit counter with a clock, reset and enable inputs
module counter (
    input  logic       clk,
    input  logic       reset,
    input  logic       enable,
    output logic [7:0] count
);

    always_ff @(posedge clk) begin
        if (reset)
            count <= 0;
        else if (enable)
            count <= count + 1;
    end

endmodule
