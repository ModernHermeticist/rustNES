#[derive(Debug, Clone)]
pub struct Opcode
{
    code: u8,
    length: usize,
    cycles: u8,
    description: String,
}

pub fn set_opcode(code: u8, length: usize, cycles: u8, description: String) -> Opcode
{
    let op = Opcode { description, code, length, cycles };

    return op;
}

pub fn print_opcode(opcode: Opcode)
{
    println!("{:?}", opcode);
}

pub fn get_opcode_length(opcode: Opcode) -> usize
{
    return opcode.length;
}

pub fn print_opcode_description(opcode: Opcode)
{
    println!("{}", opcode.description);
}

pub fn get_opcode_description(opcode: Opcode) -> String
{
    return opcode.description;
}

pub fn get_opcode_code(opcode: Opcode) -> u8
{
    return opcode.code;
}

pub fn get_left_nibble(byte: u8) -> u8
{
    return (byte & 0xF0) >> 4;
}

pub fn get_right_nibble(byte: u8) -> u8
{
    return byte & 0x0F;
}

// This is going to find the appropriate
// first part of the opcode. Then it's
// going to call the appropriate function
// to get the second part of the opcode.
// That second function will call the Opcode
// builder and return it back out.
// pub fn start_opcode_build(byte: u8) -> Opcode
pub fn build_opcode(byte: u8) -> Opcode
{
    let ln = get_left_nibble(byte);
    let rn = get_right_nibble(byte);
    let mut opcode = set_opcode(0x00, 0, 0, " ".to_string());

    match ln
    {
        0x0 =>
        {
            opcode = left_nib_is_0(rn);
        },
        0x1 =>
        {
            opcode = left_nib_is_1(rn);
        },
        0x2 =>
        {
            opcode = left_nib_is_2(rn);
        },
        0x3 =>
        {
            opcode = left_nib_is_3(rn);
        },
        0x4 =>
        {
            opcode = left_nib_is_4(rn);
        },
        0x5 =>
        {
            opcode = left_nib_is_5(rn);
        },
        0x6 =>
        {
            opcode = left_nib_is_6(rn);
        },
        0x7 =>
        {
            opcode = left_nib_is_7(rn);
        },
        0x8 =>
        {
            opcode = left_nib_is_8(rn);
        },
        0x9 =>
        {
            opcode = left_nib_is_9(rn);
        },
        0xA =>
        {
            opcode = left_nib_is_a(rn);
        },
        0xB =>
        {
            opcode = left_nib_is_b(rn);
        },
        0xC =>
        {
            opcode = left_nib_is_c(rn);
        },
        0xD =>
        {
            opcode = left_nib_is_d(rn);
        },
        0xE =>
        {
            opcode = left_nib_is_e(rn);
        },
        0xF =>
        {
            opcode = left_nib_is_f(rn);
        },
        _   => println!("Unsure about this nib."),
    }
    return opcode;
}

fn left_nib_is_0(rn: u8) -> Opcode
{
    let mut opcode = set_opcode(0x00, 0, 0, " ".to_string());
    match rn
    {
        0x0 => opcode = set_opcode(0x00, 1, 7, "BRK".to_string()),
        // add a cycle if page boundary crossed
        0x1 => opcode = set_opcode(0x01, 2, 6, "ORA Indirect, X".to_string()),
        0x2 => opcode = set_opcode(0x02, 0, 0, "Not used.".to_string()),
        0x3 => opcode = set_opcode(0x03, 0, 0, "Not used.".to_string()),
        0x4 => opcode = set_opcode(0x04, 0, 0, "Not used.".to_string()),
        0x5 => opcode = set_opcode(0x05, 2, 2, "ORA Zero Page".to_string()),
        0x6 => opcode = set_opcode(0x06, 2, 5, "ASL Zero Page".to_string()),
        0x7 => opcode = set_opcode(0x07, 0 ,0, "Not used.".to_string()),
        0x8 => opcode = set_opcode(0x08, 0 , 3, "PHP (Push processor status)".to_string()),
        0x9 => opcode = set_opcode(0x09, 2, 2, "ORA Immediate".to_string()),
        0xA => opcode = set_opcode(0x0A, 1, 2, "ASL Accumulator".to_string()),
        0xB => opcode = set_opcode(0x0B, 0, 0, "Not used.".to_string()),
        0xC => opcode = set_opcode(0x0C, 0, 0, "Not used.".to_string()),
        0xD => opcode = set_opcode(0x0D, 3, 4, "ORA Absolute".to_string()),
        0xE => opcode = set_opcode(0x0E, 3, 6, "ASL Absolute".to_string()),
        0xF => opcode = set_opcode(0x0F, 0, 0, "Not used.".to_string()),
        _   => println!("Not used."),
    }
    return opcode;
}

