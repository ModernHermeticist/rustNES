use opcode::*;

#[derive(Debug, Clone)]
pub struct CPU
{
    // program counter register
    // accessed internally by cpu fetch logic to increment,
    // an interrupt (NMI, reset, IRQ/BRQ),
    // and using the RTS, JMP, JSR, and BRANCH instructions
    pc: u16,
    a: u8, // accumulator
    x: u8, // x-index, used for address modes
    y: u8, // y-index, used for address modes
    s: u8, // stack pointer, accessed using interrupts, pulls, pushes, and transfers
    p: u8, // status register used by the ALU, used by PHP, PLP, arithmetic , testing and branches
    instruction: Opcode, // current instruction cpu is processing
}

pub fn init_cpu() -> CPU
{
    let cpu = CPU
    {
        pc: 0x10,
        a: 0x0,
        x: 0x0,
        y: 0x0,
        s: 0x0,
        p: 0x0,
        instruction: set_opcode(0x0, 0x0, 0x0, "".to_string()),
    };
    return cpu;
}

impl CPU
{

    pub fn get_pc(&self) -> u16
    {
        return self.pc;
    }

    pub fn increment_pc(&mut self, val: u16)
    {
        self.pc += val;
    }
}
