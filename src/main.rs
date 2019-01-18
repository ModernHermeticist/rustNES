mod file_handling;
mod opcode;
mod header;
mod cpu;
mod instructions;

fn main()
{
    let mut pc = 0x10;
    let mut step = 0x2;

    let mut op_desc = String::from("");
    let mut op_code: u8 = 0x0;

    let mut cpu = cpu::init_cpu();

    let path = String::from("SMB.nes");
    let f = file_handling::open_file(path);

    let buffer = file_handling::store_file(f);
    let mut list_of_opcodes = Vec::new();


    for element in buffer.iter()
    {
        list_of_opcodes.push(opcode::build_opcode(*element));
    }

    let h = header::set_header(list_of_opcodes.clone());

    list_of_opcodes.drain(0..15);

    for element in list_of_opcodes.iter().step_by(step)
    {
        op_desc = opcode::get_opcode_description(element.clone());
        op_code = opcode::get_opcode_code(element.clone());
        step = opcode::get_opcode_length(element.clone());
        println!("{:#04X}: {:#04X} | {}", pc, op_code, op_desc);
        pc += step;
    }

    header::print_header(h);

    println!("{}", cpu.get_pc());
    cpu.increment_pc(2);
    println!("{}", cpu.get_pc());
    cpu = instructions::brk(cpu);
    println!("{}", cpu.get_pc());
}