fn left_nib_is_1(rn: u8) -> Opcode
{
    let mut opcode = set_opcode(0x00, 0, 0, " ".to_string());
    match rn
    {
        // add 1 to cycles if branch is taken, and 1 more if page crossing
        0x0 => opcode = set_opcode(0x10, 2, 2, "BPL (Branch on PLus)".to_string()),
        // add 1 cycle if page boundary crossed
        0x1 => opcode = set_opcode(0x11, 2, 5, "ORA Indirect, Y".to_string()),
        0x2 => opcode = set_opcode(0x12, 0, 0, "Not used.".to_string()),
        0x3 => opcode = set_opcode(0x13, 0, 0, "Not used.".to_string()),
        0x4 => opcode = set_opcode(0x14, 0, 0, "Not used.".to_string()),
        0x5 => opcode = set_opcode(0x15, 2, 3, "ORA Zero Page, X".to_string()),
        0x6 => opcode = set_opcode(0x16, 2, 6, "ASL Zero Page, X".to_string()),
        0x7 => opcode = set_opcode(0x17, 0, 0, "Not used.".to_string()),
        0x8 => opcode = set_opcode(0x18, 1, 2, "CLC (Clear Carry flag)".to_string()),
        // add 1 cycle if page boundary crossed
        0x9 => opcode = set_opcode(0x19, 3, 4, "ORA Absolute, Y".to_string()),
        0xA => opcode = set_opcode(0x1A, 0, 0, "Not used.".to_string()),
        0xB => opcode = set_opcode(0x1B, 0, 0, "Not used.".to_string()),
        0xC => opcode = set_opcode(0x1C, 0, 0, "Not used.".to_string()),
        // add 1 cycle if page boundary crossed
        0xD => opcode = set_opcode(0x1D, 3, 4, "ORA Absolute, X".to_string()),
        0xE => opcode = set_opcode(0x1E, 3, 7, "ASL Absolute, X".to_string()),
        0xF => opcode = set_opcode(0x1F, 0, 0, "Not used.".to_string()),
        _   => println!("Not used."),
    }
    return opcode;
}

fn left_nib_is_2(rn: u8) -> Opcode
{
    let mut opcode = set_opcode(0x00, 0, 0, " ".to_string());
    match rn
    {
        0x0 => opcode = set_opcode(0x20, 3, 6, "JSR (Jump to SubRoutine)".to_string()),
        0x1 => opcode = set_opcode(0x21, 2, 6, "AND Indirect, X".to_string()),
        0x2 => opcode = set_opcode(0x22, 0, 0, "Not used.".to_string()),
        0x3 => opcode = set_opcode(0x23, 0, 0, "Not used.".to_string()),
        0x4 => opcode = set_opcode(0x24, 2, 3, "BIT (test BITs) Zero Page".to_string()),
        0x5 => opcode = set_opcode(0x25, 2, 2, "AND Zero Page".to_string()),
        0x6 => opcode = set_opcode(0x26, 2, 5, "ROL (ROtate Left) Zero Page".to_string()),
        0x7 => opcode = set_opcode(0x27, 0, 0, "Not used.".to_string()),
        0x8 => opcode = set_opcode(0x28, 1, 4, "PLP (PuLl Processor status)".to_string()),
        0x9 => opcode = set_opcode(0x29, 2, 2, "AND Immediate".to_string()),
        0xA => opcode = set_opcode(0x2A, 1, 2, "ROL (ROtate Left) Accumulator".to_string()),
        0xB => opcode = set_opcode(0x2B, 0, 0, "Not used.".to_string()),
        0xC => opcode = set_opcode(0x2C, 3, 4, "BIT (test BITs) Absolute".to_string()),
        0xD => opcode = set_opcode(0x2D, 3, 4, "AND Absolute".to_string()),
        0xE => opcode = set_opcode(0x2E, 3, 6, "ROL (ROtate Left) Absolute".to_string()),
        0xF => opcode = set_opcode(0x2F, 0, 0, "Not used.".to_string()),
        _   => println!("Not used."),
    }
    return opcode;
}

