mod file_handling;
mod opcode;
mod header;
mod cpu;

use std::time::Duration;
use std::thread;

fn main()
{
    let mut op_desc: String;
    let mut op_code: u8;

    let mut cpu = cpu::init_cpu();

    let path = String::from("SMB.nes");
    let f = file_handling::open_file(path);

    let buffer = file_handling::store_file(f);
    let mut list_of_opcodes = Vec::new();
    let mut cur_opcode: opcode::Opcode;


    for element in buffer.iter()
    {
        list_of_opcodes.push(opcode::build_opcode(*element));
    }

    let h = header::set_header(list_of_opcodes.clone());
    let mut i = 0x10;
    let mut n = 0x0;
    while i <= 0x10 + 0x20
    {
        if n == 0x0A
        {
            break;
        }
        cur_opcode = list_of_opcodes[i].clone();
        op_desc = opcode::get_opcode_description(list_of_opcodes[i].clone());
        op_code = opcode::get_opcode_code(list_of_opcodes[i].clone());
        cpu.set_instruction(opcode::get_opcode_code(list_of_opcodes[i].clone()));
        cpu.set_first_byte_of_interest(opcode::get_opcode_code(list_of_opcodes[i+1].clone()));
        cpu.set_second_byte_of_interest(opcode::get_opcode_code(list_of_opcodes[i+2].clone()));

        println!("{:#04X}: {:#04X} | {}", cpu.get_pc(), op_code, op_desc);
        print!("\n");
        cpu.execute_opcode(cur_opcode);
        println!("Interrupt Flag: {:#04X}", cpu.get_interrupt_flag());
        println!("Carry Flag: {:#04X}", cpu.get_carry_flag());
        println!("Zero Flag: {:#04X}", cpu.get_zero_flag());
        println!("Decimal Flag: {:#04X}", cpu.get_decimal_flag());
        println!("Overflow Flag: {:#04X}", cpu.get_overflow_flag());
        println!("Negative Flag: {:#04X}", cpu.get_negative_flag());
        println!("A Register: {:#04X}", cpu.get_a());
        println!("X Register: {:#04X}", cpu.get_x());
        println!("Y Register: {:#04X}", cpu.get_y());
        println!("S Register: {:#04X}", cpu.get_s());
        println!("_____________________________________\n");
        i = cpu.get_pc() as usize;
        n += 1;
        thread::sleep(Duration::from_millis(1000));
    }

}
