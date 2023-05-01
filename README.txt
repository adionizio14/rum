Andrew Dionizio

Help:

I went to Vincent's office hours on 4/19,4/24 and 4/26. I also used code from the rumdump in
my main.rs, rumassem.rs and rumload.rs. The code I used was mostly the bitwise functions
needed.

Implementations:

The implementations I have working are all 13 UM instructions that correctly use the bitwise
functions and also increments the counter variable, the loop in my main that
goes through each instruction given from input and whatever the counter
variable is set to, my big matching function that correctly exectutes whatever op code
is given from the word.

Design Changes:

The only change from my design doc was the representation of the registers are an array
of u32 instead of u8.

Architecture:

main.rs - takes the instructions from the command line or std::in and uses rumload.rs
          to get the words, initializes the universal machine struct, adds the instructions
          to our memory, creates a loop that gets each instruction from the memory and uses
          rumassem.rs to run the instructions.
rumload.rs - contains one function that converts the input from main.rs into a vector of u32s
             and passes it back to main.rs
rumassem.rs - contains a bitwise functions that pull the opcode from the instruction word.
              contains a match statement that runs the corresponding opcode operator. The functions
              are contained in instructions.rs and use the UM struct and instruction but does not
              return anything.
instruction.rs - contains bitwise functions that pull the A,B and C values from the instruction
                 and uses them in each operator function. Each 13 operator instruction is contained
                 in this module, the UM and instruction are taken in and data in the UM struct is updated
                 in each function and nothing is returned.

Timing:

80 million instructions = 5m0.698s = 300.698s
Time for 1 instruction = (300.698s seconds / 80 mil instructions) = 3.758725 * 10^ -6 seconds
Time for 50 million instructions = (50mil instructions * (3.758725 * 10^ -6 seconds)) = 187.936 seconds or 3.132 minutes

It would take around 3.132 minutes to run 50 million instructions

Time Spent:

About 2-3 hours to analyze the assignment.
About 1-2 hours to prepare my design.
About 4 hours to solve the problem/code.