fn left_nib_is_3(rn: u8) -> Opcode
{
    let mut opcode = set_opcode(0x00, 0, 0, " ".to_string());
    match rn
    {
        0x0 => opcode = set_opcode(0x30, 2, 2, "BMI (Branch on MInus)".to_string()),
        // add 1 cycle if page boundary crossed
        0x1 => opcode = set_opcode(0x31, 2, 5, "AND Indirect, Y".to_string()),
        0x2 => opcode = set_opcode(0x32, 0, 0, "Not used.".to_string()),
        0x3 => opcode = set_opcode(0x33, 0, 0, "Not used.".to_string()),
        0x4 => opcode = set_opcode(0x34, 0, 0, "Not used.".to_string()),
        0x5 => opcode = set_opcode(0x35, 2, 3, "AND Zero Page, X".to_string()),
        0x6 => opcode = set_opcode(0x36, 2, 6, "ROL (ROtate Left) Zero Page, X".to_string()),
        0x7 => opcode = set_opcode(0x37, 0, 0, "Not used.".to_string()),
        0x8 => opcode = set_opcode(0x38, 1, 2, "SEC (SEt Carry)".to_string()),
        // add 1 cycle if page boundary is crossed
        0x9 => opcode = set_opcode(0x39, 3, 4, "AND Absolute, Y".to_string()),
        0xA => opcode = set_opcode(0x3A, 0, 0, "Not used.".to_string()),
        0xB => opcode = set_opcode(0x3B, 0, 0, "Not used.".to_string()),
        0xC => opcode = set_opcode(0x3C, 0, 0, "Not used.".to_string()),
        // add 1 cycle if page boundary is crossed
        0xD => opcode = set_opcode(0x3D, 3, 4, "AND Absolute, X".to_string()),
        0xE => opcode = set_opcode(0x3E, 3, 7, "ROL (ROtate Left) Absolute, X".to_string()),
        0xF => opcode = set_opcode(0x3F, 0, 0, "Not used.".to_string()),
        _   => println!("Not used."),
    }
    return opcode;
}

fn left_nib_is_4(rn: u8) -> Opcode
{
    let mut opcode = set_opcode(0x00, 0, 0, " ".to_string());
    match rn
    {
        0x0 => opcode = set_opcode(0x40, 1, 6, "RTI (ReTurn from Interrupt)".to_string()),
        0x1 => opcode = set_opcode(0x41, 2, 6, "EOR Indirect, X".to_string()),
        0x2 => opcode = set_opcode(0x42, 0, 0, "Not used.".to_string()),
        0x3 => opcode = set_opcode(0x43, 0, 0, "Not used.".to_string()),
        0x4 => opcode = set_opcode(0x44, 0, 0, "Not used.".to_string()),
        0x5 => opcode = set_opcode(0x45, 2, 3, "EOR Zero Page".to_string()),
        0x6 => opcode = set_opcode(0x46, 2, 5, "LSR (Logical Shift Right) Zero Page".to_string()),
        0x7 => opcode = set_opcode(0x47, 0, 0, "Not used.".to_string()),
        0x8 => opcode = set_opcode(0x48, 1, 3, "PHA (PusH Accumulator)".to_string()),
        0x9 => opcode = set_opcode(0x49, 2, 2, "EOR Immediate".to_string()),
        0xA => opcode = set_opcode(0x4A, 1, 2, "LSR (Logical Shift Right) Accumulator".to_string()),
        0xB => opcode = set_opcode(0x4B, 0, 0, "Not used.".to_string()),
        0xC => opcode = set_opcode(0x4C, 3, 3, "JMP Absolute".to_string()),
        0xD => opcode = set_opcode(0x4D, 3, 4, "EOR Absolute".to_string()),
        0xE => opcode = set_opcode(0x4E, 3, 6, "LSR (Logical Shift Right) Absolute".to_string()),
        0xF => opcode = set_opcode(0x4F, 0, 0, "Not used.".to_string()),
        _   => println!("Not used."),
    }
    return opcode;
}

