use opcode;
use opcode::Opcode;

#[derive(Debug, Clone)]
pub struct Header
{
    byte_0: u8, // 0x0
    byte_1: u8, // 0x1
    byte_2: u8, // 0x2
    byte_3: u8, // 0x3
    prg_rom_size: u8, // 0x4
    chr_rom_size: u8, // 0x5
    flags_6: u8, // 0x6
    flags_7: u8, // 0x7
    flags_8: u8, // 0x8
    flags_9: u8, // 0x9
    flags_10: u8, // 0xA
}

pub fn set_header(rom: Vec<Opcode>) -> Header
{
    let h = Header
    {
        byte_0: opcode::get_opcode_code(rom[0x0].clone()),
        byte_1: opcode::get_opcode_code(rom[0x1].clone()),
        byte_2: opcode::get_opcode_code(rom[0x2].clone()),
        byte_3: opcode::get_opcode_code(rom[0x3].clone()),
        prg_rom_size: opcode::get_opcode_code(rom[0x4].clone()),
        chr_rom_size: opcode::get_opcode_code(rom[0x5].clone()),
        flags_6: opcode::get_opcode_code(rom[0x6].clone()),
        flags_7: opcode::get_opcode_code(rom[0x7].clone()),
        flags_8: opcode::get_opcode_code(rom[0x8].clone()),
        flags_9: opcode::get_opcode_code(rom[0x9].clone()),
        flags_10: opcode::get_opcode_code(rom[0xA].clone()),
    };
    return h;
}

pub fn print_header(h: Header)
{
    println!("Bytes 0-3:    {}{}{}{}",h.byte_0 as char, h.byte_1 as char,
                                      h.byte_2 as char, h.byte_3 as char);
    println!("PRG_ROM_SIZE: {:#04X}", h.prg_rom_size);
    println!("CHR_ROM_SIZE: {:#04X}", h.chr_rom_size);
    println!("flags_6:      {:#04X}", h.flags_6);
    println!("flags_7:      {:#04X}", h.flags_7);
    println!("flags_8:      {:#04X}", h.flags_8);
    println!("flags_9:      {:#04X}", h.flags_9);
    println!("flags_10:     {:#04X}", h.flags_10);
}
