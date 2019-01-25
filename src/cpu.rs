use opcode::*;

//DEFINES

//NES specific hardware defines

static PPU_CTRL_REG1      : u16 = 0x2000;
static PPU_CTRL_REG2      : u16 = 0x2001;
static PPU_STATUS         : u16 = 0x2002;
static PPU_SPR_ADDR       : u16 = 0x2003;
static PPU_SPR_DATA       : u16 = 0x2004;
static PPU_SCROLL_REG     : u16 = 0x2005;
static PPU_ADDRESS        : u16 = 0x2006;
static PPU_DATA           : u16 = 0x2007;

static SND_REGISTER       : u16 = 0x4000;
static SND_SQUARE1_REG    : u16 = 0x4000;
static SND_SQUARE2_REG    : u16 = 0x4004;
static SND_TRIANGLE_REG   : u16 = 0x4008;
static SND_NOISE_REG      : u16 = 0x400C;
static SND_DELTA_REG      : u16 = 0x4010;
static SND_MASTERCTRL_REG : u16 = 0x4015;

static SPR_DMA            : u16 = 0x4014;
static JOYPAD_PORT        : u16 = 0x4016;
static JOYPAD_PORT1       : u16 = 0x4016;
static JOYPAD_PORT2       : u16 = 0x4017;



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
    s: u16, // stack pointer, accessed using interrupts, pulls, pushes, and transfers
    // points between 0x0100 and 0x01FF in memory
    stack_offset: u16,
    p: u8, // flag register
    // NVssDIZC
    instruction: u8, // current instruction cpu is processing
    first_byte_of_interest: u8, // first byte following opcode
    second_byte_of_interest: u8, // second byte following opcode, may be of interest
    memory: Vec<u16>,
}