fn left_nib_is_5(rn: u8) -> Opcode
{
    let mut opcode = set_opcode(0x00, 0, 0, " ".to_string());
    match rn
    {
        0x0 => opcode = set_opcode(0x50, 2, 2, "BVC (Branch on oVerflow Clear)".to_string()),
        // add 1 cycle if page boundary crossed
        0x1 => opcode = set_opcode(0x51, 2, 5, "EOR Indirect, Y".to_string()),
        0x2 => opcode = set_opcode(0x52, 0, 0, "Not used.".to_string()),
        0x3 => opcode = set_opcode(0x53, 0, 0, "Not used.".to_string()),
        0x4 => opcode = set_opcode(0x54, 0, 0, "Not used.".to_string()),
        0x5 => opcode = set_opcode(0x55, 2, 4, "EOR Zero Page, X".to_string()),
        0x6 => opcode = set_opcode(0x56, 2, 6, "LSR (Logical Shift Right) Zero Page, X".to_string()),
        0x7 => opcode = set_opcode(0x57, 0, 0, "Not used.".to_string()),
        0x8 => opcode = set_opcode(0x58, 1, 2, "CLI (CLear Interrupt)".to_string()),
        // add 1 cycle if page boundary is crossed
        0x9 => opcode = set_opcode(0x59, 3, 4, "EOR Absolute, Y".to_string()),
        0xA => opcode = set_opcode(0x5A, 0, 0, "Not used.".to_string()),
        0xB => opcode = set_opcode(0x5B, 0, 0, "Not used.".to_string()),
        0xC => opcode = set_opcode(0x5C, 0, 0, "Not used.".to_string()),
        // add 1 cycle if page boundary is crossed
        0xD => opcode = set_opcode(0x5D, 3, 4, "EOR Absolute, X".to_string()),
        0xE => opcode = set_opcode(0x5E, 3, 7, "LSR (Logical Shift Right) Absolute, X".to_string()),
        0xF => opcode = set_opcode(0x5F, 0, 0, "Not used.".to_string()),
        _   => println!("Not used."),
    }
    return opcode;
}

fn left_nib_is_6(rn: u8) -> Opcode
{
    let mut opcode = set_opcode(0x00, 0, 0, " ".to_string());
    match rn
    {
        0x0 => opcode = set_opcode(0x60, 1, 6, "RTS (ReTurn from Subroutine)".to_string()),
        0x1 => opcode = set_opcode(0x61, 2, 6, "ADC Indirect, X".to_string()),
        0x2 => opcode = set_opcode(0x62, 0, 0, "Not used.".to_string()),
        0x3 => opcode = set_opcode(0x63, 0, 0, "Not used.".to_string()),
        0x4 => opcode = set_opcode(0x64, 0, 0, "Not used.".to_string()),
        0x5 => opcode = set_opcode(0x65, 2, 3, "ADC Zero Page".to_string()),
        0x6 => opcode = set_opcode(0x66, 2, 5, "ROR (ROtate Right) Zero Page".to_string()),
        0x7 => opcode = set_opcode(0x67, 0, 0, "Not used.".to_string()),
        0x8 => opcode = set_opcode(0x68, 1, 4, "PLA (PuLl Accumulator)".to_string()),
        0x9 => opcode = set_opcode(0x69, 2, 2, "ADC Immediate".to_string()),
        0xA => opcode = set_opcode(0x6A, 1, 2, "ROR (ROtate Right)".to_string()),
        0xB => opcode = set_opcode(0x6B, 0, 0, "Not used.".to_string()),
        0xC => opcode = set_opcode(0x6C, 3, 5, "JMP Indirect".to_string()),
        0xD => opcode = set_opcode(0x6D, 3, 4, "ADC Absolute".to_string()),
        0xE => opcode = set_opcode(0x6E, 3, 6, "ROR (ROtate Right) Absolute".to_string()),
        0xF => opcode = set_opcode(0x6F, 0, 0, "Not used.".to_string()),
        _   => println!("Not used."),
    }
    return opcode;
}

