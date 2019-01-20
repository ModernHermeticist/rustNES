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
    carry_flag: u8,
    zero_flag: u8,
    interrupt_flag: u8,
    decimal_flag: u8,
    overflow_flag: u8,
    negative_flag: u8,
    instruction: u8, // current instruction cpu is processing
    first_byte_of_interest: u8, // first byte following opcode
    second_byte_of_interest: u8, // second byte following opcode, may be of interest
    ram: Vec<u8>,
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
        carry_flag: 0x0,
        zero_flag: 0x0,
        interrupt_flag: 0x0,
        decimal_flag: 0x0,
        overflow_flag: 0x0,
        negative_flag: 0x0,
        instruction: 0x0,
        first_byte_of_interest: 0x0,
        second_byte_of_interest: 0x0,
        ram: Vec::with_capacity(0x07FF),
    };
    return cpu;
}

impl CPU
{
    pub fn set_instruction(&mut self, instruction: u8)
    {
        self.instruction = instruction;
    }

    pub fn get_instruction(&self) -> u8
    {
        return self.instruction.clone();
    }

    pub fn set_first_byte_of_interest(&mut self, first_byte_of_interest: u8)
    {
        self.first_byte_of_interest = first_byte_of_interest;
    }

    pub fn get_first_byte_of_interest(&self) -> u8
    {
        return self.first_byte_of_interest;
    }

    pub fn set_second_byte_of_interest(&mut self, second_byte_of_interest: u8)
    {
        self.second_byte_of_interest = second_byte_of_interest;
    }

    pub fn get_second_byte_of_interest(&self) -> u8
    {
        return self.second_byte_of_interest;
    }

    pub fn get_pc(&self) -> u16
    {
        return self.pc;
    }

    pub fn increment_pc(&mut self, val: u16)
    {
        self.pc += val;
    }

    pub fn get_a(&self) -> u8
    {
        return self.a;
    }

    pub fn set_a(&mut self, val: u8)
    {
        self.a = val;
    }

    pub fn get_x(&self) -> u8
    {
        return self.x;
    }

    pub fn set_x(&mut self, val: u8)
    {
        self.x = val;
    }

    pub fn get_y(&self) -> u8
    {
        return self.y;
    }

    pub fn set_y(&mut self, val: u8)
    {
        self.y = val;
    }

    pub fn get_carry_flag(&self) -> u8
    {
        return self.carry_flag;
    }

    pub fn get_zero_flag(&self) -> u8
    {
        return self.zero_flag;
    }

    pub fn get_interrupt_flag(&self) -> u8
    {
        return self.interrupt_flag;
    }

    pub fn get_decimal_flag(&self) -> u8
    {
        return self.decimal_flag;
    }

    pub fn get_overflow_flag(&self) -> u8
    {
        return self.overflow_flag;
    }

    pub fn get_negative_flag(&self) -> u8
    {
        return self.negative_flag;
    }

    pub fn execute_opcode(&mut self, op: Opcode)
    {
        let code = get_opcode_code(op);
        let ln = get_left_nibble(code);
        let rn = get_right_nibble(code);

        match ln
        {
            0x0 => self.left_nib_is_0(rn),
            0x1 => self.left_nib_is_1(rn),
            0x2 => self.left_nib_is_2(rn),
            0x3 => self.left_nib_is_3(rn),
            0x4 => self.left_nib_is_4(rn),
            0x5 => self.left_nib_is_5(rn),
            0x6 => self.left_nib_is_6(rn),
            0x7 => self.left_nib_is_7(rn),
            0x8 => self.left_nib_is_8(rn),
            0x9 => self.left_nib_is_9(rn),
            0xA => self.left_nib_is_a(rn),
            0xB => self.left_nib_is_b(rn),
            0xC => self.left_nib_is_c(rn),
            0xD => self.left_nib_is_d(rn),
            0xE => self.left_nib_is_e(rn),
            0xF => self.left_nib_is_f(rn),
            _   => println!("Unsure about this nib."),
        }
    }

    fn left_nib_is_0(&self, rn: u8)
    {

    }

    fn left_nib_is_1(&self, rn: u8)
    {

    }

    fn left_nib_is_2(&self, rn: u8)
    {

    }

    fn left_nib_is_3(&mut self, rn: u8)
    {
        match rn
        {
            0x00 => return ,
            0x01 => self.and_indirect_y(),
            0x02 => return ,
            0x03 => return ,
            0x04 => return ,
            0x05 => return ,
            0x06 => return ,
            0x07 => return ,
            0x08 => return ,
            0x09 => return ,
            0x0A => return ,
            0x0B => return ,
            0x0C => return ,
            0x0D => return ,
            0x0E => return ,
            0x0F => return ,
            _    => return ,
        }
    }

    fn left_nib_is_4(&self, rn: u8)
    {

    }

    fn left_nib_is_5(&self, rn: u8)
    {

    }

    fn left_nib_is_6(&self, rn: u8)
    {

    }

    fn left_nib_is_7(&mut self, rn: u8)
    {
        match rn
        {
            0x00 => return ,
            0x01 => return,
            0x02 => return ,
            0x03 => return ,
            0x04 => return ,
            0x05 => return ,
            0x06 => return ,
            0x07 => return ,
            0x08 => self.sei(),
            0x09 => return ,
            0x0A => return ,
            0x0B => return ,
            0x0C => return ,
            0x0D => return ,
            0x0E => return ,
            0x0F => return ,
            _    => return ,
        }
    }

    fn left_nib_is_8(&self, rn: u8)
    {

    }

    fn left_nib_is_9(&self, rn: u8)
    {

    }

    fn left_nib_is_a(&self, rn: u8)
    {

    }

    fn left_nib_is_b(&self, rn: u8)
    {

    }

    fn left_nib_is_c(&self, rn: u8)
    {

    }

    fn left_nib_is_d(&self, rn: u8)
    {

    }

    fn left_nib_is_e(&self, rn: u8)
    {

    }

    fn left_nib_is_f(&self, rn: u8)
    {
        return;
    }


    ////////////////////////////////////////////////////
    ////////////////////////////////////////////////////
    // INSTRUCTIONS //
    ////////////////////////////////////////////////////
    ////////////////////////////////////////////////////
    pub fn brk(&mut self) // 0x00
    {
        self.increment_pc(2);
    }

    pub fn ora_indirect_x(&mut self) // 0x01
    {
        let target_address = (self.x + self.first_byte_of_interest) & 0xFF;//wraps around zero page
        self.a |= target_address;
        self.increment_pc(2);
    }

    pub fn and_indirect_y(&mut self) // 0x31
    {
        let target_address = self.y + self.first_byte_of_interest; // does not need to wrap
        self.a &= target_address;
        self.increment_pc(2);
    }

    pub fn sei(&mut self) // 0x78
    {
        if self.interrupt_flag == 0x0
        {
            self.interrupt_flag = 0x1;
        }
        else if self.interrupt_flag == 0x1
        {
            self.interrupt_flag = 0x0;
        }
        self.increment_pc(1);
    }
}
