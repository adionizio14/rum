pub mod rumload;
pub mod rumassem;
pub mod instructions;

enum Opcode {
    CMov = 0,
    SegLoad = 1,
    SegStore = 2,
    Add = 3,
    Mul = 4,
    Div = 5,
    Nand = 6,
    Halt = 7,
    MapSeg = 8,
    UnmapSeg = 9,
    Output = 10,
    Input = 11,
    LoadPro = 12,
    LoadVel = 13,
}

#[derive(Clone, Debug)]
pub struct UniMachine {

    pub reg: [u32; 8],
    pub mem: Vec<Vec<u32>>,
    pub counter: u32,
    pub unmapped: Vec<u32>,

}
