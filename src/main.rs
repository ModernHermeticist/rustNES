mod file_handling;
mod opcode;
mod header;
mod cpu;

fn main()
{
    let mut op_desc = String::from("");
    let mut op_code: u8 = 0x0;

    let mut cpu = cpu::init_cpu();

    let path = String::from("SMB.nes");
    let f = file_handling::open_file(path);

    let buffer = file_handling::store_file(f);
    let mut list_of_opcodes = Vec::new();
    let mut cur_opcode = opcode::set_opcode(0x00, 0, 0, " ".to_string());


    for element in buffer.iter()
    {
        list_of_opcodes.push(opcode::build_opcode(*element));
    }

    let h = header::set_header(list_of_opcodes.clone());


    for mut i in 0x10..(list_of_opcodes.len() - 0x02)
    {
        if i >= 0x05 + 0x0F
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
        op_code = cpu.get_instruction();
        println!("{:#04X}", op_code);
        op_code = cpu.get_first_byte_of_interest();
        println!("{:#04X}", op_code);
        op_code = cpu.get_second_byte_of_interest();
        println!("{:#04X}", op_code);
        println!("Interrupt Flag: {:#04X}", cpu.get_interrupt_flag());
        cpu.execute_opcode(cur_opcode);
        println!("Interrupt Flag: {:#04X}", cpu.get_interrupt_flag());
        i = cpu.get_pc() as usize;
    }

}