fn left_nib_is_7(rn: u8) -> Opcode
{
    let mut opcode = set_opcode(0x00, 0, 0, " ".to_string());
    match rn
    {
        0x0 => opcode = set_opcode(0x70, 2, 2, "BVS (Branch on oVerflow Set)".to_string()),
        // add 1 cycle if page boundary crossed
        0x1 => opcode = set_opcode(0x71, 2, 5, "ADC Indirect, Y".to_string()),
        0x2 => opcode = set_opcode(0x72, 0, 0, "Not used.".to_string()),
        0x3 => opcode = set_opcode(0x73, 0, 0, "Not used.".to_string()),
        0x4 => opcode = set_opcode(0x74, 0, 0, "Not used.".to_string()),
        0x5 => opcode = set_opcode(0x75, 2, 4, "ADC Zero Page, X".to_string()),
        0x6 => opcode = set_opcode(0x76, 2, 6, "ROR (ROtate Right) Zero Page, X".to_string()),
        0x7 => opcode = set_opcode(0x77, 0, 0, "Not used.".to_string()),
        0x8 => opcode = set_opcode(0x78, 1, 2, "SEI (SEt Interrupt)".to_string()),
        // add 1 cycle if page boundary crossed
        0x9 => opcode = set_opcode(0x79, 3, 4, "ADC Absolute, Y".to_string()),
        0xA => opcode = set_opcode(0x7A, 0, 0, "Not used.".to_string()),
        0xB => opcode = set_opcode(0x7B, 0, 0, "Not used.".to_string()),
        0xC => opcode = set_opcode(0x7C, 0, 0, "Not used.".to_string()),
        // add 1 cycle if page boundary crossed
        0xD => opcode = set_opcode(0x7D, 3, 4, "ADC Absolute, X".to_string()),
        0xE => opcode = set_opcode(0x7E, 3, 7, "ROR (ROtate Right) Absolute, X".to_string()),
        0xF => opcode = set_opcode(0x7F, 0, 0, "Not used.".to_string()),
        _   => println!("Not used."),
    }
    return opcode;
}

fn left_nib_is_8(rn: u8) -> Opcode
{
    let mut opcode = set_opcode(0x00, 0, 0, " ".to_string());
    match rn
    {
        0x0 => opcode = set_opcode(0x80, 0, 0, "Not used.".to_string()),
        0x1 => opcode = set_opcode(0x81, 2, 6, "STA Indirect, X".to_string()),
        0x2 => opcode = set_opcode(0x82, 0, 0, "Not used.".to_string()),
        0x3 => opcode = set_opcode(0x83, 0, 0, "Not used.".to_string()),
        0x4 => opcode = set_opcode(0x84, 2, 3, "STY Zero Page".to_string()),
        0x5 => opcode = set_opcode(0x85, 2, 3, "STA Zero Page".to_string()),
        0x6 => opcode = set_opcode(0x86, 2, 3, "STX Zero Page".to_string()),
        0x7 => opcode = set_opcode(0x87, 0, 0, "Not used.".to_string()),
        0x8 => opcode = set_opcode(0x88, 1, 2, "DEY (DEcrement Y)".to_string()),
        0x9 => opcode = set_opcode(0x89, 0, 0, "Not used.".to_string()),
        0xA => opcode = set_opcode(0x8A, 1, 2, "TXA (TRansfer X to A)".to_string()),
        0xB => opcode = set_opcode(0x8B, 0, 0, "Not used.".to_string()),
        0xC => opcode = set_opcode(0x8C, 3, 4, "STY Absolute".to_string()),
        0xD => opcode = set_opcode(0x8D, 3, 4, "STA Absolute".to_string()),
        0xE => opcode = set_opcode(0x8E, 3, 4, "STX Absolute".to_string()),
        0xF => opcode = set_opcode(0x8F, 0, 0, "Not used.".to_string()),
        _   => println!("Not used."),
    }
    return opcode;
}

fn left_nib_is_9(rn: u8) -> Opcode
{
    let mut opcode = set_opcode(0x00, 0, 0, " ".to_string());
    match rn
    {
        0x0 => opcode = set_opcode(0x90, 2, 2, "BCC (Branch on Carry Clear)".to_string()),
        0x1 => opcode = set_opcode(0x91, 2, 6, "STA Indirect, Y".to_string()),
        0x2 => opcode = set_opcode(0x92, 0, 0, "Not used.".to_string()),
        0x3 => opcode = set_opcode(0x93, 0, 0, "Not used.".to_string()),
        0x4 => opcode = set_opcode(0x94, 2, 4, "STY Zero Page, X".to_string()),
        0x5 => opcode = set_opcode(0x95, 2, 4, "STA Zero Page, X".to_string()),
        0x6 => opcode = set_opcode(0x96, 2, 4, "STX Zero Page, Y".to_string()),
        0x7 => opcode = set_opcode(0x97, 0, 0, "Not used.".to_string()),
        0x8 => opcode = set_opcode(0x98, 1, 2, "TYA (Transfer Y to A)".to_string()),
        0x9 => opcode = set_opcode(0x99, 3, 5, "STA Absolute, Y".to_string()),
        0xA => opcode = set_opcode(0x9A, 1, 2, "TXS (Transfer X to Stack ptr)".to_string()),
        0xB => opcode = set_opcode(0x9B, 0, 0, "Not used.".to_string()),
        0xC => opcode = set_opcode(0x9C, 0, 0, "Not used.".to_string()),
        0xD => opcode = set_opcode(0x9D, 3, 5, "STA Absolute, X".to_string()),
        0xE => opcode = set_opcode(0x9E, 0, 0, "Not used.".to_string()),
        0xF => opcode = set_opcode(0x9F, 0, 0, "Not used.".to_string()),
        _   => println!("Not used."),
    }
    return opcode;
}

