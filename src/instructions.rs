use std::io::{Read, stdin};
use crate::UniMachine;

/// A Umi is just a synonym for a Universal Machine instruction
type Umi = u32;

/// A Field represents some bitfield of a Umi
pub struct Field {
    width: u32,
    lsb: u32,
}

/// Registers A, B, C of a normal instruction
static RA: Field = Field {width: 3, lsb: 6};
static RB: Field = Field {width: 3, lsb: 3};
static RC: Field = Field {width: 3, lsb: 0};

/// Register A of a Load Value instruction
static RL: Field = Field {width: 3, lsb: 25};

/// The value field for Load Value
static VL: Field = Field {width: 25, lsb: 0};

/// Create a mask (all 1s) of `bits` bits
const fn mask(bits: u32) -> u32 { (1 << bits) - 1 }

/// Given a `field` and `instruction`, extract
/// that field from the instruction as a u32
pub fn get(field: &Field, instruction: Umi) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}

/// takes in the instruction and universal machine
/// runs the cmov operator and updates the universal machine
pub fn cmov(instruction: Umi, mut um: &mut UniMachine){

    let a = get (&RA, instruction) as usize;
    let b = get (&RB, instruction) as usize;
    let c = get (&RC, instruction) as usize;

    if um.reg[c] != 0 {
        um.reg[a] = um.reg[b];
    }

    um.counter += 1;
}

/// takes in the instruction and universal machine
/// runs the segment load operator and updates the universal machine
pub fn seg_load(instruction: Umi, mut um: &mut UniMachine){

    let a = get (&RA, instruction) as usize;
    let b = get (&RB, instruction) as usize;
    let c = get (&RC, instruction) as usize;

    um.reg[a] = um.mem[um.reg[b] as usize][um.reg[c] as usize];

    um.counter += 1;

}

/// takes in the instruction and universal machine
/// runs the segment store operator and updates the universal machine
pub fn seg_store(instruction: Umi, mut um: &mut UniMachine){

    let a = get (&RA, instruction) as usize;
    let b = get (&RB, instruction) as usize;
    let c = get (&RC, instruction) as usize;

    um.mem[um.reg[a] as usize][um.reg[b] as usize] = um.reg[c];

    um.counter += 1;

}

/// takes in the instruction and universal machine
/// runs the add operator and updates the universal machine
pub fn add(instruction: Umi, mut um: &mut UniMachine){

    let a = get (&RA, instruction) as usize;
    let b = get (&RB, instruction) as usize;
    let c = get (&RC, instruction) as usize;

    let n = u64::pow(2, 32);
    um.reg[a] = ((um.reg[b] as u64 + um.reg[c] as u64) % n) as u32;

    um.counter += 1;

}

/// takes in the instruction and universal machine
/// runs the multiply operator and updates the universal machine
pub fn mul(instruction: Umi, mut um: &mut UniMachine){

    let a = get (&RA, instruction) as usize;
    let b = get (&RB, instruction) as usize;
    let c = get (&RC, instruction) as usize;

    let n = u64::pow(2, 32);
    um.reg[a] = ((um.reg[b] as u64 * um.reg[c] as u64) % n) as u32;

    um.counter += 1;

}

/// takes in the instruction and universal machine
/// runs the divide operator and updates the universal machine
pub fn div(instruction: Umi, mut um: &mut UniMachine){

    let a = get (&RA, instruction) as usize;
    let b = get (&RB, instruction) as usize;
    let c = get (&RC, instruction) as usize;

    um.reg[a] = um.reg[b] / um.reg[c];

    um.counter += 1;

}

/// takes in the instruction and universal machine
/// runs the NAND operator and updates the universal machine
pub fn nand(instruction: Umi, mut um: &mut UniMachine){

    let a = get (&RA, instruction) as usize;
    let b = get (&RB, instruction) as usize;
    let c = get (&RC, instruction) as usize;

    let and = um.reg[b]  & um.reg[c] ;
    let nand = !and ;

    um.reg[a] = nand;

    um.counter += 1;
}

/// takes in the instruction and universal machine
/// runs the Halt operator and stops the code
pub fn halt() {
    std::process::exit(0x0100);
}

/// takes in the instruction and universal machine
/// runs the map segment operator and updates the universal machine
pub fn map_seg (instruction: Umi, mut um: &mut UniMachine) {

    let b = get (&RB, instruction) as usize;
    let c = get (&RC, instruction) as usize;

    let new_seg = vec![0; um.reg[c] as usize];

    if um.unmapped.len() > 0 {

        let num = um.unmapped.pop().unwrap();
        um.reg[b] = num;
        um.mem[um.reg[b] as usize] = new_seg;

    }
    else {

        um.mem.push(new_seg);
        um.reg[b] = (um.mem.len() - 1) as u32;

    }

    um.counter += 1;

}

/// takes in the instruction and universal machine
/// runs the unmapped segment operator and updates the universal machine
pub fn unmap_seg (instruction: Umi, um: &mut UniMachine) {

    let c = get (&RC, instruction) as usize;

    um.mem[um.reg[c] as usize].clear();

    um.unmapped.push(um.reg[c] as u32);

    um.counter += 1;

}

/// takes in the instruction and universal machine
/// runs the output operator and outputs register C to terminal
pub fn output(instruction: Umi, mut um: &mut UniMachine){

    let c = get (&RC, instruction) as usize;

    print!("{}", char::from_u32(um.reg[c]).unwrap());

    um.counter += 1;
}

/// takes in the instruction and universal machine
/// runs the input operator and takes in input from std::in
/// and updates universal machine
pub fn input(instruction: Umi, mut um: &mut UniMachine){

    let c = get (&RC, instruction) as usize;
    let mut buf: [u8;1] = [0];
    let char = stdin().read(&mut buf);

    um.reg[c] = match char {
        Ok(1) => buf[0] as u32,
        Ok(0) => u32::max(0,0),

        _ => {
            0
        }
    };

    um.counter += 1;

}

/// takes in the instruction and universal machine
/// runs the load value operator and updates the universal machine
pub fn load_value(instruction: Umi, mut um: &mut UniMachine){
    let a = get (&RL, instruction) as usize;
    um.reg[a] = get (&VL, instruction);

    um.counter += 1;

}

/// takes in the instruction and universal machine
/// runs the load program operator and updates the universal machine
pub fn load_pro(instruction: Umi, mut um: &mut UniMachine){

    let b = get (&RB, instruction) as usize;
    let c = get (&RC, instruction) as usize;

    um.mem[0]  = um.mem[um.reg[b] as usize].clone();


    um.counter = um.reg[c] ;

}



