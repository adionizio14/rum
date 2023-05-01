use std::env;
use rum::{rumassem, UniMachine};
use rum::rumload;

fn main() {

    //gets our input from command line/standard input
    let input = env::args().nth(1);

    //gets the instruction vector usinf rumload
    let instructions = rumload::load(input.as_deref());

    //initializes Universal Machine
    let mut um = UniMachine {
        reg: [0;8],
        mem: Vec::new(),
        counter: 0,
        unmapped: Vec::new(),
    };

    //instructions are placed into the first row of our 2D memory vector
    um.mem.push(instructions);

    //loop is created to go through instructions
    loop {

        //gets the instruction from the first row and whatever the counter is set to
        let instruction = um.mem[0][um.counter as usize];

        //runs the instruction
        rumassem::assemble(instruction, &mut um);


    }
}