fn left_nib_is_a(rn: u8) -> Opcode
{
    let mut opcode = set_opcode(0x00, 0, 0, " ".to_string());
    match rn
    {
        0x0 => opcode = set_opcode(0xA0, 2, 2, "LDY Immediate".to_string()),
        0x1 => opcode = set_opcode(0xA1, 2, 6, "LDA Indirect, X".to_string()),
        0x2 => opcode = set_opcode(0xA2, 2, 2, "LDX Immediate".to_string()),
        0x3 => opcode = set_opcode(0xA3, 0, 0, "Not used.".to_string()),
        0x4 => opcode = set_opcode(0xA4, 2, 3, "LDY Zero Page".to_string()),
        0x5 => opcode = set_opcode(0xA5, 2, 3, "LDA Zero Page".to_string()),
        0x6 => opcode = set_opcode(0xA6, 2, 3, "LDX Zero Page".to_string()),
        0x7 => opcode = set_opcode(0xA7, 0, 0, "Not used.".to_string()),
        0x8 => opcode = set_opcode(0xA8, 1, 2, "TAY (Transfer A to Y)".to_string()),
        0x9 => opcode = set_opcode(0xA9, 2, 2, "LDA Immediate".to_string()),
        0xA => opcode = set_opcode(0xAA, 1, 2, "TAX (Transfer A to X)".to_string()),
        0xB => opcode = set_opcode(0xAB, 0, 0, "Not used.".to_string()),
        0xC => opcode = set_opcode(0xAC, 3, 4, "LDY Absolute".to_string()),
        0xD => opcode = set_opcode(0xAD, 3, 4, "LDA Absolute".to_string()),
        0xE => opcode = set_opcode(0xAE, 3, 4, "LDX Absolute".to_string()),
        0xF => opcode = set_opcode(0xAF, 0, 0, "Not used.".to_string()),
        _   => println!("Not used."),
    }
    return opcode;
}

fn left_nib_is_b(rn: u8) -> Opcode
{
    let mut opcode = set_opcode(0x00, 0, 0, " ".to_string());
    match rn
    {
        0x0 => opcode = set_opcode(0xB0, 2, 2, "BCS (Branch on Carry Set)".to_string()),
        // add 1 cycle if page boundary crossed
        0x1 => opcode = set_opcode(0xB1, 2, 5, "LDA Indirect, Y".to_string()),
        0x2 => opcode = set_opcode(0xB2, 0, 0, "Not used.".to_string()),
        0x3 => opcode = set_opcode(0xB3, 0, 0, "Not used.".to_string()),
        0x4 => opcode = set_opcode(0xB4, 2, 4, "LDY Zero Page, X".to_string()),
        0x5 => opcode = set_opcode(0xB5, 2, 4, "LDA Zero Page, X".to_string()),
        0x6 => opcode = set_opcode(0xB6, 2, 4, "LDX Zero Page, Y".to_string()),
        0x7 => opcode = set_opcode(0xB7, 0, 0, "Not used.".to_string()),
        0x8 => opcode = set_opcode(0xB8, 1, 2, "CLV (CLear oVerflow)".to_string()),
        // add 1 cycle if page boundary crossed
        0x9 => opcode = set_opcode(0xB9, 3, 4, "LDA Absolute, Y".to_string()),
        0xA => opcode = set_opcode(0xBA, 1, 2, "TSX (Transfer Stack ptr to X)".to_string()),
        0xB => opcode = set_opcode(0xBB, 0, 0, "Not used.".to_string()),
        // add 1 cycle if page boundary crossed
        0xC => opcode = set_opcode(0xBC, 3, 4, "LDY Absolute, X".to_string()),
        // add 1 cycle if page boundary crossed
        0xD => opcode = set_opcode(0xBD, 3, 4, "LDA Absolute, X".to_string()),
        // add 1 cycle if page boundary crossed
        0xE => opcode = set_opcode(0xBE, 3, 4, "LDX Absolute, Y".to_string()),
        0xF => opcode = set_opcode(0xBF, 0, 0, "Not used.".to_string()),
        _   => println!("Not used."),
    }
    return opcode;
}