pub fn init_cpu() -> CPU
{
    let mut cpu = CPU
    {
        pc: 0x10,
        a: 0x0,
        x: 0x0,
        y: 0x0,
        s: 0x1000,
        stack_offset: 0x1000,
        p: 0b11010000,
        instruction: 0x0,
        first_byte_of_interest: 0x0,
        second_byte_of_interest: 0x0,
        memory: vec![0x0; 0xFFFF],
    };
    cpu.memory[PPU_STATUS as usize] = 0b10100000;
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

    pub fn get_s(&self) -> u16
    {
        return self.s;
    }

    pub fn set_s(&mut self, val: u16)
    {
        self.s = 0x1000 + val;
    }

    pub fn get_carry_flag(&self) -> u8
    {
        return self.p & 0b00000001;
    }

    pub fn set_carry_flag(&mut self)
    {
        self.p |= 0b00000001;
    }

    pub fn reset_carry_flag(&mut self)
    {
        self.p &= 0b11111110;
    }

    pub fn get_zero_flag(&self) -> u8
    {
        return self.p & 0b00000010;
    }

    pub fn set_zero_flag(&mut self)
    {
        self.p |= 0b00000010;
    }

    pub fn reset_zero_flag(&mut self)
    {
        self.p &= 0b11111101;
    }

    pub fn get_interrupt_flag(&self) -> u8
    {
        return self.p & 0b00000100;
    }

    pub fn set_interrupt_flag(&mut self)
    {
        self.p |= 0b00000100;
    }

    pub fn reset_interrupt_flag(&mut self)
    {
        self.p &= 0b11111011;
    }

    pub fn get_decimal_flag(&self) -> u8
    {
        return self.p & 0b00001000;
    }

    pub fn set_decimal_flag(&mut self)
    {
        self.p |= 0b00001000;
    }

    pub fn reset_decimal_flag(&mut self)
    {
        self.p &= 0b11110111;
    }

    pub fn get_overflow_flag(&self) -> u8
    {
        return self.p & 0b01000000;
    }

    pub fn set_overflow_flag(&mut self)
    {
        self.p |= 0b01000000;
    }

    pub fn reset_overflow_flag(&mut self)
    {
        self.p &= 0b10111111;
    }

    pub fn get_negative_flag(&self) -> u8
    {
        return self.p & 0b10000000;
    }

    pub fn set_negative_flag(&mut self)
    {
        self.p |= 0b10000000;
    }

    pub fn reset_negative_flag(&mut self)
    {
        self.p &= 0b01111111;
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

    fn left_nib_is_0(&mut self, rn: u8)
    {
        match rn
        {
            0x00 => self.brk(),
            0x01 => self.ora_indirect_x(),
            0x02 => return,
            0x03 => return ,
            0x04 => return,
            0x05 => return ,
            0x06 => return ,
            0x07 => return ,
            0x08 => return ,
            0x09 => return,
            0x0A => return ,
            0x0B => return ,
            0x0C => return ,
            0x0D => return,
            0x0E => return ,
            0x0F => return ,
            _    => return ,
        }
    }

    fn left_nib_is_1(&mut self, rn: u8)
    {
        match rn
        {
            0x00 => self.bpl(),
            0x01 => return ,
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
            0x0D => return,
            0x0E => return ,
            0x0F => return ,
            _    => return ,
        }
    }

    fn left_nib_is_2(&mut self, rn: u8)
    {
        match rn
        {
            0x00 => return,
            0x01 => return ,
            0x02 => return,
            0x03 => return ,
            0x04 => return,
            0x05 => return ,
            0x06 => return ,
            0x07 => return ,
            0x08 => return ,
            0x09 => return,
            0x0A => return ,
            0x0B => return ,
            0x0C => return ,
            0x0D => return,
            0x0E => return ,
            0x0F => return ,
            _    => return ,
        }
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

    fn left_nib_is_4(&mut self, rn: u8)
    {
        match rn
        {
            0x00 => return,
            0x01 => return ,
            0x02 => return,
            0x03 => return ,
            0x04 => return,
            0x05 => return ,
            0x06 => return ,
            0x07 => return ,
            0x08 => return ,
            0x09 => return,
            0x0A => return ,
            0x0B => return ,
            0x0C => return ,
            0x0D => return,
            0x0E => return ,
            0x0F => return ,
            _    => return ,
        }
    }

    fn left_nib_is_5(&mut self, rn: u8)
    {
        match rn
        {
            0x00 => return,
            0x01 => return ,
            0x02 => return,
            0x03 => return ,
            0x04 => return,
            0x05 => return ,
            0x06 => return ,
            0x07 => return ,
            0x08 => return ,
            0x09 => return,
            0x0A => return ,
            0x0B => return ,
            0x0C => return ,
            0x0D => return,
            0x0E => return ,
            0x0F => return ,
            _    => return ,
        }
    }

    fn left_nib_is_6(&mut self, rn: u8)
    {
        match rn
        {
            0x00 => return,
            0x01 => return ,
            0x02 => return,
            0x03 => return ,
            0x04 => return,
            0x05 => return ,
            0x06 => return ,
            0x07 => return ,
            0x08 => return ,
            0x09 => return,
            0x0A => return ,
            0x0B => return ,
            0x0C => return ,
            0x0D => return,
            0x0E => return ,
            0x0F => return ,
            _    => return ,
        }
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

    fn left_nib_is_8(&mut self, rn: u8)
    {
        match rn
        {
            0x00 => return ,
            0x01 => return ,
            0x02 => return ,
            0x03 => return ,
            0x04 => return ,
            0x05 => return ,
            0x06 => return ,
            0x07 => return ,
            0x08 => self.dey(),
            0x09 => return ,
            0x0A => self.txa(),
            0x0B => return ,
            0x0C => return ,
            0x0D => self.sta_absolute(),
            0x0E => return ,
            0x0F => return ,
            _    => return ,
        }
    }

    fn left_nib_is_9(&mut self, rn: u8)
    {
        match rn
        {
            0x00 => return ,
            0x01 => return ,
            0x02 => return,
            0x03 => return ,
            0x04 => return ,
            0x05 => return ,
            0x06 => return ,
            0x07 => return ,
            0x08 => self.tya(),
            0x09 => return,
            0x0A => self.txs(),
            0x0B => return ,
            0x0C => return ,
            0x0D => return ,
            0x0E => return ,
            0x0F => return ,
            _    => return ,
        }
    }

    fn left_nib_is_a(&mut self, rn: u8)
    {
        match rn
        {
            0x00 => self.ldy_immediate(),
            0x01 => self.lda_indirect_x(),
            0x02 => self.ldx_immediate(),
            0x03 => return ,
            0x04 => self.ldy_zero_page(),
            0x05 => self.lda_zero_page(),
            0x06 => self.ldx_zero_page(),
            0x07 => return ,
            0x08 => self.tay(),
            0x09 => self.lda_immediate(),
            0x0A => self.tax(),
            0x0B => return ,
            0x0C => self.ldy_absolute(),
            0x0D => self.lda_absolute(),
            0x0E => self.ldx_absolute(),
            0x0F => return ,
            _    => return ,
        }
    }

    fn left_nib_is_b(&mut self, rn: u8)
    {
        match rn
        {
            0x00 => self.bcs(),
            0x01 => self.lda_indirect_y(),
            0x02 => return,
            0x03 => return ,
            0x04 => self.ldy_zero_page_x(),
            0x05 => self.lda_zero_page_x(),
            0x06 => self.ldx_zero_page_y(),
            0x07 => return ,
            0x08 => return ,
            0x09 => self.lda_absolute_y(),
            0x0A => return ,
            0x0B => return ,
            0x0C => self.ldy_absolute_x(),
            0x0D => self.lda_absolute_x(),
            0x0E => self.ldx_absolute_y(),
            0x0F => return ,
            _    => return ,
        }
    }

    fn left_nib_is_c(&mut self, rn: u8)
    {
        match rn
        {
            0x00 => return,
            0x01 => return ,
            0x02 => return,
            0x03 => return ,
            0x04 => return,
            0x05 => return ,
            0x06 => return ,
            0x07 => return ,
            0x08 => self.iny(),
            0x09 => self.cmp_immediate(),
            0x0A => self.dex(),
            0x0B => return ,
            0x0C => return ,
            0x0D => return,
            0x0E => return ,
            0x0F => return ,
            _    => return ,
        }
    }

    fn left_nib_is_d(&mut self, rn: u8)
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
            0x08 => self.cld(),
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

    fn left_nib_is_e(&mut self, rn: u8)
    {
        match rn
        {
            0x00 => return,
            0x01 => return ,
            0x02 => return,
            0x03 => return ,
            0x04 => return,
            0x05 => return ,
            0x06 => return ,
            0x07 => return ,
            0x08 => self.inx(),
            0x09 => return,
            0x0A => return ,
            0x0B => return ,
            0x0C => return ,
            0x0D => return,
            0x0E => return ,
            0x0F => return ,
            _    => return ,
        }
    }

    fn left_nib_is_f(&mut self, rn: u8)
    {
        match rn
        {
            0x00 => return,
            0x01 => return ,
            0x02 => return,
            0x03 => return ,
            0x04 => return,
            0x05 => return ,
            0x06 => return ,
            0x07 => return ,
            0x08 => return ,
            0x09 => return,
            0x0A => return ,
            0x0B => return ,
            0x0C => return ,
            0x0D => return,
            0x0E => return ,
            0x0F => return ,
            _    => return ,
        }
    }


    ////////////////////////////////////////////////////
    ////////////////////////////////////////////////////
    // INSTRUCTIONS //
    ////////////////////////////////////////////////////
    ////////////////////////////////////////////////////
    fn brk(&mut self) // 0x00
    {
        self.increment_pc(2);
    }

    fn ora_indirect_x(&mut self) // 0x01
    {
        self.x += self.first_byte_of_interest & 0xFF;//wraps around zero page
        self.a |= self.memory[self.x as usize] as u8;
        self.increment_pc(2);
    }

    fn and_indirect_y(&mut self) // 0x31
    {
        self.y += self.memory[self.first_byte_of_interest as usize] as u8; // does not need to wrap
        self.a &= self.y;
        self.increment_pc(2);
    }

    fn sei(&mut self) // 0x78
    {
        self.set_interrupt_flag();
        self.increment_pc(1);
    }

    fn cld(&mut self) // 0xD8
    {
        self.reset_decimal_flag();
        self.increment_pc(1);
    }

    fn lda_immediate(&mut self) // 0xA9
    {
        self.a = self.first_byte_of_interest;
        self.increment_pc(2);
    }

    fn lda_zero_page(&mut self) // 0xA5
    {
        self.a = self.memory[self.first_byte_of_interest as usize] as u8;
        self.increment_pc(2);
    }

    fn lda_zero_page_x(&mut self) // 0xB5
    {
        let temp_address = self.first_byte_of_interest + self.x;

        self.a = self.memory[temp_address as usize] as u8;
        self.increment_pc(2);
    }

    fn lda_absolute(&mut self) // 0xAD
    {
        let mut temp_address: u16 = 0x0000;
        temp_address |= self.second_byte_of_interest as u16;
        temp_address <<= 8;
        temp_address |= self.first_byte_of_interest as u16;
        self.a = self.memory[temp_address as usize] as u8;
        self.increment_pc(3);
    }

    fn lda_absolute_x(&mut self) // 0xBD
    {
        let mut temp_address: u16 = 0x0000;
        temp_address |= self.second_byte_of_interest as u16;
        temp_address <<= 8;
        temp_address |= self.first_byte_of_interest as u16;
        temp_address += self.x as u16;
        self.a = self.memory[temp_address as usize] as u8;
        self.increment_pc(3);
    }

    fn lda_absolute_y(&mut self) // 0xB9
    {
        let mut temp_address: u16 = 0x0000;
        temp_address |= self.second_byte_of_interest as u16;
        temp_address <<= 8;
        temp_address |= self.first_byte_of_interest as u16;
        temp_address += self.y as u16;
        self.a = self.memory[temp_address as usize] as u8;
        self.increment_pc(3);
    }

    fn lda_indirect_x(&mut self) // 0xA1
    {
        self.x += self.first_byte_of_interest & 0xFF;
        self.a = self.memory[self.x as usize] as u8;
        self.increment_pc(2);
    }

    fn lda_indirect_y(&mut self) // 0xB1
    {
        self.y += self.memory[self.first_byte_of_interest as usize] as u8; // does not need to wrap
        self.a = self.y;
        self.increment_pc(2);
    }

    fn ldx_immediate(&mut self) // 0xA2
    {
        self.x = self.first_byte_of_interest;
        self.increment_pc(2);
    }

    fn ldx_zero_page(&mut self) // 0xA6
    {
        self.x = self.memory[self.first_byte_of_interest as usize] as u8;
        self.increment_pc(2);
    }

    fn ldx_zero_page_y(&mut self) // 0xB6
    {
        let temp_address = self.first_byte_of_interest + self.y;

        self.x = self.memory[temp_address as usize] as u8;
        self.increment_pc(2);
    }

    fn ldx_absolute(&mut self) // 0xAE
    {
        let mut temp_address: u16 = 0x0000;
        temp_address |= self.second_byte_of_interest as u16;
        temp_address <<=8;
        temp_address |= self.first_byte_of_interest as u16;
        self.x = self.memory[temp_address as usize] as u8;
        self.increment_pc(3);
    }

    fn ldx_absolute_y(&mut self) // 0xBE
    {
        let mut temp_address: u16 = 0x0000;
        temp_address |= self.second_byte_of_interest as u16;
        temp_address <<= 8;
        temp_address |= self.first_byte_of_interest as u16;
        temp_address += self.y as u16;
        self.x = self.memory[temp_address as usize] as u8;
        self.increment_pc(3);
    }

    fn ldy_immediate(&mut self) // 0xA0
    {
        self.y = self.first_byte_of_interest;
        self.increment_pc(2);
    }

    fn ldy_zero_page(&mut self) // 0xA4
    {
        self.y = self.memory[self.first_byte_of_interest as usize] as u8;
        self.increment_pc(2);
    }

    fn ldy_zero_page_x(&mut self) // 0xB4
    {
        let temp_address = self.first_byte_of_interest + self.x;

        self.y = self.memory[temp_address as usize] as u8;
        self.increment_pc(2);
    }

    fn ldy_absolute(&mut self) // 0xAC
    {
        let mut temp_address: u16 = 0x0000;
        temp_address |= self.second_byte_of_interest as u16;
        temp_address <<= 8;
        temp_address |= self.first_byte_of_interest as u16;
        self.y = self.memory[temp_address as usize] as u8;
        self.increment_pc(3);
    }

    fn ldy_absolute_x(&mut self) // 0xBC
    {
        let mut temp_address: u16 = 0x0000;
        temp_address |= self.second_byte_of_interest as u16;
        temp_address <<= 8;
        temp_address |= self.first_byte_of_interest as u16;
        temp_address += self.x as u16;
        self.y = self.memory[temp_address as usize] as u8;
        self.increment_pc(3);
    }

    fn sta_absolute(&mut self) // 0x8D
    {
        let mut temp_address: u16 = 0x0000;
        temp_address |= self.second_byte_of_interest as u16;
        temp_address <<= 8;
        temp_address |= self.first_byte_of_interest as u16;
        self.memory[temp_address as usize] = self.a as u16;
        self.increment_pc(3);
    }

    fn txs(&mut self) // 0x9A
    {
        self.s = self.stack_offset + self.x as u16;
        self.increment_pc(1);
    }

    fn bpl(&mut self) // 0x10
    {
        if self.get_negative_flag() == 0b10000000
        {
            self.increment_pc(2);
        }
        else
        {
            self.pc += self.first_byte_of_interest as u16;
        }
    }

    fn bcs(&mut self) // 0xB0
    {
        if self.get_carry_flag() == 0b00000001
        {
            self.pc += self.first_byte_of_interest as u16;
        }
        else
        {
            self.increment_pc(2);
        }
    }

    fn cmp_immediate(&mut self) // 0xC9
    {
        if self.get_a() > self.first_byte_of_interest
        {
            self.set_carry_flag();
            self.reset_zero_flag();
            self.reset_negative_flag();

        }
        else if self.get_a() == self.first_byte_of_interest
        {
            self.set_carry_flag();
            self.set_zero_flag();
            self.reset_negative_flag();
        }
        else if self.get_a() < self.first_byte_of_interest
        {
            self.set_negative_flag();
            self.reset_carry_flag();
            self.reset_zero_flag();
        }
        self.increment_pc(2);
    }

    fn tax(&mut self) //0xAA
    {
        self.x = self.a;
        self.increment_pc(1);
    }

    fn txa(&mut self) //0x8A
    {
        self.a = self.x;
        self.increment_pc(1);
    }

    fn dex(&mut self) // 0xCA
    {
        self.x -= 0x01;
        self.increment_pc(1);
    }

    fn inx(&mut self) // 0xE8
    {
        self.x += 0x01;
        self.increment_pc(1);
    }

    fn tay(&mut self) // 0xA8
    {
        self.y = self.a;
        self.increment_pc(1);
    }

    fn tya(&mut self) // 0x98
    {
        self.a = self.y;
        self.increment_pc(1);
    }

    fn dey(&mut self) // 0x88
    {
        self.y -= 0x01;
        self.increment_pc(1);
    }

    fn iny(&mut self) // 0xC8
    {
        self.y += 0x01;
        self.increment_pc(1);
    }
}
