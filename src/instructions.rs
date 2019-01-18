use cpu::CPU;


pub fn brk(mut cpu: CPU) -> CPU
{
    cpu.increment_pc(2);

    return cpu;
}