fn left_nib_is_c(rn: u8) -> Opcode
{
    let mut opcode = set_opcode(0x00, 0, 0, " ".to_string());
    match rn
    {
        0x0 => opcode = set_opcode(0xC0, 2, 2, "CPY (ComPare Y register) Immediate".to_string()),
        0x1 => opcode = set_opcode(0xC1, 2, 6, "CMP (CoMPare accumulator) Indirect, X".to_string()),
        0x2 => opcode = set_opcode(0xC2, 0, 0, "Not used.".to_string()),
        0x3 => opcode = set_opcode(0xC3, 0, 0, "Not used.".to_string()),
        0x4 => opcode = set_opcode(0xC4, 2, 3, "CPY (ComPare Y register) Zero Page".to_string()),
        0x5 => opcode = set_opcode(0xC5, 2, 3, "CMP (CoMPare accumulator) Zero Page".to_string()),
        0x6 => opcode = set_opcode(0xC6, 2, 5, "DEC (DECrement memory) Zero Page".to_string()),
        0x7 => opcode = set_opcode(0xC7, 0, 0, "Not used.".to_string()),
        0x8 => opcode = set_opcode(0xC8, 1, 2, "INY (INcrement Y)".to_string()),
        0x9 => opcode = set_opcode(0xC9, 2, 2, "CMP (CoMPare accumulator) Immediate".to_string()),
        0xA => opcode = set_opcode(0xCA, 1, 2, "DEX (DEcrement X)".to_string()),
        0xB => opcode = set_opcode(0xCB, 0, 0, "Not used.".to_string()),
        0xC => opcode = set_opcode(0xCC, 3, 4, "CPY (ComPare Y register) Absolute".to_string()),
        0xD => opcode = set_opcode(0xCD, 3, 4, "CMP (CoMPare accumulator) Absolute".to_string()),
        0xE => opcode = set_opcode(0xCE, 3, 6, "DEC (DECrement memory) Absolute".to_string()),
        0xF => opcode = set_opcode(0xCF, 0, 0, "Not used.".to_string()),
        _   => println!("Not used."),
    }
    return opcode;
}

fn left_nib_is_d(rn: u8) -> Opcode
{
    let mut opcode = set_opcode(0x00, 0, 0, " ".to_string());
    match rn
    {
        0x0 => opcode = set_opcode(0xD0, 2, 2, "BNE (Branch on Not Equal)".to_string()),
        // add 1 cycle if page boundary crossed
        0x1 => opcode = set_opcode(0xD1, 2, 5, "CMP (CoMPare accumulator) Indirect, Y".to_string()),
        0x2 => opcode = set_opcode(0xD2, 0, 0, "Not used.".to_string()),
        0x3 => opcode = set_opcode(0xD3, 0, 0, "Not used.".to_string()),
        0x4 => opcode = set_opcode(0xD4, 0, 0, "Not used.".to_string()),
        0x5 => opcode = set_opcode(0xD5, 2, 4, "CMP (CoMPare accumulator) Zero Page, X".to_string()),
        0x6 => opcode = set_opcode(0xD6, 2, 6, "DEC (DECrement memory) Zero Page, X".to_string()),
        0x7 => opcode = set_opcode(0xD7, 0, 0, "Not used.".to_string()),
        0x8 => opcode = set_opcode(0xD8, 1, 2, "CLD (CLear Decimal)".to_string()),
        // add 1 cycle if page boundary crossed
        0x9 => opcode = set_opcode(0xD9, 3, 4, "CMP (CoMPare accumulator) Absolute, Y".to_string()),
        0xA => opcode = set_opcode(0xDA, 0, 0, "Not used.".to_string()),
        0xB => opcode = set_opcode(0xDB, 0, 0, "Not used.".to_string()),
        0xC => opcode = set_opcode(0xDC, 0, 0, "Not used.".to_string()),
        // add 1 cycle if page boundary crossed
        0xD => opcode = set_opcode(0xDD, 3, 4, "CMP (CoMPare accumulator) Absolute, X".to_string()),
        0xE => opcode = set_opcode(0xDE, 3, 7, "DEC (DECrement memory) Absolute, X".to_string()),
        0xF => opcode = set_opcode(0xDF, 0, 0, "Not used.".to_string()),
        _   => println!("Not used."),
    }
    return opcode;
}

