use crate::instructions::{add, cmov, div, halt, input, load_pro, load_value, map_seg, mul, nand, output, seg_load, seg_store, unmap_seg};
use crate::{Opcode, UniMachine};

/// A Umi is just a synonym for a Universal Machine instruction
type Umi = u32;

/// A Field represents some bitfield of a Umi
pub struct Field {
    width: u32,
    lsb: u32,
}

/// The Opcode
static OP: Field = Field {width: 4, lsb: 28};

/// Create a mask (all 1s) of `bits` bits
fn mask(bits: u32) -> u32 { (1 << bits) - 1 }

/// Given a `field` and `instruction`, extract
/// that field from the instruction as a u32
pub fn get(field: &Field, instruction: Umi) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}

/// Given an instruction word, extract the opcode
pub fn op(instruction: Umi) -> u32 {
    (instruction >> OP.lsb) & mask(OP.width)
}

///Given an 'inst' the operator instruction is run
pub fn assemble(inst: Umi, um: &mut UniMachine) {
    match get(&OP, inst) {

        o if o == Opcode::CMov as u32 => {
            cmov(inst, um);
        },
        o if o == Opcode::SegLoad as u32 => {
            seg_load(inst, um);
        }
        o if o == Opcode::SegStore as u32 => {
            seg_store(inst, um);
        }
        o if o == Opcode::Add as u32 => {
            add(inst, um);
        }
        o if o == Opcode::Mul as u32 => {
            mul(inst, um);
        }
        o if o == Opcode::Div as u32 => {
            div(inst, um);
        }
        o if o == Opcode::Nand as u32 => {
            nand(inst, um);
        }
        o if o == Opcode::Halt as u32 => {
            halt();
        }
        o if o == Opcode::MapSeg as u32 => {
            map_seg(inst, um);
        }
        o if o == Opcode::UnmapSeg as u32 => {
            unmap_seg(inst,um);
        }
        o if o == Opcode::Output as u32 => {
            output(inst, um);
        }
        o if o == Opcode::Input as u32 => {
            input(inst,um);
        }
        o if o == Opcode::LoadPro as u32 => {
           load_pro(inst,um);
        }
        o if o == Opcode::LoadVel as u32 => {
            load_value(inst, um);
        }
        _ => {
            todo!()
        }
    }
}