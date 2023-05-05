use std::env;
use rum::{rumassem, UniMachine};
use rum::rumload;

fn main() {
    // Gets our input from command line/standard input
    let input = env::args().nth(1);

    // Gets the instruction vector using rumload
    let instructions = rumload::load(input.as_deref());

    // Initializes Universal Machine
    let mut um = UniMachine {
        reg: [0; 8],
        mem: vec![instructions],
        counter: 0,
        unmapped: Vec::new(),
    };


    // Loop through instructions
    while um.counter < um.mem[0].len() as u32 {
        let instruction = um.mem[0][um.counter as usize];
        rumassem::assemble(instruction, &mut um);

    }
}