fn left_nib_is_e(rn: u8) -> Opcode
{
    let mut opcode = set_opcode(0x00, 0, 0, " ".to_string());
    match rn
    {
        0x0 => opcode = set_opcode(0xE0, 2, 2, "CPX (ComPare X register) Immediate".to_string()),
        0x1 => opcode = set_opcode(0xE1, 2, 6, "SBC (SuBtract with Carry) Indirect, X".to_string()),
        0x2 => opcode = set_opcode(0xE2, 0, 0, "Not used.".to_string()),
        0x3 => opcode = set_opcode(0xE3, 0, 0, "Not used.".to_string()),
        0x4 => opcode = set_opcode(0xE4, 2, 3, "CPX (ComPare X register) Zero Page".to_string()),
        0x5 => opcode = set_opcode(0xE5, 2, 3, "SBC (SuBtract with Carry) Zero Page".to_string()),
        0x6 => opcode = set_opcode(0xE6, 2, 5, "INC (INCrement memory) Zero Page".to_string()),
        0x7 => opcode = set_opcode(0xE7, 0, 0, "Not used.".to_string()),
        0x8 => opcode = set_opcode(0xE8, 1, 2, "INX (INcrement X)".to_string()),
        0x9 => opcode = set_opcode(0xE9, 2, 2, "SBC (SuBtract with Carry) Immediate".to_string()),
        0xA => opcode = set_opcode(0xEA, 1, 2, "NOP".to_string()),
        0xB => opcode = set_opcode(0xEB, 0, 0, "Not used.".to_string()),
        0xC => opcode = set_opcode(0xEC, 3, 4, "CPX (ComPare X register) Absolute".to_string()),
        0xD => opcode = set_opcode(0xED, 3, 4, "SBC (SuBtract with Carry) Absolute".to_string()),
        0xE => opcode = set_opcode(0xEE, 3, 6, "INC (INCrement memory) Absolute".to_string()),
        0xF => opcode = set_opcode(0xEF, 0, 0, "Not used.".to_string()),
        _   => println!("Not used."),
    }
    return opcode;
}

fn left_nib_is_f(rn: u8) -> Opcode
{
    let mut opcode = set_opcode(0x00, 0, 0, " ".to_string());
    match rn
    {
        0x0 => opcode = set_opcode(0xF0, 2, 2, "BEQ (Branch on EQual)".to_string()),
        // add 1 cycle if page boundary crossed
        0x1 => opcode = set_opcode(0xF1, 2, 5, "SBC (SuBtract with Carry) Indirect, Y".to_string()),
        0x2 => opcode = set_opcode(0xF2, 0, 0, "Not used.".to_string()),
        0x3 => opcode = set_opcode(0xF3, 0, 0, "Not used.".to_string()),
        0x4 => opcode = set_opcode(0xF4, 0, 0, "Not used.".to_string()),
        0x5 => opcode = set_opcode(0xF5, 2, 4, "SBC (SuBtract with Carry) Zero Page, X".to_string()),
        0x6 => opcode = set_opcode(0xF6, 2, 6, "INC (INCrement memory) Zero Page, X".to_string()),
        0x7 => opcode = set_opcode(0xF7, 0, 0, "Not used.".to_string()),
        0x8 => opcode = set_opcode(0xF8, 1, 2, "SED (SEt Decimal)".to_string()),
        // add 1 cycle if page boundary crossed
        0x9 => opcode = set_opcode(0xF9, 3, 4, "SBC (SuBtract with Carry) Absolute, Y".to_string()),
        0xA => opcode = set_opcode(0xFA, 0, 0, "Not used.".to_string()),
        0xB => opcode = set_opcode(0xFB, 0, 0, "Not used.".to_string()),
        0xC => opcode = set_opcode(0xFC, 0, 0, "Not used.".to_string()),
        // add 1 cycle if page boundary crossed
        0xD => opcode = set_opcode(0xFD, 3, 4, "SBC (SuBtract with Carry) Absolute, X".to_string()),
        0xE => opcode = set_opcode(0xFE, 3, 7, "INC (INCrement memory) Absolute, X".to_string()),
        0xF => opcode = set_opcode(0xFF, 0, 0, "Not used.".to_string()),
        _   => println!("Not used."),
    }
    return opcode;
}
